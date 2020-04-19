import 'dart:async';
import 'dart:ffi';
import 'package:ffi/ffi.dart';
import './async.dart';

typedef _rustDelayCb = Void Function(Pointer<Void> userdata);
typedef _rustDelay = Void Function(Float duration, Pointer<NativeFunction<_rustDelayCb>> callback, Pointer<Void> userdata);
typedef _RustDelay = void Function(double duration, Pointer<NativeFunction<_rustDelayCb>> callback, Pointer<Void> userdata);

typedef _rustReadFileCb = Void Function(Pointer<Utf8> data, Pointer<Void> userdata);
typedef _rustReadFile = Void Function(Pointer<Utf8> path, Pointer<NativeFunction<_rustReadFileCb>> callback, Pointer<Void> userdata);
typedef _RustReadFile = void Function(Pointer<Utf8> path, Pointer<NativeFunction<_rustReadFileCb>> callback, Pointer<Void> userdata);

class ExampleLib {
  final Pointer<NativeFunction<_rustDelayCb>> _delayCb;
  final _RustDelay _delay;
  final Pointer<NativeFunction<_rustReadFileCb>> _readFileCb;
  final _RustReadFile _readFile;

  ExampleLib(DynamicLibrary dylib)
  : _delayCb = Pointer.fromFunction(_delayCb_)
  , _delay = dylib.lookup<NativeFunction<_rustDelay>>('delay').asFunction()
  , _readFileCb = Pointer.fromFunction(_readFileCb_)
  , _readFile = dylib.lookup<NativeFunction<_rustReadFile>>('read_file').asFunction()
  {}

  static void _delayCb_(Pointer<Void> taskId) {
    asyncDispatcher.complete(taskId, null);
  }

  Future<void> delay(double duration) {
    final task = asyncDispatcher.create();

    _delay(duration, _delayCb, task.id);

    return task.future;
  }

  static void _readFileCb_(Pointer<Utf8> rawData, Pointer<Void> taskId) {
    final data = Utf8.fromUtf8(rawData);

    free(rawData);

    asyncDispatcher.complete(taskId, data);
  }

  Future<String> readFile(String path) async {
    final task = asyncDispatcher.create();

    final rawPath = Utf8.toUtf8(path);

    _readFile(rawPath, _readFileCb, task.id);

    final data = await task.future;

    free(rawPath);

    return data;
  }
}
