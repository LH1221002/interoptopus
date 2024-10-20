// Automatically generated by Interoptopus.

#ifndef interoptopus_generated
#define interoptopus_generated

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>



const uint8_t U8 = 255;
const float F32_MIN_POSITIVE = 0.000000000000000000000000000000000000011754944;
const int32_t COMPUTED_I32 = -2147483647;

/// Documented enum.
typedef enum enumdocumented
    {
    /// Variant A.
    ENUMDOCUMENTED_A = 0,
    /// Variant B.
    ENUMDOCUMENTED_B = 1,
    /// Variant B.
    ENUMDOCUMENTED_C = 2,
    } enumdocumented;

typedef enum enumrenamed
    {
    ENUMRENAMED_X = 0,
    } enumrenamed;

typedef struct basicservice basicservice;

typedef struct generic2u8 generic2u8;

typedef struct generic3 generic3;

typedef struct generic4 generic4;

/// Some struct we want to expose as a class.
typedef struct servicecallbacks servicecallbacks;

typedef struct serviceignoringmethods serviceignoringmethods;

/// Some struct we want to expose as a class.
typedef struct servicemultiplectors servicemultiplectors;

/// Some struct we want to expose as a class.
typedef struct serviceonpanic serviceonpanic;

/// Some struct we want to expose as a class.
typedef struct servicestrings servicestrings;

/// Services can use lifetimes. However, they are more dangerous to use
/// via FFI, since you will not get any help tracking lifetimes there.
typedef struct serviceusinglifetimes serviceusinglifetimes;

/// Some struct we want to expose as a class.
typedef struct servicevariousslices servicevariousslices;

typedef enum ffierror
    {
    FFIERROR_OK = 0,
    FFIERROR_NULL = 100,
    FFIERROR_PANIC = 200,
    FFIERROR_DELEGATE = 300,
    FFIERROR_FAIL = 400,
    } ffierror;

typedef struct booleanalignment
    {
    int32_t a;
    int16_t b;
    int16_t c;
    uint8_t d;
    uint8_t e;
    uint8_t f;
    uint8_t g;
    uint8_t h;
    uint8_t i;
    uint8_t j;
    uint8_t k;
    uint64_t id;
    bool is_valid;
    uint64_t datum;
    } booleanalignment;

typedef struct extratypef32
    {
    float x;
    } extratypef32;

typedef struct inner
    {
    float x;
    } inner;

typedef struct local
    {
    uint32_t x;
    } local;

typedef struct packed1
    {
    uint8_t x;
    uint16_t y;
    } packed1;

typedef struct packed2
    {
    uint16_t y;
    uint8_t x;
    } packed2;

typedef struct phantomu8
    {
    uint32_t x;
    } phantomu8;

/// Documented struct.
typedef struct structdocumented
    {
    /// Documented field.
    float x;
    } structdocumented;

typedef struct structrenamed
    {
    enumrenamed e;
    } structrenamed;

typedef struct tupled
    {
    uint8_t x0;
    } tupled;

typedef struct useasciistringpattern
    {
    const char* ascii_string;
    } useasciistringpattern;

typedef struct vec
    {
    double x;
    double z;
    } vec;

typedef struct vec1
    {
    float x;
    float y;
    } vec1;

typedef struct vec2
    {
    double x;
    double z;
    } vec2;

typedef struct vec3f32
    {
    float x;
    float y;
    float z;
    } vec3f32;

typedef struct visibility1
    {
    uint8_t pblc;
    uint8_t prvt;
    } visibility1;

typedef struct visibility2
    {
    uint8_t pblc1;
    uint8_t pblc2;
    } visibility2;

typedef struct weird1u32
    {
    uint32_t x;
    } weird1u32;

typedef uint8_t (*fptr_fn_u8_rval_u8)(uint8_t x0);

typedef uint8_t (*callbacku8)(uint8_t value);

typedef uint32_t (*mycallback)(uint32_t value);

typedef uint32_t (*mycallbacknamespaced)(uint32_t value);

typedef void (*sumdelegate1)();

typedef int32_t (*sumdelegate2)(int32_t x, int32_t y);

typedef ffierror (*sumdelegatereturn)(int32_t x, int32_t y);

typedef void (*sumdelegatereturn2)(int32_t x, int32_t y);

typedef struct array
    {
    uint8_t data[16];
    } array;

typedef struct container
    {
    local foreign;
    } container;

typedef struct genericu32
    {
    const uint32_t* x;
    } genericu32;

