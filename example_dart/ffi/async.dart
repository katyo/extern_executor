import 'dart:async';
import 'dart:ffi';
import 'dart:collection';

class Wrapper<T> {
  final Pointer<Void> id;
  final Future<T> future;

  Wrapper(this.id, this.future) {}
}

class Dispatcher {
  int _lastId = 0;
  HashMap<int, Completer> _pending = HashMap();

  Wrapper<T> create<T>() {
    final completer = Completer();

    _lastId += 1;
    _pending[_lastId] = completer;

    return Wrapper(Pointer.fromAddress(_lastId), completer.future);
  }

  void notify<T>(Pointer<Void> id, T value) {
    final completer = _pending[id.address];

    completer.complete(value);
  }

  void complete<T>(Pointer<Void> id, T value) {
    final completer = _pending.remove(id.address);

    completer.complete(value);
  }
}

final asyncDispatcher = Dispatcher();
