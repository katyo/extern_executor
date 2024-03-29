import 'dart:ffi';
import 'dart:isolate';

final class _RustTask extends Opaque {}

typedef _rustTaskPoll = Int8 Function(Pointer<_RustTask> task);
typedef _RustTaskPoll = int Function(Pointer<_RustTask> task);

typedef _rustTaskDrop = Void Function(Pointer<_RustTask> task);
typedef _RustTaskDrop = void Function(Pointer<_RustTask> task);

typedef _postCObject = Int8 Function(Int64, Pointer<Dart_CObject>);

typedef _rustLoopInit = Void Function(Int64 wakePort, Pointer<NativeFunction<_postCObject>> taskPost);
typedef _RustLoopInit = void Function(int wakePort, Pointer<NativeFunction<_postCObject>> taskPost);

class Executor {
  final _RustTaskPoll _taskPoll;
  final _RustTaskDrop _taskDrop;
  final _RustLoopInit _loopInit;
  ReceivePort? _wakePort;

  Executor(DynamicLibrary dylib)
  : _taskPoll = dylib.lookup<NativeFunction<_rustTaskPoll>>('rust_async_executor_dart_poll').asFunction()
  , _taskDrop = dylib.lookup<NativeFunction<_rustTaskDrop>>('rust_async_executor_dart_drop').asFunction()
  , _loopInit = dylib.lookup<NativeFunction<_rustLoopInit>>('rust_async_executor_dart_init').asFunction()
  , _wakePort = null
  {}

  bool get started => _wakePort != null;
  bool get stopped => !started;

  void start() {
    if (_wakePort != null) {
      print("already started");
      return;
    }
    _wakePort = ReceivePort()..listen(_pollTask);
    _loopInit(_wakePort!.sendPort.nativePort, NativeApi.postCObject);
  }

  void stop() {
    if (_wakePort == null) {
      print("already stopped");
      return;
    }
    _wakePort!.close();
    _wakePort = null;
  }

  void _pollTask(dynamic message) {
    final int taskAddr = message;
    final task = Pointer<_RustTask>.fromAddress(taskAddr);

    if (_taskPoll(task) != 0) {
      print("task_poll() = true");
    } else {
      print("task_poll() = false");
      _taskDrop(task);
    }
  }
}