typedef struct genericu8
    {
    const uint8_t* x;
    } genericu8;

typedef struct weird2u8
    {
    uint8_t t;
    uint8_t a[5];
    const uint8_t* r;
    } weird2u8;

///A pointer to an array of data someone else owns which may not be modified.
typedef struct slicebool
    {
    ///Pointer to start of immutable data.
    const uint8_t* data;
    ///Number of elements.
    uint64_t len;
    } slicebool;

///A pointer to an array of data someone else owns which may not be modified.
typedef struct slicei32
    {
    ///Pointer to start of immutable data.
    const int32_t* data;
    ///Number of elements.
    uint64_t len;
    } slicei32;

///A pointer to an array of data someone else owns which may not be modified.
typedef struct sliceu32
    {
    ///Pointer to start of immutable data.
    const uint32_t* data;
    ///Number of elements.
    uint64_t len;
    } sliceu32;

///A pointer to an array of data someone else owns which may not be modified.
typedef struct sliceu8
    {
    ///Pointer to start of immutable data.
    const uint8_t* data;
    ///Number of elements.
    uint64_t len;
    } sliceu8;

///A pointer to an array of data someone else owns which may be modified.
typedef struct slicemutconstptri8
    {
    ///Pointer to start of mutable data.
    const const char** data;
    ///Number of elements.
    uint64_t len;
    } slicemutconstptri8;

///A pointer to an array of data someone else owns which may be modified.
typedef struct slicemutu32
    {
    ///Pointer to start of mutable data.
    const uint32_t* data;
    ///Number of elements.
    uint64_t len;
    } slicemutu32;

///A pointer to an array of data someone else owns which may be modified.
typedef struct slicemutu8
    {
    ///Pointer to start of mutable data.
    const uint8_t* data;
    ///Number of elements.
    uint64_t len;
    } slicemutu8;

///Option type containing boolean flag and maybe valid data.
typedef struct optioninner
    {
    ///Element that is maybe valid.
    inner t;
    ///Byte where `1` means element `t` is valid.
    uint8_t is_some;
    } optioninner;

///Option type containing boolean flag and maybe valid data.
typedef struct optionvec
    {
    ///Element that is maybe valid.
    vec t;
    ///Byte where `1` means element `t` is valid.
    uint8_t is_some;
    } optionvec;

typedef void (*mycallbackcontextual)(const void* context, uint32_t value);

typedef void (*mycallbackvoid)(const void* ptr);

typedef struct delegatecallbackmycallbackcontextual
    {
    mycallbackcontextual callback;
    const void* context;
    } delegatecallbackmycallbackcontextual;

typedef struct delegatetable
    {
    mycallback my_callback;
    mycallbacknamespaced my_callback_namespaced;
    mycallbackvoid my_callback_void;
    mycallbackcontextual my_callback_contextual;
    sumdelegate1 sum_delegate_1;
    sumdelegate2 sum_delegate_2;
    sumdelegatereturn sum_delegate_return;
    sumdelegatereturn2 sum_delegate_return_2;
    } delegatetable;

///A pointer to an array of data someone else owns which may not be modified.
typedef struct sliceuseasciistringpattern
    {
    ///Pointer to start of immutable data.
    const useasciistringpattern* data;
    ///Number of elements.
    uint64_t len;
    } sliceuseasciistringpattern;

///A pointer to an array of data someone else owns which may not be modified.
typedef struct slicevec
    {
    ///Pointer to start of immutable data.
    const vec* data;
    ///Number of elements.
    uint64_t len;
    } slicevec;

///A pointer to an array of data someone else owns which may not be modified.
typedef struct slicevec3f32
    {
    ///Pointer to start of immutable data.
    const vec3f32* data;
    ///Number of elements.
    uint64_t len;
    } slicevec3f32;

///A pointer to an array of data someone else owns which may be modified.
typedef struct slicemutvec
    {
    ///Pointer to start of mutable data.
    const vec* data;
    ///Number of elements.
    uint64_t len;
    } slicemutvec;

typedef uint8_t (*callbackffislice)(sliceu8 slice);

typedef void (*callbackslicemut)(slicemutu8 slice);

typedef vec3f32 (*callbackhugevecslice)(slicevec3f32 slice);


void primitive_void();

void primitive_void2();

bool primitive_bool(bool x);

uint8_t primitive_u8(uint8_t x);

uint16_t primitive_u16(uint16_t x);

uint32_t primitive_u32(uint32_t x);

