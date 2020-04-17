BINARY ?= example_bin
SOURCES ?= main.c
OBJECTS = $(patsubst %.c,%.c.o,$(SOURCES))
PROFILE ?= release

PKGDEPS += libuv
INCDIRS += ../target/$(PROFILE)/include
LIBDIRS += ../target/$(PROFILE)

pkgconfig = $(foreach PKGDEP,$(PKGDEPS),$(shell pkg-config $(1) $(PKGDEP)))

CFLAGS += -Wall -Wextra
CFLAGS += $(addprefix -I,$(INCDIRS))
CFLAGS += $(call pkgconfig,--cflags)

ifeq ($(PROFILE),release)
CFLAGS += -O2
CARGOFLAGS += --release
else
CFLAGS += -O1 -g
endif

LDFLAGS += $(call pkgconfig,--libs)
LDFLAGS += $(addprefix -L,$(LIBDIRS))
LDFLAGS += -lexample_lib

ifeq ($(OS),Windows_NT)
BINSFX := .exe
LIB_PATH_VAR := PATH
else
BINSFX := $()
UNAME := $(shell uname -s)
ifeq ($(UNAME),Darwin)
LIB_PATH_VAR := DYLD_FALLBACK_LIBRARY_PATH
else
LIB_PATH_VAR := LD_LIBRARY_PATH
endif
endif

LIB_PATHS := $(subst $() ,:,$(LIBDIRS))

run: $(BINARY)
	$(LIB_PATH_VAR)=$(LIB_PATHS) $(CURDIR)/$<

build: $(BINARY)

clean:
	rm -f *.o $(BINARY)

prepare:
	cd ../example_lib && cargo build $(CARGOFLAGS)

$(BINARY): prepare $(OBJECTS)
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $(filter %.o,$^)

define compile_c
$(1).o: $(1)
	$$(CC) $$(CFLAGS) -c -o $$@ $$<
endef

$(foreach SOURCE,$(SOURCES),$(eval $(call compile_c,$(SOURCE))))

.PHONY: prepare
