import 'dart:async';
import 'dart:ffi';
import 'package:ffi/ffi.dart';
import './async.dart';

typedef _rustDelayCb = Void Function(Pointer<Void> userdata);
typedef _rustDelay = Void Function(Float duration, Pointer<NativeFunction<_rustDelayCb>> callback, Pointer<Void> userdata);
typedef _RustDelay = void Function(double duration, Pointer<NativeFunction<_rustDelayCb>> callback, Pointer<Void> userdata);

typedef _rustReadFileCb = Void Function(Pointer<Utf8> data, Pointer<Utf8> error, Pointer<Void> userdata);
typedef _rustReadFile = Void Function(Pointer<Utf8> path, Pointer<NativeFunction<_rustReadFileCb>> callback, Pointer<Void> userdata);
typedef _RustReadFile = void Function(Pointer<Utf8> path, Pointer<NativeFunction<_rustReadFileCb>> callback, Pointer<Void> userdata);

final class IPAddr extends Struct {
  @Uint8() external int b0;
  @Uint8() external int b1;
  @Uint8() external int b2;
  @Uint8() external int b3;
  @Uint8() external int b4;
  @Uint8() external int b5;
  @Uint8() external int b6;
  @Uint8() external int b7;
  @Uint8() external int b8;
  @Uint8() external int b9;
  @Uint8() external int b10;
  @Uint8() external int b11;
  @Uint8() external int b12;
  @Uint8() external int b13;
  @Uint8() external int b14;
  @Uint8() external int b15;
  @Uint8() external int kind;
}

class IpAddr {
  final List<int> data;

  int get kind => data.length == 4 ? 4 : data.length == 8 ? 6 : 0;

  IpAddr(this.data) {}

  IpAddr.from(IPAddr addr) : data = addr.kind == 4 ? [
    addr.b0, addr.b1, addr.b2, addr.b3
  ] : addr.kind == 6 ? [
    (addr.b0 << 8) | addr.b1, (addr.b2 << 8) | addr.b3,
    (addr.b4 << 8) | addr.b5, (addr.b6 << 8) | addr.b7,
    (addr.b8 << 8) | addr.b9, (addr.b10 << 8) | addr.b11,
    (addr.b12 << 8) | addr.b13, (addr.b14 << 8) | addr.b15
  ] : [] {}

  String toString() {
    switch (kind) {
      case 4: return '${data[0]}.${data[1]}.${data[2]}.${data[3]}';
      case 6: return '${_to_hex4(data[0])}:${_to_hex4(data[1])}:${_to_hex4(data[2])}:${_to_hex4(data[3])}:${_to_hex4(data[4])}:${_to_hex4(data[5])}:${_to_hex4(data[6])}:${_to_hex4(data[7])}';
    }
    return 'invalid';
  }
}

String _to_hex4(int val) {
  return _to_hex(val, 4);
}

String _to_hex(int val, [int len = 8]) {
  return String.fromCharCodes(Iterable.generate(len, (off) => (val >> (len - 1 - off) * 4) & 0xf)
    .map((h) => h + (h < 10 ? '0'.codeUnitAt(0) : ('a'.codeUnitAt(0) - 10))));
}

typedef _rustNsLookupCb = Void Function(Pointer<IPAddr> addr, Pointer<Utf8> error, Pointer<Void> userdata);
typedef _rustNsLookup = Void Function(Pointer<Utf8> domain, Pointer<NativeFunction<_rustNsLookupCb>> callback, Pointer<Void> userdata);
typedef _RustNsLookup = void Function(Pointer<Utf8> domain, Pointer<NativeFunction<_rustNsLookupCb>> callback, Pointer<Void> userdata);

typedef _rustFreeIPAddr = Void Function(Pointer<IPAddr> data);
typedef _RustFreeIPAddr = void Function(Pointer<IPAddr> data);

typedef _rustFreeCStr = Void Function(Pointer<Utf8> data);
typedef _RustFreeCStr = void Function(Pointer<Utf8> data);

class ExampleLib {
  final Pointer<NativeFunction<_rustDelayCb>> _delayCb;
  final _RustDelay _delay;
  final Pointer<NativeFunction<_rustReadFileCb>> _readFileCb;
  final _RustReadFile _readFile;
  final Pointer<NativeFunction<_rustNsLookupCb>> _nsLookupCb;
  final _RustNsLookup _nsLookup;
  final _RustFreeIPAddr _freeIPAddr;
  final _RustFreeCStr _freeCStr;

  ExampleLib(DynamicLibrary dylib)
  : _delayCb = Pointer.fromFunction(_delayCb_)
  , _delay = dylib.lookup<NativeFunction<_rustDelay>>('delay').asFunction()
  , _readFileCb = Pointer.fromFunction(_readFileCb_)
  , _readFile = dylib.lookup<NativeFunction<_rustReadFile>>('read_file').asFunction()
  , _nsLookupCb = Pointer.fromFunction(_nsLookupCb_)
  , _nsLookup = dylib.lookup<NativeFunction<_rustNsLookup>>('ns_lookup').asFunction()
  , _freeIPAddr = dylib.lookup<NativeFunction<_rustFreeIPAddr>>('free_ipaddr').asFunction()
  , _freeCStr = dylib.lookup<NativeFunction<_rustFreeCStr>>('free_cstr').asFunction()
  {}

  static void _delayCb_(Pointer<Void> taskId) {
    asyncDispatcher.complete(taskId, (ExampleLib lib) {
        return null;
    });
  }

  Future<void> delay(double duration) {
    final Task<Null> task = asyncDispatcher.initiate(this);

    _delay(duration, _delayCb, task.id);

    return task.future;
  }

  static void _readFileCb_(Pointer<Utf8> rawData, Pointer<Utf8> rawError, Pointer<Void> taskId) {
    asyncDispatcher.complete(taskId, (ExampleLib lib) {
        if (rawData != nullptr) {
          final data = rawData.toDartString();
          lib._freeCStr(rawData);

          return data;
        } else {
          final error = rawError.toDartString();
          lib._freeCStr(rawError);

          throw error;
        }
    });
  }

  Future<String> readFile(String path) async {
    final Task<String> task = asyncDispatcher.initiate(this);

    final rawPath = path.toNativeUtf8();

    _readFile(rawPath, _readFileCb, task.id);

    final data = await task.future;

    malloc.free(rawPath);

    return data;
  }

  static void _nsLookupCb_(Pointer<IPAddr> rawAddr, Pointer<Utf8> rawError, Pointer<Void> taskId) {
    asyncDispatcher.complete(taskId, (ExampleLib lib) {
        if (rawAddr != nullptr) {
          final addr = IpAddr.from(rawAddr.ref);
          lib._freeIPAddr(rawAddr);

          return addr;
        } else {
          final error = rawError.toDartString();
          lib._freeCStr(rawError);

          throw error;
        }
    });
  }

  Future<IpAddr> nsLookup(String domain) async {
    final Task<IpAddr> task = asyncDispatcher.initiate(this);

    final rawDomain = domain.toNativeUtf8();

    _nsLookup(rawDomain, _nsLookupCb, task.id);

    final data = await task.future;

    malloc.free(rawDomain);

    return data;
  }
}