uint64_t primitive_u64(uint64_t x);

int8_t primitive_i8(int8_t x);

int16_t primitive_i16(int16_t x);

int32_t primitive_i32(int32_t x);

int64_t primitive_i64(int64_t x);

booleanalignment boolean_alignment(booleanalignment x);

booleanalignment boolean_alignment2(bool rval);

packed2 packed_to_packed1(packed1 a);

int64_t many_args_5(int64_t x0, int64_t x1, int64_t x2, int64_t x3, int64_t x4);

int64_t many_args_10(int64_t x0, int64_t x1, int64_t x2, int64_t x3, int64_t x4, int64_t x5, int64_t x6, int64_t x7, int64_t x8, int64_t x9);

const int64_t* ptr(const int64_t* x);

/// # Safety
///
/// Parameter x must point to valid data.
int64_t* ptr_mut(int64_t* x);

const const int64_t** ptr_ptr(const const int64_t** x);

const int64_t* ref_simple(const int64_t* x);

int64_t* ref_mut_simple(int64_t* x);

bool ref_option(const int64_t* x);

bool ref_mut_option(int64_t* x);

tupled call_tupled(tupled x);

ffierror complex_args_1(vec3f32 a, const tupled* b);

uint8_t callback(fptr_fn_u8_rval_u8 callback, uint8_t value);

uint32_t generic_1a(genericu32 x, phantomu8 y);

uint8_t generic_1b(genericu8 x, phantomu8 y);

uint8_t generic_1c(const genericu8* x, const genericu8* y);

uint8_t generic_2(const generic2u8* x);

uint8_t generic_3(const generic3* x);

uint8_t generic_4(const generic4* x);

uint8_t array_1(array x);

/// This function has documentation.
enumdocumented documented(structdocumented x);

vec1 ambiguous_1(vec1 x);

vec2 ambiguous_2(vec2 x);

bool ambiguous_3(vec1 x, vec2 y);

vec namespaced_type(vec x);

optionvec namespaced_inner_option(optionvec x);

slicevec namespaced_inner_slice(slicevec x);

slicemutvec namespaced_inner_slice_mut(slicemutvec x);

ffierror panics();

enumrenamed renamed(structrenamed x);

void sleep(uint64_t millis);

bool weird_1(weird1u32 x, weird2u8 y);

void visibility(visibility1 x, visibility2 y);

tupled repr_transparent(tupled x, const tupled* r);

uint32_t pattern_ascii_pointer_1(const char* x);

const char* pattern_ascii_pointer_2();

uint32_t pattern_ascii_pointer_len(const char* x, useasciistringpattern y);

sliceuseasciistringpattern pattern_ascii_pointer_return_slice();

uint32_t pattern_ffi_slice_1(sliceu32 ffi_slice);

uint32_t pattern_ffi_slice_1b(slicemutu32 ffi_slice);

vec3f32 pattern_ffi_slice_2(slicevec3f32 ffi_slice, int32_t i);

void pattern_ffi_slice_3(slicemutu8 slice, callbackslicemut callback);

void pattern_ffi_slice_4(sliceu8 slice, slicemutu8 slice2);

void pattern_ffi_slice_5(const sliceu8* slice, slicemutu8* slice2);

void pattern_ffi_slice_6(const slicemutu8* slice, callbacku8 callback);

uint32_t pattern_ffi_slice_7(slicemutconstptri8 slices);

uint8_t pattern_ffi_slice_delegate(callbackffislice callback);

vec3f32 pattern_ffi_slice_delegate_huge(callbackhugevecslice callback);

optioninner pattern_ffi_option_1(optioninner ffi_slice);

inner pattern_ffi_option_2(optioninner ffi_slice);

uint8_t pattern_ffi_bool(uint8_t ffi_bool);

char pattern_ffi_cchar(char ffi_cchar);

const char* pattern_ffi_cchar_const_pointer(const char* ffi_cchar);

char* pattern_ffi_cchar_mut_pointer(char* ffi_cchar);

uint64_t pattern_api_guard();

uint32_t pattern_callback_1(mycallback callback, uint32_t x);

mycallbackvoid pattern_callback_2(mycallbackvoid callback);

void pattern_callback_3(delegatecallbackmycallbackcontextual callback, uint32_t x);

uint32_t pattern_callback_4(mycallbacknamespaced callback, uint32_t x);

sumdelegate1 pattern_callback_5();

sumdelegate2 pattern_callback_6();

