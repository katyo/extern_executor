import 'dart:io' show Platform;
import 'dart:ffi';
import 'dart:isolate';
import 'package:ffi/ffi.dart';
import 'ffi/executor.dart';
import 'ffi/example_lib.dart';

main() async {
  final dylib = DynamicLibrary.open(dylibPath("example_lib"));
  final executor = Executor(dylib);
  final exampleLib = ExampleLib(dylib);

  executor.start();

  print("async delay() start");

  final delayPend = exampleLib.delay(2.5).then((res) {
      print("async delay() end");
  });

  print("async read_file() start");

  final readFilePend = exampleLib.readFile("main.dart").then((data) {
      print("async read_file() end. Read ${data.length} chars");
  });

  await Future.wait([delayPend, readFilePend]);

  executor.stop();
}

String dylibPath(String name, {String path}) {
  if (path == null) path = '';
  if (Platform.isLinux || Platform.isAndroid)
  return '${path}lib${name}.so';
  if (Platform.isMacOS) return '${path}lib${name}.dylib';
  if (Platform.isWindows) return '${path}${name}.dll';
  throw Exception("Platform not implemented");
}
