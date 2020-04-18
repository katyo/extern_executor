import 'dart:io' show Platform;
import 'dart:ffi';
import 'dart:isolate';
import 'package:ffi/ffi.dart';

class RustTask extends Struct {}

final dylib = DynamicLibrary.open(dylibPath("example_lib"));

final rustLoopInit = dylib.lookupFunction<
Void Function(Int64 wakePort, Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>> taskPost),
void Function(int wakePort, Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>> taskPost)
>('rust_async_executor_dart_init');

final rustTaskPoll = dylib.lookupFunction<
Int8 Function(Pointer<RustTask> task),
int Function(Pointer<RustTask> task)
>('rust_async_executor_dart_poll');

final rustTaskDrop = dylib.lookupFunction<
Void Function(Pointer<RustTask> task),
void Function(Pointer<RustTask> task)
>('rust_async_executor_dart_drop');

final rustDelay = dylib.lookupFunction<
Void Function(Float duration, Pointer<NativeFunction<Void Function()>> callback),
void Function(double duration, Pointer<NativeFunction<Void Function()>> callback)
>('delay');

final rustReadFile = dylib.lookupFunction<
Void Function(Pointer<Utf8> path, Pointer<NativeFunction<Void Function(Pointer<Utf8>)>> callback),
void Function(Pointer<Utf8> path, Pointer<NativeFunction<Void Function(Pointer<Utf8>)>> callback)
>('read_file');

main() {
  final wakePort = ReceivePort()..listen(pollTask);

  rustLoopInit(wakePort.sendPort.nativePort, NativeApi.postCObject);

  print("async delay() start");

  final rustDelayCb = Pointer.fromFunction<Void Function()>(delayCb);
  rustDelay(2.5, rustDelayCb);

  print("async read_file() start");

  final rustReadFileCb = Pointer.fromFunction<Void Function(Pointer<Utf8>)>(readFileCb);
  final path = Utf8.toUtf8("main.dart");
  rustReadFile(path, rustReadFileCb);
  //free(path);
}

void delayCb() {
  print("async delay() end");
}

void readFileCb(Pointer<Utf8> data) {
  final str = Utf8.fromUtf8(data);
  print("async read_file() end. Read ${str.length} chars");
  free(data);
}

void pollTask(dynamic message) {
  final int taskAddr = message;
  final task = Pointer<RustTask>.fromAddress(taskAddr);

  if (rustTaskPoll(task) != 0) {
    print("task_poll() = true");
  } else {
    print("task_poll() = false");
    rustTaskDrop(task);
  }
}

String dylibPath(String name, {String path}) {
  if (path == null) path = '';
  if (Platform.isLinux || Platform.isAndroid)
  return '${path}lib${name}.so';
  if (Platform.isMacOS) return '${path}lib${name}.dylib';
  if (Platform.isWindows) return '${path}${name}.dll';
  throw Exception("Platform not implemented");
}