ffierror pattern_callback_7(sumdelegatereturn c1, sumdelegatereturn2 c2, int32_t x, int32_t i, int32_t* o);

void pattern_surrogates_1(local s, container* c);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
ffierror basic_service_destroy(basicservice** context);

ffierror basic_service_new(basicservice** context);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
ffierror service_on_panic_destroy(serviceonpanic** context);

ffierror service_on_panic_new(serviceonpanic** context);

/// Methods returning a Result<(), _> are the default and do not
/// need annotations.
ffierror service_on_panic_return_result(const serviceonpanic* context, uint32_t anon1);

/// Methods returning a value need an `on_panic` annotation.
uint32_t service_on_panic_return_default_value(const serviceonpanic* context, uint32_t x);

/// This function has no panic safeguards. It will be a bit faster to
/// call, but if it panics your host app will be in an undefined state.
const char* service_on_panic_return_ub_on_panic(serviceonpanic* context);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
ffierror service_callbacks_destroy(servicecallbacks** context);

ffierror service_callbacks_new(servicecallbacks** context);

ffierror service_callbacks_callback_simple(servicecallbacks* context, mycallback callback);

ffierror service_callbacks_callback_ffi_return(servicecallbacks* context, sumdelegatereturn callback);

ffierror service_callbacks_callback_with_slice(servicecallbacks* context, sumdelegatereturn callback, slicei32 input);

void service_callbacks_set_delegate_table(servicecallbacks* context, const delegatetable* table);

ffierror service_callbacks_invoke_delegates(const servicecallbacks* context);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
ffierror service_ignoring_methods_destroy(serviceignoringmethods** context);

ffierror service_ignoring_methods_new(serviceignoringmethods** context);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
ffierror service_multiple_ctors_destroy(servicemultiplectors** context);

ffierror service_multiple_ctors_new_with(servicemultiplectors** context, uint32_t some_value);

ffierror service_multiple_ctors_new_without(servicemultiplectors** context);

ffierror service_multiple_ctors_new_with_string(servicemultiplectors** context, const char* anon0);

ffierror service_multiple_ctors_new_failing(servicemultiplectors** context, uint8_t some_value);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
ffierror service_using_lifetimes_destroy(serviceusinglifetimes** context);

ffierror service_using_lifetimes_new_with(serviceusinglifetimes** context, const uint32_t* some_value);

void service_using_lifetimes_lifetime_1(serviceusinglifetimes* context, slicebool slice);

void service_using_lifetimes_lifetime_2(serviceusinglifetimes* context, slicebool slice);

const char* service_using_lifetimes_return_string_accept_slice(serviceusinglifetimes* anon0, sliceu8 anon1);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
ffierror service_various_slices_destroy(servicevariousslices** context);

ffierror service_various_slices_new(servicevariousslices** context);

uint8_t service_various_slices_mut_self(servicevariousslices* context, sliceu8 slice);

/// Single line.
void service_various_slices_mut_self_void(servicevariousslices* context, slicebool slice);

uint8_t service_various_slices_mut_self_ref(servicevariousslices* context, const uint8_t* x, uint8_t* y);

uint8_t service_various_slices_mut_self_ref_slice(servicevariousslices* context, const uint8_t* x, uint8_t* y, sliceu8 slice);

uint8_t service_various_slices_mut_self_ref_slice_limited(servicevariousslices* context, const uint8_t* x, uint8_t* y, sliceu8 slice, sliceu8 slice2);

ffierror service_various_slices_mut_self_ffi_error(servicevariousslices* context, slicemutu8 slice);

ffierror service_various_slices_mut_self_no_error(servicevariousslices* context, slicemutu8 slice);

/// Warning, you _must_ discard the returned slice object before calling into this service
/// again, as otherwise undefined behavior might happen.
sliceu32 service_various_slices_return_slice(servicevariousslices* context);

/// Warning, you _must_ discard the returned slice object before calling into this service
/// again, as otherwise undefined behavior might happen.
slicemutu32 service_various_slices_return_slice_mut(servicevariousslices* context);

/// Destroys the given instance.
///
/// # Safety
///
/// The passed parameter MUST have been created with the corresponding init function;
/// passing any other value results in undefined behavior.
ffierror service_strings_destroy(servicestrings** context);

ffierror service_strings_new(servicestrings** context);

void service_strings_pass_string(servicestrings* context, const char* anon1);

const char* service_strings_return_string(servicestrings* context);


#ifdef __cplusplus
}
#endif

#endif /* interoptopus_generated */
