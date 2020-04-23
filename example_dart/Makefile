SOURCE ?= main.dart
PROFILE ?= release

DART ?= dart

LIBDIRS += ../target/$(PROFILE)

CARGOFLAGS += --features dart

ifeq ($(PROFILE),release)
CARGOFLAGS += --release
endif

ifeq ($(OS),Windows_NT)
BINSFX := .exe
PATHSEP := ;
LIB_PATH_VAR := PATH
else
BINSFX := $()
PATHSEP := :
UNAME := $(shell uname -s)
ifeq ($(UNAME),Darwin)
LIB_PATH_VAR := DYLD_FALLBACK_LIBRARY_PATH
else
LIB_PATH_VAR := LD_LIBRARY_PATH
endif
endif

BINARY := main$(BINSFX)
LIB_PATHS := $(subst $() ,$(PATHSEP),$(LIBDIRS))

run: prepare build-lib
	@echo Run example
	$(LIB_PATH_VAR)="$(if $($(LIB_PATH_VAR)),$($(LIB_PATH_VAR))$(PATHSEP))$(LIB_PATHS)" $(DART) $(CURDIR)/$(SOURCE)

run-aot: $(BINARY) build-lib
	@echo Run aot example
	$(LIB_PATH_VAR)="$(if $($(LIB_PATH_VAR)),$($(LIB_PATH_VAR))$(PATHSEP))$(LIB_PATHS)" $(CURDIR)/$(BINARY)

$(BINARY): prepare
	dart2native -o $@ $(SOURCE)

clean:
	rm -f $(BINARY)

prepare:
	pub get

build-lib:
	cd ../example_lib && cargo build $(CARGOFLAGS)
