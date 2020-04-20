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
    final completer = Completer<T>();

    _lastId += 1;
    _pending[_lastId] = completer;

    return Wrapper(Pointer.fromAddress(_lastId), completer.future);
  }

  void complete<T>(Pointer<Void> id, T value) {
    final Completer<T> completer = _pending.remove(id.address);

    completer.complete(value);
  }

  void failure<E>(Pointer<Void> id, E error) {
    final completer = _pending.remove(id.address);

    completer.completeError(error);
  }
}

final asyncDispatcher = Dispatcher();
