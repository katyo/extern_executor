import 'dart:async';
import 'dart:ffi';
import 'dart:collection';

class Task<T> {
  final Pointer<Void> id;
  final Future<T> future;

  Task(this.id, this.future) {}
}

class _TaskData<T, X> {
  final Completer<T> completer;
  final X? context;

  _TaskData(this.completer, [this.context]) {}
}

class Dispatcher {
  int _lastId = 0;
  HashMap<int, _TaskData<dynamic, dynamic>> _pending = HashMap();

  Task<T> initiate<T, X>(X context) {
    final completer = Completer<T>();

    _lastId += 1;
    _pending[_lastId] = _TaskData<T, X>(completer, context);

    return Task(Pointer.fromAddress(_lastId), completer.future);
  }

  void complete<T, X>(Pointer<Void> id, T Function(X context) func) {
    final _TaskData<T, X> task = _pending.remove(id.address) as _TaskData<T, X>;
    T val;

    try {
      val = func(task.context!);
    } catch (err) {
      task.completer.completeError(err);
      return;
    }

    task.completer.complete(val);
  }
}

final asyncDispatcher = Dispatcher();
