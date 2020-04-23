TARGET ?= example_uv
SOURCES ?= main.c
OBJECTS = $(patsubst %.c,%.c.o,$(SOURCES))
PROFILE ?= release

PKGDEPS += libuv
INCDIRS += ../target/$(PROFILE)/include
LIBDIRS += ../target/$(PROFILE)

pkgconfig = $(foreach PKGDEP,$(PKGDEPS),$(shell pkg-config $(1) $(PKGDEP)))

CFLAGS += -Wall -Wextra -pedantic
CFLAGS += $(addprefix -I,$(INCDIRS))
CFLAGS += $(call pkgconfig,--cflags)

CARGOFLAGS += --features uv

ifeq ($(PROFILE),release)
CFLAGS += -O2
CARGOFLAGS += --release
else
CFLAGS += -O1 -g
endif

LDFLAGS += $(addprefix -L,$(LIBDIRS))
LDFLAGS += -lexample_lib
LDFLAGS += $(call pkgconfig,--libs)

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

LIB_PATHS := $(subst $() ,$(PATHSEP),$(LIBDIRS))

BINARY ?= $(TARGET)$(BINSFX)

run: $(BINARY)
	@echo Run example
	export $(LIB_PATH_VAR)="$(if $($(LIB_PATH_VAR)),$($(LIB_PATH_VAR))$(PATHSEP))$(LIB_PATHS)"; $(CURDIR)/$(BINARY)

build: $(BINARY)

clean:
	rm -f *.o $(BINARY)

build-lib:
	cd ../example_lib && cargo build $(CARGOFLAGS)

$(BINARY): $(OBJECTS)
	$(CC) $(CFLAGS) -o $@ $(filter %.o,$^) $(LDFLAGS)

define compile_c
$(1).o: $(1) build-lib
	$$(CC) $$(CFLAGS) -c -o $$@ $$<
endef

$(foreach SOURCE,$(SOURCES),$(eval $(call compile_c,$(SOURCE))))
