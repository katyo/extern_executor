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

  List<Future> tasks = [];

  for (var secs in [2.5]) {
    print("async delay(${secs}) start");

    tasks.add(exampleLib.delay(2.5).then((res) {
          print("async delay(${secs}) end");
    }));
  }

  for (var path in ['main.dart', 'other.dart']) {
    print("async read_file('${path}') start");

    tasks.add(exampleLib.readFile(path).then(
        (data) {
          print("async read_file('${path}') ok: read ${data.length} chars");
        },
        onError: (error) {
          print("async read_file('${path}') error: ${error}");
        }
    ));
  }

  for (var name in ['illumium.org', 'nihil.illumium.org']) {
    print("async ns_lookup('${name}') start");

    tasks.add(exampleLib.nsLookup(name).then(
        (addr) {
          print("async ns_lookup('${name}') ok: ${addr}");
        },
        onError: (error) {
          print("async ns_lookup('${name}') error: ${error}");
        }
    ));
  }

  await Future.wait(tasks);

  executor.stop();
}

String dylibPath(String name, {String path = ''}) {
  if (path.length > 0 && !path.endsWith('/')) {
    path += '/';
  }
  if (Platform.isLinux || Platform.isAndroid)
  return '${path}lib${name}.so';
  if (Platform.isMacOS) return '${path}lib${name}.dylib';
  if (Platform.isWindows) return '${path}${name}.dll';
  throw Exception("Platform not implemented");
}
