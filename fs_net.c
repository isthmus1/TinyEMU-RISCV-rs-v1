





typedef long unsigned int size_t;
typedef int wchar_t;




typedef struct
  {
    int quot;
    int rem;
  } div_t;



typedef struct
  {
    long int quot;
    long int rem;
  } ldiv_t;





__extension__ typedef struct
  {
    long long int quot;
    long long int rem;
  } lldiv_t;
extern size_t __ctype_get_mb_cur_max (void) __attribute__ ((__nothrow__ , __leaf__)) ;



extern double atof (const char *__nptr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1))) ;

extern int atoi (const char *__nptr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1))) ;

extern long int atol (const char *__nptr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1))) ;



__extension__ extern long long int atoll (const char *__nptr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1))) ;



extern double strtod (const char *__restrict __nptr,
        char **__restrict __endptr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



extern float strtof (const char *__restrict __nptr,
       char **__restrict __endptr) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));

extern long double strtold (const char *__restrict __nptr,
       char **__restrict __endptr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));
extern _Float32 strtof32 (const char *__restrict __nptr,
     char **__restrict __endptr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



extern _Float64 strtof64 (const char *__restrict __nptr,
     char **__restrict __endptr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



extern _Float128 strtof128 (const char *__restrict __nptr,
       char **__restrict __endptr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



extern _Float32x strtof32x (const char *__restrict __nptr,
       char **__restrict __endptr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



extern _Float64x strtof64x (const char *__restrict __nptr,
       char **__restrict __endptr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));
extern long int strtol (const char *__restrict __nptr,
   char **__restrict __endptr, int __base)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));

extern unsigned long int strtoul (const char *__restrict __nptr,
      char **__restrict __endptr, int __base)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



__extension__
extern long long int strtoq (const char *__restrict __nptr,
        char **__restrict __endptr, int __base)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));

__extension__
extern unsigned long long int strtouq (const char *__restrict __nptr,
           char **__restrict __endptr, int __base)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));




__extension__
extern long long int strtoll (const char *__restrict __nptr,
         char **__restrict __endptr, int __base)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));

__extension__
extern unsigned long long int strtoull (const char *__restrict __nptr,
     char **__restrict __endptr, int __base)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));






extern long int strtol (const char *__restrict __nptr, char **__restrict __endptr, int __base) __asm__ ("" "__isoc23_strtol") __attribute__ ((__nothrow__ , __leaf__))


     __attribute__ ((__nonnull__ (1)));
extern unsigned long int strtoul (const char *__restrict __nptr, char **__restrict __endptr, int __base) __asm__ ("" "__isoc23_strtoul") __attribute__ ((__nothrow__ , __leaf__))



     __attribute__ ((__nonnull__ (1)));

__extension__
extern long long int strtoq (const char *__restrict __nptr, char **__restrict __endptr, int __base) __asm__ ("" "__isoc23_strtoll") __attribute__ ((__nothrow__ , __leaf__))


     __attribute__ ((__nonnull__ (1)));
__extension__
extern unsigned long long int strtouq (const char *__restrict __nptr, char **__restrict __endptr, int __base) __asm__ ("" "__isoc23_strtoull") __attribute__ ((__nothrow__ , __leaf__))



     __attribute__ ((__nonnull__ (1)));

__extension__
extern long long int strtoll (const char *__restrict __nptr, char **__restrict __endptr, int __base) __asm__ ("" "__isoc23_strtoll") __attribute__ ((__nothrow__ , __leaf__))


     __attribute__ ((__nonnull__ (1)));
__extension__
extern unsigned long long int strtoull (const char *__restrict __nptr, char **__restrict __endptr, int __base) __asm__ ("" "__isoc23_strtoull") __attribute__ ((__nothrow__ , __leaf__))



     __attribute__ ((__nonnull__ (1)));
extern int strfromd (char *__dest, size_t __size, const char *__format,
       double __f)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3)));

extern int strfromf (char *__dest, size_t __size, const char *__format,
       float __f)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3)));

extern int strfroml (char *__dest, size_t __size, const char *__format,
       long double __f)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3)));
extern int strfromf32 (char *__dest, size_t __size, const char * __format,
         _Float32 __f)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3)));



extern int strfromf64 (char *__dest, size_t __size, const char * __format,
         _Float64 __f)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3)));



extern int strfromf128 (char *__dest, size_t __size, const char * __format,
   _Float128 __f)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3)));



extern int strfromf32x (char *__dest, size_t __size, const char * __format,
   _Float32x __f)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3)));



extern int strfromf64x (char *__dest, size_t __size, const char * __format,
   _Float64x __f)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3)));
struct __locale_struct
{

  struct __locale_data *__locales[13];


  const unsigned short int *__ctype_b;
  const int *__ctype_tolower;
  const int *__ctype_toupper;


  const char *__names[13];
};

typedef struct __locale_struct *__locale_t;

typedef __locale_t locale_t;

extern long int strtol_l (const char *__restrict __nptr,
     char **__restrict __endptr, int __base,
     locale_t __loc) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 4)));

extern unsigned long int strtoul_l (const char *__restrict __nptr,
        char **__restrict __endptr,
        int __base, locale_t __loc)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 4)));

__extension__
extern long long int strtoll_l (const char *__restrict __nptr,
    char **__restrict __endptr, int __base,
    locale_t __loc)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 4)));

__extension__
extern unsigned long long int strtoull_l (const char *__restrict __nptr,
       char **__restrict __endptr,
       int __base, locale_t __loc)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 4)));





extern long int strtol_l (const char *__restrict __nptr, char **__restrict __endptr, int __base, locale_t __loc) __asm__ ("" "__isoc23_strtol_l") __attribute__ ((__nothrow__ , __leaf__))



     __attribute__ ((__nonnull__ (1, 4)));
extern unsigned long int strtoul_l (const char *__restrict __nptr, char **__restrict __endptr, int __base, locale_t __loc) __asm__ ("" "__isoc23_strtoul_l") __attribute__ ((__nothrow__ , __leaf__))




     __attribute__ ((__nonnull__ (1, 4)));
__extension__
extern long long int strtoll_l (const char *__restrict __nptr, char **__restrict __endptr, int __base, locale_t __loc) __asm__ ("" "__isoc23_strtoll_l") __attribute__ ((__nothrow__ , __leaf__))




     __attribute__ ((__nonnull__ (1, 4)));
__extension__
extern unsigned long long int strtoull_l (const char *__restrict __nptr, char **__restrict __endptr, int __base, locale_t __loc) __asm__ ("" "__isoc23_strtoull_l") __attribute__ ((__nothrow__ , __leaf__))




     __attribute__ ((__nonnull__ (1, 4)));
extern double strtod_l (const char *__restrict __nptr,
   char **__restrict __endptr, locale_t __loc)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 3)));

extern float strtof_l (const char *__restrict __nptr,
         char **__restrict __endptr, locale_t __loc)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 3)));

extern long double strtold_l (const char *__restrict __nptr,
         char **__restrict __endptr,
         locale_t __loc)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 3)));
extern _Float32 strtof32_l (const char *__restrict __nptr,
       char **__restrict __endptr,
       locale_t __loc)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 3)));



extern _Float64 strtof64_l (const char *__restrict __nptr,
       char **__restrict __endptr,
       locale_t __loc)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 3)));



extern _Float128 strtof128_l (const char *__restrict __nptr,
         char **__restrict __endptr,
         locale_t __loc)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 3)));



extern _Float32x strtof32x_l (const char *__restrict __nptr,
         char **__restrict __endptr,
         locale_t __loc)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 3)));



extern _Float64x strtof64x_l (const char *__restrict __nptr,
         char **__restrict __endptr,
         locale_t __loc)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 3)));
extern char *l64a (long int __n) __attribute__ ((__nothrow__ , __leaf__)) ;


extern long int a64l (const char *__s)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1))) ;








typedef unsigned char __u_char;
typedef unsigned short int __u_short;
typedef unsigned int __u_int;
typedef unsigned long int __u_long;


typedef signed char __int8_t;
typedef unsigned char __uint8_t;
typedef signed short int __int16_t;
typedef unsigned short int __uint16_t;
typedef signed int __int32_t;
typedef unsigned int __uint32_t;

typedef signed long int __int64_t;
typedef unsigned long int __uint64_t;






typedef __int8_t __int_least8_t;
typedef __uint8_t __uint_least8_t;
typedef __int16_t __int_least16_t;
typedef __uint16_t __uint_least16_t;
typedef __int32_t __int_least32_t;
typedef __uint32_t __uint_least32_t;
typedef __int64_t __int_least64_t;
typedef __uint64_t __uint_least64_t;



typedef long int __quad_t;
typedef unsigned long int __u_quad_t;







typedef long int __intmax_t;
typedef unsigned long int __uintmax_t;


typedef unsigned long int __dev_t;
typedef unsigned int __uid_t;
typedef unsigned int __gid_t;
typedef unsigned long int __ino_t;
typedef unsigned long int __ino64_t;
typedef unsigned int __mode_t;
typedef unsigned long int __nlink_t;
typedef long int __off_t;
typedef long int __off64_t;
typedef int __pid_t;
typedef struct { int __val[2]; } __fsid_t;
typedef long int __clock_t;
typedef unsigned long int __rlim_t;
typedef unsigned long int __rlim64_t;
typedef unsigned int __id_t;
typedef long int __time_t;
typedef unsigned int __useconds_t;
typedef long int __suseconds_t;
typedef long int __suseconds64_t;

typedef int __daddr_t;
typedef int __key_t;


typedef int __clockid_t;


typedef void * __timer_t;


typedef long int __blksize_t;




typedef long int __blkcnt_t;
typedef long int __blkcnt64_t;


typedef unsigned long int __fsblkcnt_t;
typedef unsigned long int __fsblkcnt64_t;


typedef unsigned long int __fsfilcnt_t;
typedef unsigned long int __fsfilcnt64_t;


typedef long int __fsword_t;

typedef long int __ssize_t;


typedef long int __syscall_slong_t;

typedef unsigned long int __syscall_ulong_t;



typedef __off64_t __loff_t;
typedef char *__caddr_t;


typedef long int __intptr_t;


typedef unsigned int __socklen_t;




typedef int __sig_atomic_t;



typedef __u_char u_char;
typedef __u_short u_short;
typedef __u_int u_int;
typedef __u_long u_long;
typedef __quad_t quad_t;
typedef __u_quad_t u_quad_t;
typedef __fsid_t fsid_t;


typedef __loff_t loff_t;






typedef __ino64_t ino_t;




typedef __ino64_t ino64_t;




typedef __dev_t dev_t;




typedef __gid_t gid_t;




typedef __mode_t mode_t;




typedef __nlink_t nlink_t;




typedef __uid_t uid_t;







typedef __off64_t off_t;




typedef __off64_t off64_t;




typedef __pid_t pid_t;





typedef __id_t id_t;




typedef __ssize_t ssize_t;





typedef __daddr_t daddr_t;
typedef __caddr_t caddr_t;





typedef __key_t key_t;










typedef __clock_t clock_t;







typedef __clockid_t clockid_t;
typedef __time_t time_t;






typedef __timer_t timer_t;



typedef __useconds_t useconds_t;



typedef __suseconds_t suseconds_t;








typedef unsigned long int ulong;
typedef unsigned short int ushort;
typedef unsigned int uint;




typedef __int8_t int8_t;
typedef __int16_t int16_t;
typedef __int32_t int32_t;
typedef __int64_t int64_t;


typedef __uint8_t u_int8_t;
typedef __uint16_t u_int16_t;
typedef __uint32_t u_int32_t;
typedef __uint64_t u_int64_t;


typedef int register_t __attribute__ ((__mode__ (__word__)));
static __inline __uint16_t
__bswap_16 (__uint16_t __bsx)
{

  return __builtin_bswap16 (__bsx);



}






static __inline __uint32_t
__bswap_32 (__uint32_t __bsx)
{

  return __builtin_bswap32 (__bsx);



}
__extension__ static __inline __uint64_t
__bswap_64 (__uint64_t __bsx)
{

  return __builtin_bswap64 (__bsx);



}
static __inline __uint16_t
__uint16_identity (__uint16_t __x)
{
  return __x;
}

static __inline __uint32_t
__uint32_identity (__uint32_t __x)
{
  return __x;
}

static __inline __uint64_t
__uint64_identity (__uint64_t __x)
{
  return __x;
}











typedef struct
{
  unsigned long int __val[(1024 / (8 * sizeof (unsigned long int)))];
} __sigset_t;


typedef __sigset_t sigset_t;










struct timeval
{




  __time_t tv_sec;
  __suseconds_t tv_usec;

};

struct timespec
{



  __time_t tv_sec;




  __syscall_slong_t tv_nsec;
};
typedef long int __fd_mask;
typedef struct
  {



    __fd_mask fds_bits[1024 / (8 * (int) sizeof (__fd_mask))];





  } fd_set;






typedef __fd_mask fd_mask;

extern int select (int __nfds, fd_set *__restrict __readfds,
     fd_set *__restrict __writefds,
     fd_set *__restrict __exceptfds,
     struct timeval *__restrict __timeout);
extern int pselect (int __nfds, fd_set *__restrict __readfds,
      fd_set *__restrict __writefds,
      fd_set *__restrict __exceptfds,
      const struct timespec *__restrict __timeout,
      const __sigset_t *__restrict __sigmask);






typedef __blksize_t blksize_t;
typedef __blkcnt64_t blkcnt_t;



typedef __fsblkcnt64_t fsblkcnt_t;



typedef __fsfilcnt64_t fsfilcnt_t;





typedef __blkcnt64_t blkcnt64_t;
typedef __fsblkcnt64_t fsblkcnt64_t;
typedef __fsfilcnt64_t fsfilcnt64_t;






typedef union
{
  __extension__ unsigned long long int __value64;
  struct
  {
    unsigned int __low;
    unsigned int __high;
  } __value32;
} __atomic_wide_counter;




typedef struct __pthread_internal_list
{
  struct __pthread_internal_list *__prev;
  struct __pthread_internal_list *__next;
} __pthread_list_t;

typedef struct __pthread_internal_slist
{
  struct __pthread_internal_slist *__next;
} __pthread_slist_t;
struct __pthread_mutex_s
{
  int __lock;
  unsigned int __count;
  int __owner;

  unsigned int __nusers;



  int __kind;

  short __spins;
  short __glibc_reserved;
  __pthread_list_t __list;
};
struct __pthread_rwlock_arch_t
{
  unsigned int __readers;
  unsigned int __writers;
  unsigned int __wrphase_futex;
  unsigned int __writers_futex;
  unsigned int __pad3;
  unsigned int __pad4;

  int __cur_writer;
  int __shared;
  unsigned long int __pad1;
  unsigned long int __pad2;


  unsigned int __flags;
};




struct __pthread_cond_s
{
  __atomic_wide_counter __wseq;
  __atomic_wide_counter __g1_start;
  unsigned int __g_size[2] ;
  unsigned int __g1_orig_size;
  unsigned int __wrefs;
  unsigned int __g_signals[2];
  unsigned int __unused_initialized_1;
  unsigned int __unused_initialized_2;
};

typedef unsigned int __tss_t;
typedef unsigned long int __thrd_t;

typedef struct
{
  int __data ;
} __once_flag;



typedef unsigned long int pthread_t;




typedef union
{
  char __size[4];
  int __align;
} pthread_mutexattr_t;




typedef union
{
  char __size[4];
  int __align;
} pthread_condattr_t;



typedef unsigned int pthread_key_t;



typedef int pthread_once_t;


union pthread_attr_t
{
  char __size[56];
  long int __align;
};

typedef union pthread_attr_t pthread_attr_t;




typedef union
{
  struct __pthread_mutex_s __data;
  char __size[40];
  long int __align;
} pthread_mutex_t;


typedef union
{
  struct __pthread_cond_s __data;
  char __size[48];
  __extension__ long long int __align;
} pthread_cond_t;





typedef union
{
  struct __pthread_rwlock_arch_t __data;
  char __size[56];
  long int __align;
} pthread_rwlock_t;

typedef union
{
  char __size[8];
  long int __align;
} pthread_rwlockattr_t;





typedef volatile int pthread_spinlock_t;




typedef union
{
  char __size[32];
  long int __align;
} pthread_barrier_t;

typedef union
{
  char __size[4];
  int __align;
} pthread_barrierattr_t;









extern long int random (void) __attribute__ ((__nothrow__ , __leaf__));


extern void srandom (unsigned int __seed) __attribute__ ((__nothrow__ , __leaf__));





extern char *initstate (unsigned int __seed, char *__statebuf,
   size_t __statelen) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2)));



extern char *setstate (char *__statebuf) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));







struct random_data
  {
    int32_t *fptr;
    int32_t *rptr;
    int32_t *state;
    int rand_type;
    int rand_deg;
    int rand_sep;
    int32_t *end_ptr;
  };

extern int random_r (struct random_data *__restrict __buf,
       int32_t *__restrict __result) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));

extern int srandom_r (unsigned int __seed, struct random_data *__buf)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2)));

extern int initstate_r (unsigned int __seed, char *__restrict __statebuf,
   size_t __statelen,
   struct random_data *__restrict __buf)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2, 4)));

extern int setstate_r (char *__restrict __statebuf,
         struct random_data *__restrict __buf)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));





extern int rand (void) __attribute__ ((__nothrow__ , __leaf__));

extern void srand (unsigned int __seed) __attribute__ ((__nothrow__ , __leaf__));



extern int rand_r (unsigned int *__seed) __attribute__ ((__nothrow__ , __leaf__));







extern double drand48 (void) __attribute__ ((__nothrow__ , __leaf__));
extern double erand48 (unsigned short int __xsubi[3]) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern long int lrand48 (void) __attribute__ ((__nothrow__ , __leaf__));
extern long int nrand48 (unsigned short int __xsubi[3])
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern long int mrand48 (void) __attribute__ ((__nothrow__ , __leaf__));
extern long int jrand48 (unsigned short int __xsubi[3])
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern void srand48 (long int __seedval) __attribute__ ((__nothrow__ , __leaf__));
extern unsigned short int *seed48 (unsigned short int __seed16v[3])
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));
extern void lcong48 (unsigned short int __param[7]) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));





struct drand48_data
  {
    unsigned short int __x[3];
    unsigned short int __old_x[3];
    unsigned short int __c;
    unsigned short int __init;
    __extension__ unsigned long long int __a;

  };


extern int drand48_r (struct drand48_data *__restrict __buffer,
        double *__restrict __result) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));
extern int erand48_r (unsigned short int __xsubi[3],
        struct drand48_data *__restrict __buffer,
        double *__restrict __result) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int lrand48_r (struct drand48_data *__restrict __buffer,
        long int *__restrict __result)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));
extern int nrand48_r (unsigned short int __xsubi[3],
        struct drand48_data *__restrict __buffer,
        long int *__restrict __result)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int mrand48_r (struct drand48_data *__restrict __buffer,
        long int *__restrict __result)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));
extern int jrand48_r (unsigned short int __xsubi[3],
        struct drand48_data *__restrict __buffer,
        long int *__restrict __result)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int srand48_r (long int __seedval, struct drand48_data *__buffer)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2)));

extern int seed48_r (unsigned short int __seed16v[3],
       struct drand48_data *__buffer) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));

extern int lcong48_r (unsigned short int __param[7],
        struct drand48_data *__buffer)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern __uint32_t arc4random (void)
     __attribute__ ((__nothrow__ , __leaf__)) ;


extern void arc4random_buf (void *__buf, size_t __size)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



extern __uint32_t arc4random_uniform (__uint32_t __upper_bound)
     __attribute__ ((__nothrow__ , __leaf__)) ;




extern void *malloc (size_t __size) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__malloc__))
     __attribute__ ((__alloc_size__ (1))) ;

extern void *calloc (size_t __nmemb, size_t __size)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__malloc__)) __attribute__ ((__alloc_size__ (1, 2))) ;






extern void *realloc (void *__ptr, size_t __size)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__warn_unused_result__)) __attribute__ ((__alloc_size__ (2)));


extern void free (void *__ptr) __attribute__ ((__nothrow__ , __leaf__));
extern void free_sized (void *__ptr, size_t __size) __attribute__ ((__nothrow__ , __leaf__));




extern void free_aligned_sized (void *__ptr, size_t __alignment, size_t __size)
     __attribute__ ((__nothrow__ , __leaf__));
extern void *reallocarray (void *__ptr, size_t __nmemb, size_t __size)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__warn_unused_result__))
     __attribute__ ((__alloc_size__ (2, 3)))
    __attribute__ ((__malloc__ (__builtin_free, 1)));


extern void *reallocarray (void *__ptr, size_t __nmemb, size_t __size)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__malloc__ (reallocarray, 1)));










extern void *alloca (size_t __size) __attribute__ ((__nothrow__ , __leaf__));











extern void *valloc (size_t __size) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__malloc__))
     __attribute__ ((__alloc_size__ (1))) ;




extern int posix_memalign (void **__memptr, size_t __alignment, size_t __size)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1))) ;




extern void *aligned_alloc (size_t __alignment, size_t __size)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__malloc__)) __attribute__ ((__alloc_align__ (1)))
     __attribute__ ((__alloc_size__ (2))) ;



extern void abort (void) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__noreturn__)) __attribute__ ((__cold__));



extern int atexit (void (*__func) (void)) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));







extern int at_quick_exit (void (*__func) (void)) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));






extern int on_exit (void (*__func) (int __status, void *__arg), void *__arg)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));





extern void exit (int __status) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__noreturn__));





extern void quick_exit (int __status) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__noreturn__));





extern void _Exit (int __status) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__noreturn__));




extern char *getenv (const char *__name) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1))) ;




extern char *secure_getenv (const char *__name)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1))) ;






extern int putenv (char *__string) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));





extern int setenv (const char *__name, const char *__value, int __replace)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2)));


extern int unsetenv (const char *__name) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));






extern int clearenv (void) __attribute__ ((__nothrow__ , __leaf__));
extern char *mktemp (char *__template) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));
extern int mkstemp (char *__template) __asm__ ("" "mkstemp64")
     __attribute__ ((__nonnull__ (1))) ;





extern int mkstemp64 (char *__template) __attribute__ ((__nonnull__ (1))) ;
extern int mkstemps (char *__template, int __suffixlen) __asm__ ("" "mkstemps64")
                     __attribute__ ((__nonnull__ (1))) ;





extern int mkstemps64 (char *__template, int __suffixlen)
     __attribute__ ((__nonnull__ (1))) ;
extern char *mkdtemp (char *__template) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1))) ;
extern int mkostemp (char *__template, int __flags) __asm__ ("" "mkostemp64")
     __attribute__ ((__nonnull__ (1))) ;





extern int mkostemp64 (char *__template, int __flags) __attribute__ ((__nonnull__ (1))) ;
extern int mkostemps (char *__template, int __suffixlen, int __flags) __asm__ ("" "mkostemps64")

     __attribute__ ((__nonnull__ (1))) ;





extern int mkostemps64 (char *__template, int __suffixlen, int __flags)
     __attribute__ ((__nonnull__ (1))) ;
extern int system (const char *__command) ;





extern char *canonicalize_file_name (const char *__name)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1))) __attribute__ ((__malloc__))
     __attribute__ ((__malloc__ (__builtin_free, 1))) ;
extern char *realpath (const char *__restrict __name,
         char *__restrict __resolved) __attribute__ ((__nothrow__ , __leaf__)) ;






typedef int (*__compar_fn_t) (const void *, const void *);


typedef __compar_fn_t comparison_fn_t;



typedef int (*__compar_d_fn_t) (const void *, const void *, void *);




extern void *bsearch (const void *__key, const void *__base,
        size_t __nmemb, size_t __size, __compar_fn_t __compar)
     __attribute__ ((__nonnull__ (1, 2, 5))) ;
extern void qsort (void *__base, size_t __nmemb, size_t __size,
     __compar_fn_t __compar) __attribute__ ((__nonnull__ (1, 4)));

extern void qsort_r (void *__base, size_t __nmemb, size_t __size,
       __compar_d_fn_t __compar, void *__arg)
  __attribute__ ((__nonnull__ (1, 4)));




extern int abs (int __x) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__)) ;
extern long int labs (long int __x) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__)) ;


__extension__ extern long long int llabs (long long int __x)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__)) ;



extern unsigned int uabs (int __x) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__)) ;
extern unsigned long int ulabs (long int __x) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__)) ;
__extension__ extern unsigned long long int ullabs (long long int __x)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__)) ;





extern div_t div (int __numer, int __denom)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__)) ;
extern ldiv_t ldiv (long int __numer, long int __denom)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__)) ;


__extension__ extern lldiv_t lldiv (long long int __numer,
        long long int __denom)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__)) ;
extern char *ecvt (double __value, int __ndigit, int *__restrict __decpt,
     int *__restrict __sign) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3, 4))) ;




extern char *fcvt (double __value, int __ndigit, int *__restrict __decpt,
     int *__restrict __sign) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3, 4))) ;




extern char *gcvt (double __value, int __ndigit, char *__buf)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3))) ;




extern char *qecvt (long double __value, int __ndigit,
      int *__restrict __decpt, int *__restrict __sign)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3, 4))) ;
extern char *qfcvt (long double __value, int __ndigit,
      int *__restrict __decpt, int *__restrict __sign)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3, 4))) ;
extern char *qgcvt (long double __value, int __ndigit, char *__buf)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3))) ;




extern int ecvt_r (double __value, int __ndigit, int *__restrict __decpt,
     int *__restrict __sign, char *__restrict __buf,
     size_t __len) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3, 4, 5)));
extern int fcvt_r (double __value, int __ndigit, int *__restrict __decpt,
     int *__restrict __sign, char *__restrict __buf,
     size_t __len) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3, 4, 5)));

extern int qecvt_r (long double __value, int __ndigit,
      int *__restrict __decpt, int *__restrict __sign,
      char *__restrict __buf, size_t __len)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3, 4, 5)));
extern int qfcvt_r (long double __value, int __ndigit,
      int *__restrict __decpt, int *__restrict __sign,
      char *__restrict __buf, size_t __len)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3, 4, 5)));





extern int mblen (const char *__s, size_t __n) __attribute__ ((__nothrow__ , __leaf__));


extern int mbtowc (wchar_t *__restrict __pwc,
     const char *__restrict __s, size_t __n) __attribute__ ((__nothrow__ , __leaf__));


extern int wctomb (char *__s, wchar_t __wchar) __attribute__ ((__nothrow__ , __leaf__));



extern size_t mbstowcs (wchar_t *__restrict __pwcs,
   const char *__restrict __s, size_t __n) __attribute__ ((__nothrow__ , __leaf__))
    __attribute__ ((__access__ (__read_only__, 2)));

extern size_t wcstombs (char *__restrict __s,
   const wchar_t *__restrict __pwcs, size_t __n)
     __attribute__ ((__nothrow__ , __leaf__))
  __attribute__ ((__access__ (__write_only__, 1, 3)))
  __attribute__ ((__access__ (__read_only__, 2)));






extern int rpmatch (const char *__response) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1))) ;
extern int getsubopt (char **__restrict __optionp,
        char *const *__restrict __tokens,
        char **__restrict __valuep)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2, 3))) ;







extern int posix_openpt (int __oflag) ;







extern int grantpt (int __fd) __attribute__ ((__nothrow__ , __leaf__));



extern int unlockpt (int __fd) __attribute__ ((__nothrow__ , __leaf__));




extern char *ptsname (int __fd) __attribute__ ((__nothrow__ , __leaf__)) ;






extern int ptsname_r (int __fd, char *__buf, size_t __buflen)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2))) __attribute__ ((__access__ (__write_only__, 2, 3)));


extern int getpt (void);






extern int getloadavg (double __loadavg[], int __nelem)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));
typedef __once_flag once_flag;



extern void call_once (once_flag *__flag, void (*__func)(void));



extern size_t memalignment (const void *__p);














typedef __builtin_va_list __gnuc_va_list;






typedef struct
{
  int __count;
  union
  {
    unsigned int __wch;
    char __wchb[4];
  } __value;
} __mbstate_t;




typedef struct _G_fpos_t
{
  __off_t __pos;
  __mbstate_t __state;
} __fpos_t;
typedef struct _G_fpos64_t
{
  __off64_t __pos;
  __mbstate_t __state;
} __fpos64_t;



struct _IO_FILE;
typedef struct _IO_FILE __FILE;



struct _IO_FILE;


typedef struct _IO_FILE FILE;

struct _IO_FILE;
struct _IO_marker;
struct _IO_codecvt;
struct _IO_wide_data;




typedef void _IO_lock_t;





struct _IO_FILE
{
  int _flags;


  char *_IO_read_ptr;
  char *_IO_read_end;
  char *_IO_read_base;
  char *_IO_write_base;
  char *_IO_write_ptr;
  char *_IO_write_end;
  char *_IO_buf_base;
  char *_IO_buf_end;


  char *_IO_save_base;
  char *_IO_backup_base;
  char *_IO_save_end;

  struct _IO_marker *_markers;

  struct _IO_FILE *_chain;

  int _fileno;
  int _flags2:24;

  char _short_backupbuf[1];
  __off_t _old_offset;


  unsigned short _cur_column;
  signed char _vtable_offset;
  char _shortbuf[1];

  _IO_lock_t *_lock;







  __off64_t _offset;

  struct _IO_codecvt *_codecvt;
  struct _IO_wide_data *_wide_data;
  struct _IO_FILE *_freeres_list;
  void *_freeres_buf;
  struct _IO_FILE **_prevchain;
  int _mode;

  int _unused3;

  __uint64_t _total_written;




  char _unused2[12 * sizeof (int) - 5 * sizeof (void *)];
};


typedef __ssize_t cookie_read_function_t (void *__cookie, char *__buf,
                                          size_t __nbytes);







typedef __ssize_t cookie_write_function_t (void *__cookie, const char *__buf,
                                           size_t __nbytes);







typedef int cookie_seek_function_t (void *__cookie, __off64_t *__pos, int __w);


typedef int cookie_close_function_t (void *__cookie);






typedef struct _IO_cookie_io_functions_t
{
  cookie_read_function_t *read;
  cookie_write_function_t *write;
  cookie_seek_function_t *seek;
  cookie_close_function_t *close;
} cookie_io_functions_t;





typedef __gnuc_va_list va_list;
typedef __fpos64_t fpos_t;


typedef __fpos64_t fpos64_t;
extern FILE *stdin;
extern FILE *stdout;
extern FILE *stderr;






extern int remove (const char *__filename) __attribute__ ((__nothrow__ , __leaf__));

extern int rename (const char *__old, const char *__new) __attribute__ ((__nothrow__ , __leaf__));



extern int renameat (int __oldfd, const char *__old, int __newfd,
       const char *__new) __attribute__ ((__nothrow__ , __leaf__));
extern int renameat2 (int __oldfd, const char *__old, int __newfd,
        const char *__new, unsigned int __flags) __attribute__ ((__nothrow__ , __leaf__));






extern int fclose (FILE *__stream) __attribute__ ((__nonnull__ (1)));
extern FILE *tmpfile (void) __asm__ ("" "tmpfile64")
  __attribute__ ((__malloc__)) __attribute__ ((__malloc__ (fclose, 1))) ;






extern FILE *tmpfile64 (void)
   __attribute__ ((__malloc__)) __attribute__ ((__malloc__ (fclose, 1))) ;



extern char *tmpnam (char[20]) __attribute__ ((__nothrow__ , __leaf__)) ;




extern char *tmpnam_r (char __s[20]) __attribute__ ((__nothrow__ , __leaf__)) ;
extern char *tempnam (const char *__dir, const char *__pfx)
   __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__malloc__)) __attribute__ ((__malloc__ (__builtin_free, 1)));






extern int fflush (FILE *__stream);
extern int fflush_unlocked (FILE *__stream);
extern int fcloseall (void);
extern FILE *fopen (const char *__restrict __filename, const char *__restrict __modes) __asm__ ("" "fopen64")

  __attribute__ ((__malloc__)) __attribute__ ((__malloc__ (fclose, 1))) ;
extern FILE *freopen (const char *__restrict __filename, const char *__restrict __modes, FILE *__restrict __stream) __asm__ ("" "freopen64")


  __attribute__ ((__nonnull__ (3)));






extern FILE *fopen64 (const char *__restrict __filename,
        const char *__restrict __modes)
  __attribute__ ((__malloc__)) __attribute__ ((__malloc__ (fclose, 1))) ;
extern FILE *freopen64 (const char *__restrict __filename,
   const char *__restrict __modes,
   FILE *__restrict __stream) __attribute__ ((__nonnull__ (3)));




extern FILE *fdopen (int __fd, const char *__modes) __attribute__ ((__nothrow__ , __leaf__))
  __attribute__ ((__malloc__)) __attribute__ ((__malloc__ (fclose, 1))) ;





extern FILE *fopencookie (void *__restrict __magic_cookie,
     const char *__restrict __modes,
     cookie_io_functions_t __io_funcs) __attribute__ ((__nothrow__ , __leaf__))
  __attribute__ ((__malloc__)) __attribute__ ((__malloc__ (fclose, 1))) ;




extern FILE *fmemopen (void *__s, size_t __len, const char *__modes)
  __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__malloc__)) __attribute__ ((__malloc__ (fclose, 1))) ;




extern FILE *open_memstream (char **__bufloc, size_t *__sizeloc) __attribute__ ((__nothrow__ , __leaf__))
  __attribute__ ((__malloc__)) __attribute__ ((__malloc__ (fclose, 1))) ;
extern void setbuf (FILE *__restrict __stream, char *__restrict __buf) __attribute__ ((__nothrow__ , __leaf__))
  __attribute__ ((__nonnull__ (1)));



extern int setvbuf (FILE *__restrict __stream, char *__restrict __buf,
      int __modes, size_t __n) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));




extern void setbuffer (FILE *__restrict __stream, char *__restrict __buf,
         size_t __size) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern void setlinebuf (FILE *__stream) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));







extern int fprintf (FILE *__restrict __stream,
      const char *__restrict __format, ...) __attribute__ ((__nonnull__ (1)));




extern int printf (const char *__restrict __format, ...);

extern int sprintf (char *__restrict __s,
      const char *__restrict __format, ...) __attribute__ ((__nothrow__));





extern int vfprintf (FILE *__restrict __s, const char *__restrict __format,
       __gnuc_va_list __arg) __attribute__ ((__nonnull__ (1)));




extern int vprintf (const char *__restrict __format, __gnuc_va_list __arg);

extern int vsprintf (char *__restrict __s, const char *__restrict __format,
       __gnuc_va_list __arg) __attribute__ ((__nothrow__));



extern int snprintf (char *__restrict __s, size_t __maxlen,
       const char *__restrict __format, ...)
     __attribute__ ((__nothrow__)) __attribute__ ((__format__ (__printf__, 3, 4)));

extern int vsnprintf (char *__restrict __s, size_t __maxlen,
        const char *__restrict __format, __gnuc_va_list __arg)
     __attribute__ ((__nothrow__)) __attribute__ ((__format__ (__printf__, 3, 0)));





extern int vasprintf (char **__restrict __ptr, const char *__restrict __f,
        __gnuc_va_list __arg)
     __attribute__ ((__nothrow__)) __attribute__ ((__format__ (__printf__, 2, 0))) ;
extern int __asprintf (char **__restrict __ptr,
         const char *__restrict __fmt, ...)
     __attribute__ ((__nothrow__)) __attribute__ ((__format__ (__printf__, 2, 3))) ;
extern int asprintf (char **__restrict __ptr,
       const char *__restrict __fmt, ...)
     __attribute__ ((__nothrow__)) __attribute__ ((__format__ (__printf__, 2, 3))) ;




extern int vdprintf (int __fd, const char *__restrict __fmt,
       __gnuc_va_list __arg)
     __attribute__ ((__format__ (__printf__, 2, 0)));
extern int dprintf (int __fd, const char *__restrict __fmt, ...)
     __attribute__ ((__format__ (__printf__, 2, 3)));







extern int fscanf (FILE *__restrict __stream,
     const char *__restrict __format, ...) __attribute__ ((__nonnull__ (1)));




extern int scanf (const char *__restrict __format, ...) ;

extern int sscanf (const char *__restrict __s,
     const char *__restrict __format, ...) __attribute__ ((__nothrow__ , __leaf__));
extern int fscanf (FILE *__restrict __stream, const char *__restrict __format, ...) __asm__ ("" "__isoc23_fscanf")

                                __attribute__ ((__nonnull__ (1)));
extern int scanf (const char *__restrict __format, ...) __asm__ ("" "__isoc23_scanf")
                              ;
extern int sscanf (const char *__restrict __s, const char *__restrict __format, ...) __asm__ ("" "__isoc23_sscanf") __attribute__ ((__nothrow__ , __leaf__))

                      ;
extern int vfscanf (FILE *__restrict __s, const char *__restrict __format,
      __gnuc_va_list __arg)
     __attribute__ ((__format__ (__scanf__, 2, 0))) __attribute__ ((__nonnull__ (1)));





extern int vscanf (const char *__restrict __format, __gnuc_va_list __arg)
     __attribute__ ((__format__ (__scanf__, 1, 0))) ;


extern int vsscanf (const char *__restrict __s,
      const char *__restrict __format, __gnuc_va_list __arg)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__format__ (__scanf__, 2, 0)));






extern int vfscanf (FILE *__restrict __s, const char *__restrict __format, __gnuc_va_list __arg) __asm__ ("" "__isoc23_vfscanf")



     __attribute__ ((__format__ (__scanf__, 2, 0))) __attribute__ ((__nonnull__ (1)));
extern int vscanf (const char *__restrict __format, __gnuc_va_list __arg) __asm__ ("" "__isoc23_vscanf")

     __attribute__ ((__format__ (__scanf__, 1, 0))) ;
extern int vsscanf (const char *__restrict __s, const char *__restrict __format, __gnuc_va_list __arg) __asm__ ("" "__isoc23_vsscanf") __attribute__ ((__nothrow__ , __leaf__))



     __attribute__ ((__format__ (__scanf__, 2, 0)));
extern int fgetc (FILE *__stream) __attribute__ ((__nonnull__ (1)));
extern int getc (FILE *__stream) __attribute__ ((__nonnull__ (1)));





extern int getchar (void);






extern int getc_unlocked (FILE *__stream) __attribute__ ((__nonnull__ (1)));
extern int getchar_unlocked (void);
extern int fgetc_unlocked (FILE *__stream) __attribute__ ((__nonnull__ (1)));







extern int fputc (int __c, FILE *__stream) __attribute__ ((__nonnull__ (2)));
extern int putc (int __c, FILE *__stream) __attribute__ ((__nonnull__ (2)));





extern int putchar (int __c);
extern int fputc_unlocked (int __c, FILE *__stream) __attribute__ ((__nonnull__ (2)));







extern int putc_unlocked (int __c, FILE *__stream) __attribute__ ((__nonnull__ (2)));
extern int putchar_unlocked (int __c);






extern int getw (FILE *__stream) __attribute__ ((__nonnull__ (1)));


extern int putw (int __w, FILE *__stream) __attribute__ ((__nonnull__ (2)));







extern char *fgets (char *__restrict __s, int __n, FILE *__restrict __stream)
     __attribute__ ((__access__ (__write_only__, 1, 2))) __attribute__ ((__nonnull__ (3)));
extern char *fgets_unlocked (char *__restrict __s, int __n,
        FILE *__restrict __stream)
    __attribute__ ((__access__ (__write_only__, 1, 2))) __attribute__ ((__nonnull__ (3)));
extern __ssize_t __getdelim (char **__restrict __lineptr,
                             size_t *__restrict __n, int __delimiter,
                             FILE *__restrict __stream) __attribute__ ((__nonnull__ (4)));
extern __ssize_t getdelim (char **__restrict __lineptr,
                           size_t *__restrict __n, int __delimiter,
                           FILE *__restrict __stream) __attribute__ ((__nonnull__ (4)));


extern __ssize_t getline (char **__restrict __lineptr,
                          size_t *__restrict __n,
                          FILE *__restrict __stream) __attribute__ ((__nonnull__ (3)));







extern int fputs (const char *__restrict __s, FILE *__restrict __stream)
  __attribute__ ((__nonnull__ (2)));





extern int puts (const char *__s);






extern int ungetc (int __c, FILE *__stream) __attribute__ ((__nonnull__ (2)));






extern size_t fread (void *__restrict __ptr, size_t __size,
       size_t __n, FILE *__restrict __stream)
  __attribute__ ((__nonnull__ (4)));




extern size_t fwrite (const void *__restrict __ptr, size_t __size,
        size_t __n, FILE *__restrict __s) __attribute__ ((__nonnull__ (4)));
extern int fputs_unlocked (const char *__restrict __s,
      FILE *__restrict __stream) __attribute__ ((__nonnull__ (2)));
extern size_t fread_unlocked (void *__restrict __ptr, size_t __size,
         size_t __n, FILE *__restrict __stream)
  __attribute__ ((__nonnull__ (4)));
extern size_t fwrite_unlocked (const void *__restrict __ptr, size_t __size,
          size_t __n, FILE *__restrict __stream)
  __attribute__ ((__nonnull__ (4)));







extern int fseek (FILE *__stream, long int __off, int __whence)
  __attribute__ ((__nonnull__ (1)));




extern long int ftell (FILE *__stream) __attribute__ ((__nonnull__ (1)));




extern void rewind (FILE *__stream) __attribute__ ((__nonnull__ (1)));
extern int fseeko (FILE *__stream, __off64_t __off, int __whence) __asm__ ("" "fseeko64")

                   __attribute__ ((__nonnull__ (1)));
extern __off64_t ftello (FILE *__stream) __asm__ ("" "ftello64")
  __attribute__ ((__nonnull__ (1)));
extern int fgetpos (FILE *__restrict __stream, fpos_t *__restrict __pos) __asm__ ("" "fgetpos64")

  __attribute__ ((__nonnull__ (1)));
extern int fsetpos (FILE *__stream, const fpos_t *__pos) __asm__ ("" "fsetpos64")

  __attribute__ ((__nonnull__ (1)));







extern int fseeko64 (FILE *__stream, __off64_t __off, int __whence)
  __attribute__ ((__nonnull__ (1)));
extern __off64_t ftello64 (FILE *__stream) __attribute__ ((__nonnull__ (1)));
extern int fgetpos64 (FILE *__restrict __stream, fpos64_t *__restrict __pos)
  __attribute__ ((__nonnull__ (1)));
extern int fsetpos64 (FILE *__stream, const fpos64_t *__pos) __attribute__ ((__nonnull__ (1)));



extern void clearerr (FILE *__stream) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));

extern int feof (FILE *__stream) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));

extern int ferror (FILE *__stream) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



extern void clearerr_unlocked (FILE *__stream) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));
extern int feof_unlocked (FILE *__stream) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));
extern int ferror_unlocked (FILE *__stream) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));







extern void perror (const char *__s) __attribute__ ((__cold__));




extern int fileno (FILE *__stream) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));




extern int fileno_unlocked (FILE *__stream) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));
extern int pclose (FILE *__stream) __attribute__ ((__nonnull__ (1)));





extern FILE *popen (const char *__command, const char *__modes)
  __attribute__ ((__malloc__)) __attribute__ ((__malloc__ (pclose, 1))) ;






extern char *ctermid (char *__s) __attribute__ ((__nothrow__ , __leaf__))
  __attribute__ ((__access__ (__write_only__, 1)));





extern char *cuserid (char *__s)
  __attribute__ ((__access__ (__write_only__, 1)));




struct obstack;


extern int obstack_printf (struct obstack *__restrict __obstack,
      const char *__restrict __format, ...)
     __attribute__ ((__nothrow__)) __attribute__ ((__format__ (__printf__, 2, 3)));
extern int obstack_vprintf (struct obstack *__restrict __obstack,
       const char *__restrict __format,
       __gnuc_va_list __args)
     __attribute__ ((__nothrow__)) __attribute__ ((__format__ (__printf__, 2, 0)));







extern void flockfile (FILE *__stream) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



extern int ftrylockfile (FILE *__stream) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern void funlockfile (FILE *__stream) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));
extern int __uflow (FILE *);
extern int __overflow (FILE *, int);



extern void *memcpy (void *__restrict __dest, const void *__restrict __src,
       size_t __n) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern void *memmove (void *__dest, const void *__src, size_t __n)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));





extern void *memccpy (void *__restrict __dest, const void *__restrict __src,
        int __c, size_t __n)
    __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2))) __attribute__ ((__access__ (__write_only__, 1, 4)));




extern void *memset (void *__s, int __c, size_t __n) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));




extern void *memset_explicit (void *__s, int __c, size_t __n)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1))) __attribute__ ((__access__ (__write_only__, 1, 3)));



extern int memcmp (const void *__s1, const void *__s2, size_t __n)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern int __memcmpeq (const void *__s1, const void *__s2, size_t __n)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern void *memchr (const void *__s, int __c, size_t __n)
      __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
extern void *rawmemchr (const void *__s, int __c)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
extern void *memrchr (const void *__s, int __c, size_t __n)
      __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)))
      __attribute__ ((__access__ (__read_only__, 1, 3)));





extern char *strcpy (char *__restrict __dest, const char *__restrict __src)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));

extern char *strncpy (char *__restrict __dest,
        const char *__restrict __src, size_t __n)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern char *strcat (char *__restrict __dest, const char *__restrict __src)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));

extern char *strncat (char *__restrict __dest, const char *__restrict __src,
        size_t __n) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int strcmp (const char *__s1, const char *__s2)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));

extern int strncmp (const char *__s1, const char *__s2, size_t __n)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));


extern int strcoll (const char *__s1, const char *__s2)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));

extern size_t strxfrm (char *__restrict __dest,
         const char *__restrict __src, size_t __n)
    __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2))) __attribute__ ((__access__ (__write_only__, 1, 3)));






extern int strcoll_l (const char *__s1, const char *__s2, locale_t __l)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2, 3)));


extern size_t strxfrm_l (char *__dest, const char *__src, size_t __n,
    locale_t __l) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2, 4)))
     __attribute__ ((__access__ (__write_only__, 1, 3)));





extern char *strdup (const char *__s)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__malloc__)) __attribute__ ((__nonnull__ (1)));






extern char *strndup (const char *__string, size_t __n)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__malloc__)) __attribute__ ((__nonnull__ (1)));
extern char *strchr (const char *__s, int __c)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
extern char *strrchr (const char *__s, int __c)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
extern char *strchrnul (const char *__s, int __c)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));





extern size_t strcspn (const char *__s, const char *__reject)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));


extern size_t strspn (const char *__s, const char *__accept)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern char *strpbrk (const char *__s, const char *__accept)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern char *strstr (const char *__haystack, const char *__needle)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));
extern char *strtok (char *__restrict __s, const char *__restrict __delim)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2)));



extern char *__strtok_r (char *__restrict __s,
    const char *__restrict __delim,
    char **__restrict __save_ptr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2, 3)));

extern char *strtok_r (char *__restrict __s, const char *__restrict __delim,
         char **__restrict __save_ptr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2, 3)));
extern char *strcasestr (const char *__haystack, const char *__needle)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));







extern void *memmem (const void *__haystack, size_t __haystacklen,
       const void *__needle, size_t __needlelen)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 3)))
    __attribute__ ((__access__ (__read_only__, 1, 2)))
    __attribute__ ((__access__ (__read_only__, 3, 4)));



extern void *__mempcpy (void *__restrict __dest,
   const void *__restrict __src, size_t __n)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));
extern void *mempcpy (void *__restrict __dest,
        const void *__restrict __src, size_t __n)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));




extern size_t strlen (const char *__s)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));




extern size_t strnlen (const char *__string, size_t __maxlen)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));




extern char *strerror (int __errnum) __attribute__ ((__nothrow__ , __leaf__));
extern char *strerror_r (int __errnum, char *__buf, size_t __buflen)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2))) __attribute__ ((__access__ (__write_only__, 2, 3)));




extern const char *strerrordesc_np (int __err) __attribute__ ((__nothrow__ , __leaf__));

extern const char *strerrorname_np (int __err) __attribute__ ((__nothrow__ , __leaf__));





extern char *strerror_l (int __errnum, locale_t __l) __attribute__ ((__nothrow__ , __leaf__));













extern int bcmp (const void *__s1, const void *__s2, size_t __n)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));


extern void bcopy (const void *__src, void *__dest, size_t __n)
  __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern void bzero (void *__s, size_t __n) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));
extern char *index (const char *__s, int __c)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));
extern char *rindex (const char *__s, int __c)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1)));






extern int ffs (int __i) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__));





extern int ffsl (long int __l) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__));
__extension__ extern int ffsll (long long int __ll)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__));



extern int strcasecmp (const char *__s1, const char *__s2)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));


extern int strncasecmp (const char *__s1, const char *__s2, size_t __n)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));






extern int strcasecmp_l (const char *__s1, const char *__s2, locale_t __loc)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2, 3)));



extern int strncasecmp_l (const char *__s1, const char *__s2,
     size_t __n, locale_t __loc)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2, 4)));






extern void explicit_bzero (void *__s, size_t __n) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)))
    __attribute__ ((__access__ (__write_only__, 1, 2)));



extern char *strsep (char **__restrict __stringp,
       const char *__restrict __delim)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));




extern char *strsignal (int __sig) __attribute__ ((__nothrow__ , __leaf__));



extern const char *sigabbrev_np (int __sig) __attribute__ ((__nothrow__ , __leaf__));


extern const char *sigdescr_np (int __sig) __attribute__ ((__nothrow__ , __leaf__));



extern char *__stpcpy (char *__restrict __dest, const char *__restrict __src)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));
extern char *stpcpy (char *__restrict __dest, const char *__restrict __src)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));



extern char *__stpncpy (char *__restrict __dest,
   const char *__restrict __src, size_t __n)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));
extern char *stpncpy (char *__restrict __dest,
        const char *__restrict __src, size_t __n)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));




extern size_t strlcpy (char *__restrict __dest,
         const char *__restrict __src, size_t __n)
  __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2))) __attribute__ ((__access__ (__write_only__, 1, 3)));



extern size_t strlcat (char *__restrict __dest,
         const char *__restrict __src, size_t __n)
  __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2))) __attribute__ ((__access__ (__read_write__, 1, 3)));




extern int strverscmp (const char *__s1, const char *__s2)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__pure__)) __attribute__ ((__nonnull__ (1, 2)));


extern char *strfry (char *__string) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern void *memfrob (void *__s, size_t __n) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)))
    __attribute__ ((__access__ (__read_write__, 1, 2)));
extern char *basename (const char *__filename) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));

 

typedef __uint8_t uint8_t;
typedef __uint16_t uint16_t;
typedef __uint32_t uint32_t;
typedef __uint64_t uint64_t;



typedef __int_least8_t int_least8_t;
typedef __int_least16_t int_least16_t;
typedef __int_least32_t int_least32_t;
typedef __int_least64_t int_least64_t;


typedef __uint_least8_t uint_least8_t;
typedef __uint_least16_t uint_least16_t;
typedef __uint_least32_t uint_least32_t;
typedef __uint_least64_t uint_least64_t;





typedef signed char int_fast8_t;

typedef long int int_fast16_t;
typedef long int int_fast32_t;
typedef long int int_fast64_t;
typedef unsigned char uint_fast8_t;

typedef unsigned long int uint_fast16_t;
typedef unsigned long int uint_fast32_t;
typedef unsigned long int uint_fast64_t;
typedef long int intptr_t;


typedef unsigned long int uintptr_t;
typedef __intmax_t intmax_t;
typedef __uintmax_t uintmax_t;






typedef int __gwchar_t;





typedef struct
  {
    long int quot;
    long int rem;
  } imaxdiv_t;
extern intmax_t imaxabs (intmax_t __n) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__));



extern uintmax_t umaxabs (intmax_t __n) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__));



extern imaxdiv_t imaxdiv (intmax_t __numer, intmax_t __denom)
      __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__));


extern intmax_t strtoimax (const char *__restrict __nptr,
      char **__restrict __endptr, int __base) __attribute__ ((__nothrow__ , __leaf__));


extern uintmax_t strtoumax (const char *__restrict __nptr,
       char ** __restrict __endptr, int __base) __attribute__ ((__nothrow__ , __leaf__));


extern intmax_t wcstoimax (const __gwchar_t *__restrict __nptr,
      __gwchar_t **__restrict __endptr, int __base)
     __attribute__ ((__nothrow__ , __leaf__));


extern uintmax_t wcstoumax (const __gwchar_t *__restrict __nptr,
       __gwchar_t ** __restrict __endptr, int __base)
     __attribute__ ((__nothrow__ , __leaf__));





extern intmax_t strtoimax (const char *__restrict __nptr, char **__restrict __endptr, int __base) __asm__ ("" "__isoc23_strtoimax") __attribute__ ((__nothrow__ , __leaf__))

                                         ;
extern uintmax_t strtoumax (const char *__restrict __nptr, char **__restrict __endptr, int __base) __asm__ ("" "__isoc23_strtoumax") __attribute__ ((__nothrow__ , __leaf__))

                                          ;
extern intmax_t wcstoimax (const __gwchar_t *__restrict __nptr, __gwchar_t **__restrict __endptr, int __base) __asm__ ("" "__isoc23_wcstoimax") __attribute__ ((__nothrow__ , __leaf__))


                       ;
extern uintmax_t wcstoumax (const __gwchar_t *__restrict __nptr, __gwchar_t **__restrict __endptr, int __base) __asm__ ("" "__isoc23_wcstoumax") __attribute__ ((__nothrow__ , __leaf__))


                        ;




extern void __assert_fail (const char *__assertion, const char *__file,
      unsigned int __line, const char *__function)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__noreturn__)) __attribute__ ((__cold__));


extern void __assert_perror_fail (int __errnum, const char *__file,
      unsigned int __line, const char *__function)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__noreturn__)) __attribute__ ((__cold__));




extern void __assert (const char *__assertion, const char *__file, int __line)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__noreturn__)) __attribute__ ((__cold__));






extern _Bool __assert_single_arg (_Bool);




struct timezone
  {
    int tz_minuteswest;
    int tz_dsttime;
  };
extern int gettimeofday (struct timeval *__restrict __tv,
    void *__restrict __tz) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));
extern int settimeofday (const struct timeval *__tv,
    const struct timezone *__tz)
     __attribute__ ((__nothrow__ , __leaf__));





extern int adjtime (const struct timeval *__delta,
      struct timeval *__olddelta) __attribute__ ((__nothrow__ , __leaf__));
enum __itimer_which
  {

    ITIMER_REAL = 0,


    ITIMER_VIRTUAL = 1,



    ITIMER_PROF = 2

  };



struct itimerval
  {

    struct timeval it_interval;

    struct timeval it_value;
  };




typedef enum __itimer_which __itimer_which_t;







extern int getitimer (__itimer_which_t __which,
        struct itimerval *__value) __attribute__ ((__nothrow__ , __leaf__));




extern int setitimer (__itimer_which_t __which,
        const struct itimerval *__restrict __new,
        struct itimerval *__restrict __old) __attribute__ ((__nothrow__ , __leaf__));




extern int utimes (const char *__file, const struct timeval __tvp[2])
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));
extern int lutimes (const char *__file, const struct timeval __tvp[2])
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int futimes (int __fd, const struct timeval __tvp[2]) __attribute__ ((__nothrow__ , __leaf__));
extern int futimesat (int __fd, const char *__file,
        const struct timeval __tvp[2]) __attribute__ ((__nothrow__ , __leaf__));


enum
{
  _ISupper = ((0) < 8 ? ((1 << (0)) << 8) : ((1 << (0)) >> 8)),
  _ISlower = ((1) < 8 ? ((1 << (1)) << 8) : ((1 << (1)) >> 8)),
  _ISalpha = ((2) < 8 ? ((1 << (2)) << 8) : ((1 << (2)) >> 8)),
  _ISdigit = ((3) < 8 ? ((1 << (3)) << 8) : ((1 << (3)) >> 8)),
  _ISxdigit = ((4) < 8 ? ((1 << (4)) << 8) : ((1 << (4)) >> 8)),
  _ISspace = ((5) < 8 ? ((1 << (5)) << 8) : ((1 << (5)) >> 8)),
  _ISprint = ((6) < 8 ? ((1 << (6)) << 8) : ((1 << (6)) >> 8)),
  _ISgraph = ((7) < 8 ? ((1 << (7)) << 8) : ((1 << (7)) >> 8)),
  _ISblank = ((8) < 8 ? ((1 << (8)) << 8) : ((1 << (8)) >> 8)),
  _IScntrl = ((9) < 8 ? ((1 << (9)) << 8) : ((1 << (9)) >> 8)),
  _ISpunct = ((10) < 8 ? ((1 << (10)) << 8) : ((1 << (10)) >> 8)),
  _ISalnum = ((11) < 8 ? ((1 << (11)) << 8) : ((1 << (11)) >> 8))
};
extern const unsigned short int **__ctype_b_loc (void)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__));
extern const __int32_t **__ctype_tolower_loc (void)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__));
extern const __int32_t **__ctype_toupper_loc (void)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__));
extern int isalnum (int) __attribute__ ((__nothrow__ , __leaf__));
extern int isalpha (int) __attribute__ ((__nothrow__ , __leaf__));
extern int iscntrl (int) __attribute__ ((__nothrow__ , __leaf__));
extern int isdigit (int) __attribute__ ((__nothrow__ , __leaf__));
extern int islower (int) __attribute__ ((__nothrow__ , __leaf__));
extern int isgraph (int) __attribute__ ((__nothrow__ , __leaf__));
extern int isprint (int) __attribute__ ((__nothrow__ , __leaf__));
extern int ispunct (int) __attribute__ ((__nothrow__ , __leaf__));
extern int isspace (int) __attribute__ ((__nothrow__ , __leaf__));
extern int isupper (int) __attribute__ ((__nothrow__ , __leaf__));
extern int isxdigit (int) __attribute__ ((__nothrow__ , __leaf__));



extern int tolower (int __c) __attribute__ ((__nothrow__ , __leaf__));


extern int toupper (int __c) __attribute__ ((__nothrow__ , __leaf__));




extern int isblank (int) __attribute__ ((__nothrow__ , __leaf__));




extern int isctype (int __c, int __mask) __attribute__ ((__nothrow__ , __leaf__));






extern int isascii (int __c) __attribute__ ((__nothrow__ , __leaf__));



extern int toascii (int __c) __attribute__ ((__nothrow__ , __leaf__));



extern int _toupper (int) __attribute__ ((__nothrow__ , __leaf__));
extern int _tolower (int) __attribute__ ((__nothrow__ , __leaf__));
extern int isalnum_l (int, locale_t) __attribute__ ((__nothrow__ , __leaf__));
extern int isalpha_l (int, locale_t) __attribute__ ((__nothrow__ , __leaf__));
extern int iscntrl_l (int, locale_t) __attribute__ ((__nothrow__ , __leaf__));
extern int isdigit_l (int, locale_t) __attribute__ ((__nothrow__ , __leaf__));
extern int islower_l (int, locale_t) __attribute__ ((__nothrow__ , __leaf__));
extern int isgraph_l (int, locale_t) __attribute__ ((__nothrow__ , __leaf__));
extern int isprint_l (int, locale_t) __attribute__ ((__nothrow__ , __leaf__));
extern int ispunct_l (int, locale_t) __attribute__ ((__nothrow__ , __leaf__));
extern int isspace_l (int, locale_t) __attribute__ ((__nothrow__ , __leaf__));
extern int isupper_l (int, locale_t) __attribute__ ((__nothrow__ , __leaf__));
extern int isxdigit_l (int, locale_t) __attribute__ ((__nothrow__ , __leaf__));

extern int isblank_l (int, locale_t) __attribute__ ((__nothrow__ , __leaf__));



extern int __tolower_l (int __c, locale_t __l) __attribute__ ((__nothrow__ , __leaf__));
extern int tolower_l (int __c, locale_t __l) __attribute__ ((__nothrow__ , __leaf__));


extern int __toupper_l (int __c, locale_t __l) __attribute__ ((__nothrow__ , __leaf__));
extern int toupper_l (int __c, locale_t __l) __attribute__ ((__nothrow__ , __leaf__));



typedef int BOOL;
enum {
    FALSE = 0,
    TRUE = 1,
};
typedef __int128 int128_t;
typedef unsigned __int128 uint128_t;


static inline int max_int(int a, int b)
{
    if (a > b)
        return a;
    else
        return b;
}

static inline int min_int(int a, int b)
{
    if (a < b)
        return a;
    else
        return b;
}

void *mallocz(size_t size);


static inline uint16_t get_le16(const uint8_t *ptr)
{
    return ptr[0] | (ptr[1] << 8);
}

static inline uint32_t get_le32(const uint8_t *ptr)
{
    return ptr[0] | (ptr[1] << 8) | (ptr[2] << 16) | (ptr[3] << 24);
}

static inline uint64_t get_le64(const uint8_t *ptr)
{
    return get_le32(ptr) | ((uint64_t)get_le32(ptr + 4) << 32);
}

static inline void put_le16(uint8_t *ptr, uint16_t v)
{
    ptr[0] = v;
    ptr[1] = v >> 8;
}

static inline void put_le32(uint8_t *ptr, uint32_t v)
{
    ptr[0] = v;
    ptr[1] = v >> 8;
    ptr[2] = v >> 16;
    ptr[3] = v >> 24;
}

static inline void put_le64(uint8_t *ptr, uint64_t v)
{
    put_le32(ptr, v);
    put_le32(ptr + 4, v >> 32);
}

static inline uint32_t get_be32(const uint8_t *d)
{
    return (d[0] << 24) | (d[1] << 16) | (d[2] << 8) | d[3];
}

static inline void put_be32(uint8_t *d, uint32_t v)
{
    d[0] = v >> 24;
    d[1] = v >> 16;
    d[2] = v >> 8;
    d[3] = v >> 0;
}

static inline void put_be64(uint8_t *d, uint64_t v)
{
    put_be32(d, v >> 32);
    put_be32(d + 4, v);
}







static inline uint32_t cpu_to_be32(uint32_t v)
{
    return 
          __bswap_32 (
          v
          )
                     ;
}


static inline int ctz32(uint32_t val)
{

    return val ? __builtin_ctz(val) : 32;
}


void *mallocz(size_t size);
void pstrcpy(char *buf, int buf_size, const char *str);
char *pstrcat(char *buf, int buf_size, const char *s);
int strstart(const char *str, const char *val, const char **ptr);

typedef struct {
    uint8_t *buf;
    size_t size;
    size_t allocated_size;
} DynBuf;

void dbuf_init(DynBuf *s);
void dbuf_write(DynBuf *s, size_t offset, const uint8_t *data, size_t len);
void dbuf_putc(DynBuf *s, uint8_t c);
void dbuf_putstr(DynBuf *s, const char *str);
void dbuf_free(DynBuf *s);
struct list_head {
    struct list_head *prev;
    struct list_head *next;
};





static inline void init_list_head(struct list_head *head)
{
    head->prev = head;
    head->next = head;
}


static inline void __list_add(struct list_head *el,
                              struct list_head *prev, struct list_head *next)
{
    prev->next = el;
    el->prev = prev;
    el->next = next;
    next->prev = el;
}


static inline void list_add(struct list_head *el, struct list_head *head)
{
    __list_add(el, head, head->next);
}


static inline void list_add_tail(struct list_head *el, struct list_head *head)
{
    __list_add(el, head->prev, head);
}

static inline void list_del(struct list_head *el)
{
    struct list_head *prev, *next;
    prev = el->prev;
    next = el->next;
    prev->next = next;
    next->prev = prev;
    el->prev = 
              ((void *)0)
                  ;
    el->next = 
              ((void *)0)
                  ;
}

static inline int list_empty(struct list_head *el)
{
    return el->next == el;
}
typedef struct FSDevice FSDevice;
typedef struct FSFile FSFile;

typedef struct {
    uint32_t f_bsize;
    uint64_t f_blocks;
    uint64_t f_bfree;
    uint64_t f_bavail;
    uint64_t f_files;
    uint64_t f_ffree;
} FSStatFS;

typedef struct {
    uint8_t type;
    uint32_t version;
    uint64_t path;
} FSQID;

typedef struct {
    FSQID qid;
    uint32_t st_mode;
    uint32_t st_uid;
    uint32_t st_gid;
    uint64_t st_nlink;
    uint64_t st_rdev;
    uint64_t st_size;
    uint64_t st_blksize;
    uint64_t st_blocks;
    uint64_t st_atime_sec;
    uint32_t st_atime_nsec;
    uint64_t st_mtime_sec;
    uint32_t st_mtime_nsec;
    uint64_t st_ctime_sec;
    uint32_t st_ctime_nsec;
} FSStat;
typedef struct {
    uint8_t type;
    uint32_t flags;
    uint64_t start;
    uint64_t length;
    uint32_t proc_id;
    char *client_id;
} FSLock;

typedef void FSOpenCompletionFunc(FSDevice *fs, FSQID *qid, int err,
                                  void *opaque);

struct FSDevice {
    void (*fs_end)(FSDevice *s);
    void (*fs_delete)(FSDevice *s, FSFile *f);
    void (*fs_statfs)(FSDevice *fs, FSStatFS *st);
    int (*fs_attach)(FSDevice *fs, FSFile **pf, FSQID *qid, uint32_t uid,
                     const char *uname, const char *aname);
    int (*fs_walk)(FSDevice *fs, FSFile **pf, FSQID *qids,
                   FSFile *f, int n, char **names);
    int (*fs_mkdir)(FSDevice *fs, FSQID *qid, FSFile *f,
                    const char *name, uint32_t mode, uint32_t gid);
    int (*fs_open)(FSDevice *fs, FSQID *qid, FSFile *f, uint32_t flags,
                   FSOpenCompletionFunc *cb, void *opaque);
    int (*fs_create)(FSDevice *fs, FSQID *qid, FSFile *f, const char *name,
                     uint32_t flags, uint32_t mode, uint32_t gid);
    int (*fs_stat)(FSDevice *fs, FSFile *f, FSStat *st);
    int (*fs_setattr)(FSDevice *fs, FSFile *f, uint32_t mask,
                      uint32_t mode, uint32_t uid, uint32_t gid,
                      uint64_t size, uint64_t atime_sec, uint64_t atime_nsec,
                      uint64_t mtime_sec, uint64_t mtime_nsec);
    void (*fs_close)(FSDevice *fs, FSFile *f);
    int (*fs_readdir)(FSDevice *fs, FSFile *f, uint64_t offset,
                      uint8_t *buf, int count);
    int (*fs_read)(FSDevice *fs, FSFile *f, uint64_t offset,
            uint8_t *buf, int count);
    int (*fs_write)(FSDevice *fs, FSFile *f, uint64_t offset,
             const uint8_t *buf, int count);
    int (*fs_link)(FSDevice *fs, FSFile *df, FSFile *f, const char *name);
    int (*fs_symlink)(FSDevice *fs, FSQID *qid,
                      FSFile *f, const char *name, const char *symgt, uint32_t gid);
    int (*fs_mknod)(FSDevice *fs, FSQID *qid,
                    FSFile *f, const char *name, uint32_t mode, uint32_t major,
                    uint32_t minor, uint32_t gid);
    int (*fs_readlink)(FSDevice *fs, char *buf, int buf_size, FSFile *f);
    int (*fs_renameat)(FSDevice *fs, FSFile *f, const char *name,
                       FSFile *new_f, const char *new_name);
    int (*fs_unlinkat)(FSDevice *fs, FSFile *f, const char *name);
    int (*fs_lock)(FSDevice *fs, FSFile *f, const FSLock *lock);
    int (*fs_getlock)(FSDevice *fs, FSFile *f, FSLock *lock);
};

FSDevice *fs_disk_init(const char *root_path);
FSDevice *fs_mem_init(void);
FSDevice *fs_net_init(const char *url, void (*start)(void *opaque), void *opaque);
void fs_net_set_pwd(FSDevice *fs, const char *pwd);



void fs_export_file(const char *filename,
                    const uint8_t *buf, int buf_len);
void fs_end(FSDevice *fs);
void fs_dump_cache_load(FSDevice *fs1, const char *filename);

FSFile *fs_dup(FSDevice *fs, FSFile *f);
FSFile *fs_walk_path1(FSDevice *fs, FSFile *f, const char *path,
                      char **pname);
FSFile *fs_walk_path(FSDevice *fs, FSFile *f, const char *path);
typedef enum {
    FS_ERR_OK = 0,
    FS_ERR_GENERIC = -1,
    FS_ERR_SYNTAX = -2,
    FS_ERR_REVISION = -3,
    FS_ERR_FILE_ID = -4,
    FS_ERR_IO = -5,
    FS_ERR_NOENT = -6,
    FS_ERR_COUNTERS = -7,
    FS_ERR_QUOTA = -8,
    FS_ERR_PROTOCOL_VERSION = -9,
    FS_ERR_HEAD = -10,
} FSCommitErrorCode;

typedef uint64_t FSFileID;

static inline BOOL isspace_nolf(int c)
{
    return (c == ' ' || c == '\t');
}

static inline int from_hex(int c)
{
    if (c >= '0' && c <= '9')
        return c - '0';
    else if (c >= 'A' && c <= 'F')
        return c - 'A' + 10;
    else if (c >= 'a' && c <= 'f')
        return c - 'a' + 10;
    else
        return -1;
}

static inline uint64_t block_align(uint64_t val, uint64_t align)
{
    return (val + align - 1) & ~(align - 1);
}

void pstrcpy(char *buf, int buf_size, const char *str);
char *pstrcat(char *buf, int buf_size, const char *s);
char *compose_path(const char *path, const char *name);
char *compose_url(const char *base_url, const char *name);
void skip_line(const char **pp);
char *quoted_str(const char *str);
int parse_fname(char *buf, int buf_size, const char **pp);
int parse_uint32_base(uint32_t *pval, const char **pp, int base);
int parse_uint64_base(uint64_t *pval, const char **pp, int base);
int parse_uint64(uint64_t *pval, const char **pp);
int parse_uint32(uint32_t *pval, const char **pp);
int parse_time(uint32_t *psec, uint32_t *pnsec, const char **pp);
int parse_file_id(FSFileID *pval, const char **pp);
char *file_id_to_filename(char *buf, FSFileID file_id);
void encode_hex(char *str, const uint8_t *buf, int len);
int decode_hex(uint8_t *buf, const char *str, int len);
BOOL is_url(const char *path);

const char *skip_header(const char *p);
int parse_tag(char *buf, int buf_size, const char *str, const char *tag);
int parse_tag_uint64(uint64_t *pval, const char *str, const char *tag);
int parse_tag_file_id(FSFileID *pval, const char *str, const char *tag);
int parse_tag_version(const char *str);
       

       

       

       
       







typedef long int ptrdiff_t;
typedef struct {
  long long __max_align_ll __attribute__((__aligned__(__alignof__(long long))));
  long double __max_align_ld __attribute__((__aligned__(__alignof__(long double))));
} max_align_t;
  typedef __typeof__(nullptr) nullptr_t;
struct aes_key_st {



    unsigned int rd_key[4 * (14 + 1)];

    int rounds;
};
typedef struct aes_key_st AES_KEY;



__attribute__((deprecated("Since OpenSSL " "3.0"))) const char *AES_options(void);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int AES_set_encrypt_key(const unsigned char *userKey, const int bits,
    AES_KEY *key);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int AES_set_decrypt_key(const unsigned char *userKey, const int bits,
    AES_KEY *key);
__attribute__((deprecated("Since OpenSSL " "3.0")))
void AES_encrypt(const unsigned char *in, unsigned char *out,
    const AES_KEY *key);
__attribute__((deprecated("Since OpenSSL " "3.0")))
void AES_decrypt(const unsigned char *in, unsigned char *out,
    const AES_KEY *key);
__attribute__((deprecated("Since OpenSSL " "3.0")))
void AES_ecb_encrypt(const unsigned char *in, unsigned char *out,
    const AES_KEY *key, const int enc);
__attribute__((deprecated("Since OpenSSL " "3.0")))
void AES_cbc_encrypt(const unsigned char *in, unsigned char *out,
    size_t length, const AES_KEY *key,
    unsigned char *ivec, const int enc);
__attribute__((deprecated("Since OpenSSL " "3.0")))
void AES_cfb128_encrypt(const unsigned char *in, unsigned char *out,
    size_t length, const AES_KEY *key,
    unsigned char *ivec, int *num, const int enc);
__attribute__((deprecated("Since OpenSSL " "3.0")))
void AES_cfb1_encrypt(const unsigned char *in, unsigned char *out,
    size_t length, const AES_KEY *key,
    unsigned char *ivec, int *num, const int enc);
__attribute__((deprecated("Since OpenSSL " "3.0")))
void AES_cfb8_encrypt(const unsigned char *in, unsigned char *out,
    size_t length, const AES_KEY *key,
    unsigned char *ivec, int *num, const int enc);
__attribute__((deprecated("Since OpenSSL " "3.0")))
void AES_ofb128_encrypt(const unsigned char *in, unsigned char *out,
    size_t length, const AES_KEY *key,
    unsigned char *ivec, int *num);


__attribute__((deprecated("Since OpenSSL " "3.0")))
void AES_ige_encrypt(const unsigned char *in, unsigned char *out,
    size_t length, const AES_KEY *key,
    unsigned char *ivec, const int enc);

__attribute__((deprecated("Since OpenSSL " "3.0")))
void AES_bi_ige_encrypt(const unsigned char *in, unsigned char *out,
    size_t length, const AES_KEY *key, const AES_KEY *key2,
    const unsigned char *ivec, const int enc);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int AES_wrap_key(AES_KEY *key, const unsigned char *iv,
    unsigned char *out, const unsigned char *in,
    unsigned int inlen);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int AES_unwrap_key(AES_KEY *key, const unsigned char *iv,
    unsigned char *out, const unsigned char *in,
    unsigned int inlen);
       






       
typedef intmax_t ossl_intmax_t;
typedef uintmax_t ossl_uintmax_t;
typedef struct SHAstate_st {
    unsigned int h0, h1, h2, h3, h4;
    unsigned int Nl, Nh;
    unsigned int data[16];
    unsigned int num;
} SHA_CTX;

__attribute__((deprecated("Since OpenSSL " "3.0"))) int SHA1_Init(SHA_CTX *c);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int SHA1_Update(SHA_CTX *c, const void *data, size_t len);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int SHA1_Final(unsigned char *md, SHA_CTX *c);
__attribute__((deprecated("Since OpenSSL " "3.0"))) void SHA1_Transform(SHA_CTX *c, const unsigned char *data);


unsigned char *SHA1(const unsigned char *d, size_t n, unsigned char *md);






typedef struct SHA256state_st {
    unsigned int h[8];
    unsigned int Nl, Nh;
    unsigned int data[16];
    unsigned int num, md_len;
} SHA256_CTX;

__attribute__((deprecated("Since OpenSSL " "3.0"))) int SHA224_Init(SHA256_CTX *c);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int SHA224_Update(SHA256_CTX *c,
    const void *data, size_t len);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int SHA224_Final(unsigned char *md, SHA256_CTX *c);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int SHA256_Init(SHA256_CTX *c);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int SHA256_Update(SHA256_CTX *c,
    const void *data, size_t len);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int SHA256_Final(unsigned char *md, SHA256_CTX *c);
__attribute__((deprecated("Since OpenSSL " "3.0"))) void SHA256_Transform(SHA256_CTX *c,
    const unsigned char *data);


unsigned char *SHA224(const unsigned char *d, size_t n, unsigned char *md);
unsigned char *SHA256(const unsigned char *d, size_t n, unsigned char *md);
typedef struct SHA512state_st {
    unsigned long long h[8];
    unsigned long long Nl, Nh;
    union {
        unsigned long long d[16];
        unsigned char p[(16 * 8)];
    } u;
    unsigned int num, md_len;
} SHA512_CTX;

__attribute__((deprecated("Since OpenSSL " "3.0"))) int SHA384_Init(SHA512_CTX *c);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int SHA384_Update(SHA512_CTX *c,
    const void *data, size_t len);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int SHA384_Final(unsigned char *md, SHA512_CTX *c);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int SHA512_Init(SHA512_CTX *c);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int SHA512_Update(SHA512_CTX *c,
    const void *data, size_t len);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int SHA512_Final(unsigned char *md, SHA512_CTX *c);
__attribute__((deprecated("Since OpenSSL " "3.0"))) void SHA512_Transform(SHA512_CTX *c,
    const unsigned char *data);


unsigned char *SHA384(const unsigned char *d, size_t n, unsigned char *md);
unsigned char *SHA512(const unsigned char *d, size_t n, unsigned char *md);
       







extern long int __sysconf (int __name) __attribute__ ((__nothrow__ , __leaf__));







 






       






       
typedef struct stack_st OPENSSL_STACK;

typedef int (*OPENSSL_sk_compfunc)(const void *, const void *);
typedef void (*OPENSSL_sk_freefunc)(void *);
typedef void (*OPENSSL_sk_freefunc_thunk)(OPENSSL_sk_freefunc, void *);
typedef void *(*OPENSSL_sk_copyfunc)(const void *);

int OPENSSL_sk_num(const OPENSSL_STACK *);
void *OPENSSL_sk_value(const OPENSSL_STACK *, int);

void *OPENSSL_sk_set(OPENSSL_STACK *st, int i, const void *data);

OPENSSL_STACK *OPENSSL_sk_new(OPENSSL_sk_compfunc cmp);
OPENSSL_STACK *OPENSSL_sk_new_null(void);
OPENSSL_STACK *OPENSSL_sk_new_reserve(OPENSSL_sk_compfunc c, int n);
OPENSSL_STACK *OPENSSL_sk_set_thunks(OPENSSL_STACK *st, OPENSSL_sk_freefunc_thunk f_thunk);
int OPENSSL_sk_reserve(OPENSSL_STACK *st, int n);
void OPENSSL_sk_free(OPENSSL_STACK *);
void OPENSSL_sk_pop_free(OPENSSL_STACK *st, OPENSSL_sk_freefunc func);
OPENSSL_STACK *OPENSSL_sk_deep_copy(const OPENSSL_STACK *,
    OPENSSL_sk_copyfunc c,
    OPENSSL_sk_freefunc f);
int OPENSSL_sk_insert(OPENSSL_STACK *sk, const void *data, int where);
void *OPENSSL_sk_delete(OPENSSL_STACK *st, int loc);
void *OPENSSL_sk_delete_ptr(OPENSSL_STACK *st, const void *p);
int OPENSSL_sk_find(OPENSSL_STACK *st, const void *data);
int OPENSSL_sk_find_ex(OPENSSL_STACK *st, const void *data);
int OPENSSL_sk_find_all(OPENSSL_STACK *st, const void *data, int *pnum);
int OPENSSL_sk_push(OPENSSL_STACK *st, const void *data);
int OPENSSL_sk_unshift(OPENSSL_STACK *st, const void *data);
void *OPENSSL_sk_shift(OPENSSL_STACK *st);
void *OPENSSL_sk_pop(OPENSSL_STACK *st);
void OPENSSL_sk_zero(OPENSSL_STACK *st);
OPENSSL_sk_compfunc OPENSSL_sk_set_cmp_func(OPENSSL_STACK *sk,
    OPENSSL_sk_compfunc cmp);
OPENSSL_STACK *OPENSSL_sk_dup(const OPENSSL_STACK *st);
void OPENSSL_sk_sort(OPENSSL_STACK *st);
int OPENSSL_sk_is_sorted(const OPENSSL_STACK *st);
typedef char *OPENSSL_STRING;
typedef const char *OPENSSL_CSTRING;
struct stack_st_OPENSSL_STRING; typedef int (*sk_OPENSSL_STRING_compfunc)(const char *const *a, const char *const *b); typedef void (*sk_OPENSSL_STRING_freefunc)(char * a); typedef char *(*sk_OPENSSL_STRING_copyfunc)(const char *a); static inline void sk_OPENSSL_STRING_freefunc_thunk(OPENSSL_sk_freefunc freefunc_arg, void *ptr) { sk_OPENSSL_STRING_freefunc freefunc = (sk_OPENSSL_STRING_freefunc)freefunc_arg; freefunc((char *)ptr); } static __attribute__((unused)) inline char *ossl_check_OPENSSL_STRING_type(char *ptr) { return ptr; } static __attribute__((unused)) inline const OPENSSL_STACK *ossl_check_const_OPENSSL_STRING_sk_type(const struct stack_st_OPENSSL_STRING *sk) { return (const OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_STACK *ossl_check_OPENSSL_STRING_sk_type(struct stack_st_OPENSSL_STRING *sk) { return (OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_sk_compfunc ossl_check_OPENSSL_STRING_compfunc_type(sk_OPENSSL_STRING_compfunc cmp) { return (OPENSSL_sk_compfunc)cmp; } static __attribute__((unused)) inline OPENSSL_sk_copyfunc ossl_check_OPENSSL_STRING_copyfunc_type(sk_OPENSSL_STRING_copyfunc cpy) { return (OPENSSL_sk_copyfunc)cpy; } static __attribute__((unused)) inline OPENSSL_sk_freefunc ossl_check_OPENSSL_STRING_freefunc_type(sk_OPENSSL_STRING_freefunc fr) { return (OPENSSL_sk_freefunc)fr; }
struct stack_st_OPENSSL_CSTRING; typedef int (*sk_OPENSSL_CSTRING_compfunc)(const char *const *a, const char *const *b); typedef void (*sk_OPENSSL_CSTRING_freefunc)(char * a); typedef char *(*sk_OPENSSL_CSTRING_copyfunc)(const char *a); static inline void sk_OPENSSL_CSTRING_freefunc_thunk(OPENSSL_sk_freefunc freefunc_arg, void *ptr) { sk_OPENSSL_CSTRING_freefunc freefunc = (sk_OPENSSL_CSTRING_freefunc)freefunc_arg; freefunc((char *)ptr); } static __attribute__((unused)) inline const char *ossl_check_OPENSSL_CSTRING_type(const char *ptr) { return ptr; } static __attribute__((unused)) inline const OPENSSL_STACK *ossl_check_const_OPENSSL_CSTRING_sk_type(const struct stack_st_OPENSSL_CSTRING *sk) { return (const OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_STACK *ossl_check_OPENSSL_CSTRING_sk_type(struct stack_st_OPENSSL_CSTRING *sk) { return (OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_sk_compfunc ossl_check_OPENSSL_CSTRING_compfunc_type(sk_OPENSSL_CSTRING_compfunc cmp) { return (OPENSSL_sk_compfunc)cmp; } static __attribute__((unused)) inline OPENSSL_sk_copyfunc ossl_check_OPENSSL_CSTRING_copyfunc_type(sk_OPENSSL_CSTRING_copyfunc cpy) { return (OPENSSL_sk_copyfunc)cpy; } static __attribute__((unused)) inline OPENSSL_sk_freefunc ossl_check_OPENSSL_CSTRING_freefunc_type(sk_OPENSSL_CSTRING_freefunc fr) { return (OPENSSL_sk_freefunc)fr; }
typedef void *OPENSSL_BLOCK;

struct stack_st_OPENSSL_BLOCK; typedef int (*sk_OPENSSL_BLOCK_compfunc)(const void *const *a, const void *const *b); typedef void (*sk_OPENSSL_BLOCK_freefunc)(void * a); typedef void *(*sk_OPENSSL_BLOCK_copyfunc)(const void *a); static inline void sk_OPENSSL_BLOCK_freefunc_thunk(OPENSSL_sk_freefunc freefunc_arg, void *ptr) { sk_OPENSSL_BLOCK_freefunc freefunc = (sk_OPENSSL_BLOCK_freefunc)freefunc_arg; freefunc((void *)ptr); } static __attribute__((unused)) inline void *ossl_check_OPENSSL_BLOCK_type(void *ptr) { return ptr; } static __attribute__((unused)) inline const OPENSSL_STACK *ossl_check_const_OPENSSL_BLOCK_sk_type(const struct stack_st_OPENSSL_BLOCK *sk) { return (const OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_STACK *ossl_check_OPENSSL_BLOCK_sk_type(struct stack_st_OPENSSL_BLOCK *sk) { return (OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_sk_compfunc ossl_check_OPENSSL_BLOCK_compfunc_type(sk_OPENSSL_BLOCK_compfunc cmp) { return (OPENSSL_sk_compfunc)cmp; } static __attribute__((unused)) inline OPENSSL_sk_copyfunc ossl_check_OPENSSL_BLOCK_copyfunc_type(sk_OPENSSL_BLOCK_copyfunc cpy) { return (OPENSSL_sk_copyfunc)cpy; } static __attribute__((unused)) inline OPENSSL_sk_freefunc ossl_check_OPENSSL_BLOCK_freefunc_type(sk_OPENSSL_BLOCK_freefunc fr) { return (OPENSSL_sk_freefunc)fr; }
typedef struct ossl_provider_st OSSL_PROVIDER;
typedef struct asn1_string_st ASN1_INTEGER;
typedef struct asn1_string_st ASN1_ENUMERATED;
typedef struct asn1_string_st ASN1_BIT_STRING;
typedef struct asn1_string_st ASN1_OCTET_STRING;
typedef struct asn1_string_st ASN1_PRINTABLESTRING;
typedef struct asn1_string_st ASN1_T61STRING;
typedef struct asn1_string_st ASN1_IA5STRING;
typedef struct asn1_string_st ASN1_GENERALSTRING;
typedef struct asn1_string_st ASN1_UNIVERSALSTRING;
typedef struct asn1_string_st ASN1_BMPSTRING;
typedef struct asn1_string_st ASN1_UTCTIME;
typedef struct asn1_string_st ASN1_TIME;
typedef struct asn1_string_st ASN1_GENERALIZEDTIME;
typedef struct asn1_string_st ASN1_VISIBLESTRING;
typedef struct asn1_string_st ASN1_UTF8STRING;
typedef struct asn1_string_st ASN1_STRING;
typedef int ASN1_BOOLEAN;
typedef int ASN1_NULL;


typedef struct asn1_type_st ASN1_TYPE;
typedef struct asn1_object_st ASN1_OBJECT;
typedef struct asn1_string_table_st ASN1_STRING_TABLE;

typedef struct ASN1_ITEM_st ASN1_ITEM;
typedef struct asn1_pctx_st ASN1_PCTX;
typedef struct asn1_sctx_st ASN1_SCTX;





typedef struct bio_st BIO;
typedef struct bignum_st BIGNUM;
typedef struct bignum_ctx BN_CTX;
typedef struct bn_blinding_st BN_BLINDING;
typedef struct bn_mont_ctx_st BN_MONT_CTX;
typedef struct bn_recp_ctx_st BN_RECP_CTX;
typedef struct bn_gencb_st BN_GENCB;

typedef struct buf_mem_st BUF_MEM;

struct stack_st_BIGNUM;
struct stack_st_BIGNUM_const;

typedef struct err_state_st ERR_STATE;

typedef struct evp_cipher_st EVP_CIPHER;
typedef struct evp_cipher_ctx_st EVP_CIPHER_CTX;
typedef struct evp_md_st EVP_MD;
typedef struct evp_md_ctx_st EVP_MD_CTX;
typedef struct evp_mac_st EVP_MAC;
typedef struct evp_mac_ctx_st EVP_MAC_CTX;
typedef struct evp_pkey_st EVP_PKEY;
typedef struct evp_skey_st EVP_SKEY;

typedef struct evp_pkey_asn1_method_st EVP_PKEY_ASN1_METHOD;

typedef struct evp_pkey_method_st EVP_PKEY_METHOD;
typedef struct evp_pkey_ctx_st EVP_PKEY_CTX;

typedef struct evp_keymgmt_st EVP_KEYMGMT;

typedef struct evp_kdf_st EVP_KDF;
typedef struct evp_kdf_ctx_st EVP_KDF_CTX;

typedef struct evp_rand_st EVP_RAND;
typedef struct evp_rand_ctx_st EVP_RAND_CTX;

typedef struct evp_keyexch_st EVP_KEYEXCH;

typedef struct evp_signature_st EVP_SIGNATURE;

typedef struct evp_skeymgmt_st EVP_SKEYMGMT;

typedef struct evp_asym_cipher_st EVP_ASYM_CIPHER;

typedef struct evp_kem_st EVP_KEM;

typedef struct evp_Encode_Ctx_st EVP_ENCODE_CTX;

typedef struct hmac_ctx_st HMAC_CTX;

typedef struct dh_st DH;
typedef struct dh_method DH_METHOD;


typedef struct dsa_st DSA;
typedef struct dsa_method DSA_METHOD;



typedef struct rsa_st RSA;
typedef struct rsa_meth_st RSA_METHOD;


typedef struct rsa_pss_params_st RSA_PSS_PARAMS;
typedef struct rsa_oaep_params_st RSA_OAEP_PARAMS;


typedef struct ec_key_st EC_KEY;
typedef struct ec_key_method_st EC_KEY_METHOD;


typedef struct rand_meth_st RAND_METHOD;
typedef struct rand_drbg_st RAND_DRBG;

typedef struct ssl_dane_st SSL_DANE;
typedef struct x509_st X509;
typedef struct X509_algor_st X509_ALGOR;
typedef struct X509_crl_st X509_CRL;
typedef struct x509_crl_method_st X509_CRL_METHOD;
typedef struct x509_revoked_st X509_REVOKED;
typedef struct X509_name_st X509_NAME;
typedef struct X509_pubkey_st X509_PUBKEY;
typedef struct x509_store_st X509_STORE;
typedef struct x509_store_ctx_st X509_STORE_CTX;

typedef struct x509_object_st X509_OBJECT;
typedef struct x509_lookup_st X509_LOOKUP;
typedef struct x509_lookup_method_st X509_LOOKUP_METHOD;
typedef struct X509_VERIFY_PARAM_st X509_VERIFY_PARAM;

typedef struct x509_sig_info_st X509_SIG_INFO;

typedef struct pkcs8_priv_key_info_st PKCS8_PRIV_KEY_INFO;

typedef struct v3_ext_ctx X509V3_CTX;
typedef struct conf_st CONF;
typedef struct ossl_init_settings_st OPENSSL_INIT_SETTINGS;

typedef struct ui_st UI;
typedef struct ui_method_st UI_METHOD;

typedef struct engine_st ENGINE;
typedef struct ssl_st SSL;
typedef struct ssl_ctx_st SSL_CTX;

typedef struct comp_ctx_st COMP_CTX;
typedef struct comp_method_st COMP_METHOD;

typedef struct X509_POLICY_NODE_st X509_POLICY_NODE;
typedef struct X509_POLICY_LEVEL_st X509_POLICY_LEVEL;
typedef struct X509_POLICY_TREE_st X509_POLICY_TREE;
typedef struct X509_POLICY_CACHE_st X509_POLICY_CACHE;

typedef struct AUTHORITY_KEYID_st AUTHORITY_KEYID;
typedef struct DIST_POINT_st DIST_POINT;
typedef struct ISSUING_DIST_POINT_st ISSUING_DIST_POINT;
typedef struct NAME_CONSTRAINTS_st NAME_CONSTRAINTS;

typedef struct crypto_ex_data_st CRYPTO_EX_DATA;

typedef struct ossl_http_req_ctx_st OSSL_HTTP_REQ_CTX;
typedef struct ocsp_response_st OCSP_RESPONSE;
typedef struct ocsp_responder_id_st OCSP_RESPID;

typedef struct sct_st SCT;
typedef struct sct_ctx_st SCT_CTX;
typedef struct ctlog_st CTLOG;
typedef struct ctlog_store_st CTLOG_STORE;
typedef struct ct_policy_eval_ctx_st CT_POLICY_EVAL_CTX;

typedef struct ossl_store_info_st OSSL_STORE_INFO;
typedef struct ossl_store_search_st OSSL_STORE_SEARCH;

typedef struct ossl_lib_ctx_st OSSL_LIB_CTX;

typedef struct ossl_dispatch_st OSSL_DISPATCH;
typedef struct ossl_item_st OSSL_ITEM;
typedef struct ossl_algorithm_st OSSL_ALGORITHM;
typedef struct ossl_param_st OSSL_PARAM;
typedef struct ossl_param_bld_st OSSL_PARAM_BLD;

typedef int pem_password_cb(char *buf, int size, int rwflag, void *userdata);

typedef struct ossl_encoder_st OSSL_ENCODER;
typedef struct ossl_encoder_ctx_st OSSL_ENCODER_CTX;
typedef struct ossl_decoder_st OSSL_DECODER;
typedef struct ossl_decoder_ctx_st OSSL_DECODER_CTX;

typedef struct ossl_self_test_st OSSL_SELF_TEST;
       

typedef struct ossl_core_handle_st OSSL_CORE_HANDLE;
typedef struct openssl_core_ctx_st OPENSSL_CORE_CTX;
typedef struct ossl_core_bio_st OSSL_CORE_BIO;







struct ossl_dispatch_st {
    int function_id;
    void (*function)(void);
};
struct ossl_item_st {
    unsigned int id;
    void *ptr;
};







struct ossl_algorithm_st {
    const char *algorithm_names;
    const char *property_definition;
    const OSSL_DISPATCH *implementation;
    const char *algorithm_description;
};







struct ossl_param_st {
    const char *key;
    unsigned int data_type;
    void *data;
    size_t data_size;
    size_t return_size;
};
typedef void (*OSSL_thread_stop_handler_fn)(void *arg);
typedef int(OSSL_provider_init_fn)(const OSSL_CORE_HANDLE *handle,
    const OSSL_DISPATCH *in,
    const OSSL_DISPATCH **out,
    void **provctx);




extern OSSL_provider_init_fn OSSL_provider_init;
typedef int(OSSL_CALLBACK)(const OSSL_PARAM params[], void *arg);
typedef int(OSSL_INOUT_CALLBACK)(const OSSL_PARAM in_params[],
    OSSL_PARAM out_params[], void *arg);






typedef int(OSSL_PASSPHRASE_CALLBACK)(char *pass, size_t pass_size,
    size_t *pass_len,
    const OSSL_PARAM params[], void *arg);
       



       





       


       
       














struct timex
{
  unsigned int modes;
  __syscall_slong_t offset;
  __syscall_slong_t freq;
  __syscall_slong_t maxerror;
  __syscall_slong_t esterror;
  int status;
  __syscall_slong_t constant;
  __syscall_slong_t precision;
  __syscall_slong_t tolerance;
  struct timeval time;
  __syscall_slong_t tick;
  __syscall_slong_t ppsfreq;
  __syscall_slong_t jitter;
  int shift;
  __syscall_slong_t stabil;
  __syscall_slong_t jitcnt;
  __syscall_slong_t calcnt;
  __syscall_slong_t errcnt;
  __syscall_slong_t stbcnt;

  int tai;


  int :32; int :32; int :32; int :32;
  int :32; int :32; int :32; int :32;
  int :32; int :32; int :32;

};




extern int clock_adjtime (__clockid_t __clock_id, struct timex *__utx) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2)));












struct tm
{
  int tm_sec;
  int tm_min;
  int tm_hour;
  int tm_mday;
  int tm_mon;
  int tm_year;
  int tm_wday;
  int tm_yday;
  int tm_isdst;


  long int tm_gmtoff;
  const char *tm_zone;




};







struct itimerspec
  {
    struct timespec it_interval;
    struct timespec it_value;
  };
struct sigevent;




extern clock_t clock (void) __attribute__ ((__nothrow__ , __leaf__));



extern time_t time (time_t *__timer) __attribute__ ((__nothrow__ , __leaf__));


extern double difftime (time_t __time1, time_t __time0);


extern time_t mktime (struct tm *__tp) __attribute__ ((__nothrow__ , __leaf__));
extern size_t strftime (char *__restrict __s, size_t __maxsize,
   const char *__restrict __format,
   const struct tm *__restrict __tp)
   __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 3, 4)));




extern char *strptime (const char *__restrict __s,
         const char *__restrict __fmt, struct tm *__tp)
     __attribute__ ((__nothrow__ , __leaf__));






extern size_t strftime_l (char *__restrict __s, size_t __maxsize,
     const char *__restrict __format,
     const struct tm *__restrict __tp,
     locale_t __loc) __attribute__ ((__nothrow__ , __leaf__));



extern char *strptime_l (const char *__restrict __s,
    const char *__restrict __fmt, struct tm *__tp,
    locale_t __loc) __attribute__ ((__nothrow__ , __leaf__));






extern struct tm *gmtime (const time_t *__timer) __attribute__ ((__nothrow__ , __leaf__));



extern struct tm *localtime (const time_t *__timer) __attribute__ ((__nothrow__ , __leaf__));
extern struct tm *gmtime_r (const time_t *__restrict __timer,
       struct tm *__restrict __tp) __attribute__ ((__nothrow__ , __leaf__));



extern struct tm *localtime_r (const time_t *__restrict __timer,
          struct tm *__restrict __tp) __attribute__ ((__nothrow__ , __leaf__));
extern char *asctime (const struct tm *__tp) __attribute__ ((__nothrow__ , __leaf__));



extern char *ctime (const time_t *__timer) __attribute__ ((__nothrow__ , __leaf__));
extern char *asctime_r (const struct tm *__restrict __tp,
   char *__restrict __buf) __attribute__ ((__nothrow__ , __leaf__));



extern char *ctime_r (const time_t *__restrict __timer,
        char *__restrict __buf) __attribute__ ((__nothrow__ , __leaf__));
extern char *__tzname[2];
extern int __daylight;
extern long int __timezone;




extern char *tzname[2];



extern void tzset (void) __attribute__ ((__nothrow__ , __leaf__));



extern int daylight;
extern long int timezone;
extern time_t timegm (struct tm *__tp) __attribute__ ((__nothrow__ , __leaf__));
extern time_t timelocal (struct tm *__tp) __attribute__ ((__nothrow__ , __leaf__));







extern int dysize (int __year) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__));
extern int nanosleep (const struct timespec *__requested_time,
        struct timespec *__remaining);


extern int clock_getres (clockid_t __clock_id, struct timespec *__res) __attribute__ ((__nothrow__ , __leaf__));


extern int clock_gettime (clockid_t __clock_id, struct timespec *__tp)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2)));


extern int clock_settime (clockid_t __clock_id, const struct timespec *__tp)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2)));
extern int clock_nanosleep (clockid_t __clock_id, int __flags,
       const struct timespec *__req,
       struct timespec *__rem);
extern int clock_getcpuclockid (pid_t __pid, clockid_t *__clock_id) __attribute__ ((__nothrow__ , __leaf__));




extern int timer_create (clockid_t __clock_id,
    struct sigevent *__restrict __evp,
    timer_t *__restrict __timerid) __attribute__ ((__nothrow__ , __leaf__));


extern int timer_delete (timer_t __timerid) __attribute__ ((__nothrow__ , __leaf__));



extern int timer_settime (timer_t __timerid, int __flags,
     const struct itimerspec *__restrict __value,
     struct itimerspec *__restrict __ovalue) __attribute__ ((__nothrow__ , __leaf__));


extern int timer_gettime (timer_t __timerid, struct itimerspec *__value)
     __attribute__ ((__nothrow__ , __leaf__));
extern int timer_getoverrun (timer_t __timerid) __attribute__ ((__nothrow__ , __leaf__));






extern int timespec_get (struct timespec *__ts, int __base)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));
extern int timespec_getres (struct timespec *__ts, int __base)
     __attribute__ ((__nothrow__ , __leaf__));
extern int getdate_err;
extern struct tm *getdate (const char *__string);
extern int getdate_r (const char *__restrict __string,
        struct tm *__restrict __resbufp);




       


       
       
__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_ASN1_strings(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_ASYNC_strings(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_BIO_strings(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_BN_strings(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_BUF_strings(void);

__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_CMS_strings(void);


__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_COMP_strings(void);

__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_CONF_strings(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_CRYPTO_strings(void);

__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_CT_strings(void);


__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_DH_strings(void);


__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_DSA_strings(void);


__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_EC_strings(void);


__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_ENGINE_strings(void);

__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_ERR_strings(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_EVP_strings(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_KDF_strings(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_OBJ_strings(void);

__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_OCSP_strings(void);

__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_PEM_strings(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_PKCS12_strings(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_PKCS7_strings(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_RAND_strings(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_RSA_strings(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_OSSL_STORE_strings(void);

__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_TS_strings(void);

__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_UI_strings(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_X509_strings(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int ERR_load_X509V3_strings(void);
typedef struct {
    int dummy;
} CRYPTO_dynlock;



typedef void CRYPTO_RWLOCK;

CRYPTO_RWLOCK *CRYPTO_THREAD_lock_new(void);
 int CRYPTO_THREAD_read_lock(CRYPTO_RWLOCK *lock);
 int CRYPTO_THREAD_write_lock(CRYPTO_RWLOCK *lock);
int CRYPTO_THREAD_unlock(CRYPTO_RWLOCK *lock);
void CRYPTO_THREAD_lock_free(CRYPTO_RWLOCK *lock);

int CRYPTO_atomic_add(int *val, int amount, int *ret, CRYPTO_RWLOCK *lock);
int CRYPTO_atomic_add64(uint64_t *val, uint64_t op, uint64_t *ret,
    CRYPTO_RWLOCK *lock);
int CRYPTO_atomic_and(uint64_t *val, uint64_t op, uint64_t *ret,
    CRYPTO_RWLOCK *lock);
int CRYPTO_atomic_or(uint64_t *val, uint64_t op, uint64_t *ret,
    CRYPTO_RWLOCK *lock);
int CRYPTO_atomic_load(uint64_t *val, uint64_t *ret, CRYPTO_RWLOCK *lock);
int CRYPTO_atomic_load_int(int *val, int *ret, CRYPTO_RWLOCK *lock);
int CRYPTO_atomic_store(uint64_t *dst, uint64_t val, CRYPTO_RWLOCK *lock);
size_t OPENSSL_strlcpy(char *dst, const char *src, size_t siz);
size_t OPENSSL_strlcat(char *dst, const char *src, size_t siz);
size_t OPENSSL_strnlen(const char *str, size_t maxlen);
int OPENSSL_strtoul(const char *str, char **endptr, int base, unsigned long *num);
int OPENSSL_buf2hexstr_ex(char *str, size_t str_n, size_t *strlength,
    const unsigned char *buf, size_t buflen,
    const char sep);
char *OPENSSL_buf2hexstr(const unsigned char *buf, long buflen);
int OPENSSL_hexstr2buf_ex(unsigned char *buf, size_t buf_n, size_t *buflen,
    const char *str, const char sep);
unsigned char *OPENSSL_hexstr2buf(const char *str, long *buflen);
int OPENSSL_hexchar2int(unsigned char c);
int OPENSSL_strcasecmp(const char *s1, const char *s2);
int OPENSSL_strncasecmp(const char *s1, const char *s2, size_t n);
unsigned int OPENSSL_version_major(void);
unsigned int OPENSSL_version_minor(void);
unsigned int OPENSSL_version_patch(void);
const char *OPENSSL_version_pre_release(void);
const char *OPENSSL_version_build_metadata(void);

unsigned long OpenSSL_version_num(void);
const char *OpenSSL_version(int type);
const char *OPENSSL_info(int type);
int OPENSSL_issetugid(void);

struct crypto_ex_data_st {
    OSSL_LIB_CTX *ctx;
    struct stack_st_void *sk;
};


struct stack_st_void; typedef int (*sk_void_compfunc)(const void *const *a, const void *const *b); typedef void (*sk_void_freefunc)(void * a); typedef void *(*sk_void_copyfunc)(const void *a); static inline void sk_void_freefunc_thunk(OPENSSL_sk_freefunc freefunc_arg, void *ptr) { sk_void_freefunc freefunc = (sk_void_freefunc)freefunc_arg; freefunc((void *)ptr); } static __attribute__((unused)) inline void *ossl_check_void_type(void *ptr) { return ptr; } static __attribute__((unused)) inline const OPENSSL_STACK *ossl_check_const_void_sk_type(const struct stack_st_void *sk) { return (const OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_STACK *ossl_check_void_sk_type(struct stack_st_void *sk) { return (OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_sk_compfunc ossl_check_void_compfunc_type(sk_void_compfunc cmp) { return (OPENSSL_sk_compfunc)cmp; } static __attribute__((unused)) inline OPENSSL_sk_copyfunc ossl_check_void_copyfunc_type(sk_void_copyfunc cpy) { return (OPENSSL_sk_copyfunc)cpy; } static __attribute__((unused)) inline OPENSSL_sk_freefunc ossl_check_void_freefunc_type(sk_void_freefunc fr) { return (OPENSSL_sk_freefunc)fr; }
typedef void CRYPTO_EX_new(void *parent, void *ptr, CRYPTO_EX_DATA *ad,
    int idx, long argl, void *argp);
typedef void CRYPTO_EX_free(void *parent, void *ptr, CRYPTO_EX_DATA *ad,
    int idx, long argl, void *argp);
typedef int CRYPTO_EX_dup(CRYPTO_EX_DATA *to, const CRYPTO_EX_DATA *from,
    void **from_d, int idx, long argl, void *argp);
 int CRYPTO_get_ex_new_index(int class_index, long argl, void *argp,
    CRYPTO_EX_new *new_func,
    CRYPTO_EX_dup *dup_func,
    CRYPTO_EX_free *free_func);

int CRYPTO_free_ex_index(int class_index, int idx);





int CRYPTO_new_ex_data(int class_index, void *obj, CRYPTO_EX_DATA *ad);
int CRYPTO_dup_ex_data(int class_index, CRYPTO_EX_DATA *to,
    const CRYPTO_EX_DATA *from);

void CRYPTO_free_ex_data(int class_index, void *obj, CRYPTO_EX_DATA *ad);


int CRYPTO_alloc_ex_data(int class_index, void *obj, CRYPTO_EX_DATA *ad,
    int idx);





int CRYPTO_set_ex_data(CRYPTO_EX_DATA *ad, int idx, void *val);
void *CRYPTO_get_ex_data(const CRYPTO_EX_DATA *ad, int idx);
typedef struct crypto_threadid_st {
    int dummy;
} CRYPTO_THREADID;
typedef void *(*CRYPTO_malloc_fn)(size_t num, const char *file, int line);
typedef void *(*CRYPTO_realloc_fn)(void *addr, size_t num, const char *file,
    int line);
typedef void (*CRYPTO_free_fn)(void *addr, const char *file, int line);
int CRYPTO_set_mem_functions(CRYPTO_malloc_fn malloc_fn,
    CRYPTO_realloc_fn realloc_fn,
    CRYPTO_free_fn free_fn);
void CRYPTO_get_mem_functions(CRYPTO_malloc_fn *malloc_fn,
    CRYPTO_realloc_fn *realloc_fn,
    CRYPTO_free_fn *free_fn);

__attribute__((__malloc__)) void *CRYPTO_malloc(size_t num, const char *file, int line);
__attribute__((__malloc__)) void *CRYPTO_zalloc(size_t num, const char *file, int line);
__attribute__((__malloc__)) void *CRYPTO_malloc_array(size_t num, size_t size,
    const char *file, int line);
__attribute__((__malloc__)) void *CRYPTO_calloc(size_t num, size_t size,
    const char *file, int line);
__attribute__((__malloc__)) void *CRYPTO_aligned_alloc(size_t num, size_t align,
    void **freeptr, const char *file,
    int line);
__attribute__((__malloc__)) void *CRYPTO_aligned_alloc_array(size_t num, size_t size,
    size_t align, void **freeptr,
    const char *file, int line);
void *CRYPTO_memdup(const void *str, size_t siz, const char *file, int line);
char *CRYPTO_strdup(const char *str, const char *file, int line);
char *CRYPTO_strndup(const char *str, size_t s, const char *file, int line);
void CRYPTO_free(void *ptr, const char *file, int line);
void CRYPTO_clear_free(void *ptr, size_t num, const char *file, int line);
void *CRYPTO_realloc(void *addr, size_t num, const char *file, int line);
void *CRYPTO_clear_realloc(void *addr, size_t old_num, size_t num,
    const char *file, int line);
void *CRYPTO_realloc_array(void *addr, size_t num, size_t size,
    const char *file, int line);
void *CRYPTO_clear_realloc_array(void *addr, size_t old_num, size_t num,
    size_t size, const char *file, int line);

int CRYPTO_secure_malloc_init(size_t sz, size_t minsize);
int CRYPTO_secure_malloc_done(void);
__attribute__((__malloc__)) void *CRYPTO_secure_malloc(size_t num, const char *file, int line);
__attribute__((__malloc__)) void *CRYPTO_secure_zalloc(size_t num, const char *file, int line);
__attribute__((__malloc__)) void *CRYPTO_secure_malloc_array(size_t num, size_t size,
    const char *file, int line);
__attribute__((__malloc__)) void *CRYPTO_secure_calloc(size_t num, size_t size,
    const char *file, int line);
void CRYPTO_secure_free(void *ptr, const char *file, int line);
void CRYPTO_secure_clear_free(void *ptr, size_t num,
    const char *file, int line);
int CRYPTO_secure_allocated(const void *ptr);
int CRYPTO_secure_malloc_initialized(void);
size_t CRYPTO_secure_actual_size(void *ptr);
size_t CRYPTO_secure_used(void);

void OPENSSL_cleanse(void *ptr, size_t len);
_Noreturn void OPENSSL_die(const char *assertion, const char *file, int line);






int OPENSSL_isservice(void);

void OPENSSL_init(void);


__attribute__((deprecated("Since OpenSSL " "3.0"))) void OPENSSL_fork_prepare(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) void OPENSSL_fork_parent(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) void OPENSSL_fork_child(void);



struct tm *OPENSSL_gmtime(const time_t *timer, struct tm *result);
int OPENSSL_gmtime_adj(struct tm *tm, int offset_day, long offset_sec);
int OPENSSL_gmtime_diff(int *pday, int *psec,
    const struct tm *from, const struct tm *to);
int CRYPTO_memcmp(const void *in_a, const void *in_b, size_t len);
void OPENSSL_cleanup(void);
int OPENSSL_init_crypto(uint64_t opts, const OPENSSL_INIT_SETTINGS *settings);
int OPENSSL_atexit(void (*handler)(void));
void OPENSSL_thread_stop(void);
void OPENSSL_thread_stop_ex(OSSL_LIB_CTX *ctx);


OPENSSL_INIT_SETTINGS *OPENSSL_INIT_new(void);

int OPENSSL_INIT_set_config_filename(OPENSSL_INIT_SETTINGS *settings,
    const char *config_filename);
void OPENSSL_INIT_set_config_file_flags(OPENSSL_INIT_SETTINGS *settings,
    unsigned long flags);
int OPENSSL_INIT_set_config_appname(OPENSSL_INIT_SETTINGS *settings,
    const char *config_appname);

void OPENSSL_INIT_free(OPENSSL_INIT_SETTINGS *settings);





















typedef __signed__ char __s8;
typedef unsigned char __u8;

typedef __signed__ short __s16;
typedef unsigned short __u16;

typedef __signed__ int __s32;
typedef unsigned int __u32;


__extension__ typedef __signed__ long long __s64;
__extension__ typedef unsigned long long __u64;







typedef struct {
 unsigned long fds_bits[1024 / (8 * sizeof(long))];
} __kernel_fd_set;


typedef void (*__kernel_sighandler_t)(int);


typedef int __kernel_key_t;
typedef int __kernel_mqd_t;







typedef unsigned short __kernel_old_uid_t;
typedef unsigned short __kernel_old_gid_t;


typedef unsigned long __kernel_old_dev_t;


typedef long __kernel_long_t;
typedef unsigned long __kernel_ulong_t;



typedef __kernel_ulong_t __kernel_ino_t;



typedef unsigned int __kernel_mode_t;



typedef int __kernel_pid_t;



typedef int __kernel_ipc_pid_t;



typedef unsigned int __kernel_uid_t;
typedef unsigned int __kernel_gid_t;



typedef __kernel_long_t __kernel_suseconds_t;



typedef int __kernel_daddr_t;



typedef unsigned int __kernel_uid32_t;
typedef unsigned int __kernel_gid32_t;
typedef __kernel_ulong_t __kernel_size_t;
typedef __kernel_long_t __kernel_ssize_t;
typedef __kernel_long_t __kernel_ptrdiff_t;




typedef struct {
 int val[2];
} __kernel_fsid_t;





typedef __kernel_long_t __kernel_off_t;
typedef long long __kernel_loff_t;
typedef unsigned long long __kernel_uoff_t;
typedef __kernel_long_t __kernel_old_time_t;
typedef __kernel_long_t __kernel_time_t;
typedef long long __kernel_time64_t;
typedef __kernel_long_t __kernel_clock_t;
typedef int __kernel_timer_t;
typedef int __kernel_clockid_t;
typedef char * __kernel_caddr_t;
typedef unsigned short __kernel_uid16_t;
typedef unsigned short __kernel_gid16_t;


typedef __signed__ __int128 __s128 __attribute__((aligned(16)));
typedef unsigned __int128 __u128 __attribute__((aligned(16)));
typedef __u16 __le16;
typedef __u16 __be16;
typedef __u32 __le32;
typedef __u32 __be32;
typedef __u64 __le64;
typedef __u64 __be64;

typedef __u16 __sum16;
typedef __u32 __wsum;
typedef unsigned __poll_t;
struct sched_attr {
 __u32 size;

 __u32 sched_policy;
 __u64 sched_flags;


 __s32 sched_nice;


 __u32 sched_priority;


 __u64 sched_runtime;
 __u64 sched_deadline;
 __u64 sched_period;


 __u32 sched_util_min;
 __u32 sched_util_max;

};
struct sched_param
{
  int sched_priority;
};





extern int clone (int (*__fn) (void *__arg), void *__child_stack,
    int __flags, void *__arg, ...) __attribute__ ((__nothrow__ , __leaf__));


extern int unshare (int __flags) __attribute__ ((__nothrow__ , __leaf__));


extern int sched_getcpu (void) __attribute__ ((__nothrow__ , __leaf__));


extern int getcpu (unsigned int *, unsigned int *) __attribute__ ((__nothrow__ , __leaf__));


extern int setns (int __fd, int __nstype) __attribute__ ((__nothrow__ , __leaf__));


int sched_setattr (pid_t tid, struct sched_attr *attr, unsigned int flags)
  __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2)));



int sched_getattr (pid_t tid, struct sched_attr *attr, unsigned int size,
     unsigned int flags)
  __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2)));




typedef unsigned long int __cpu_mask;






typedef struct
{
  __cpu_mask __bits[1024 / (8 * sizeof (__cpu_mask))];
} cpu_set_t;


extern int __sched_cpucount (size_t __setsize, const cpu_set_t *__setp)
     __attribute__ ((__nothrow__ , __leaf__));
extern cpu_set_t *__sched_cpualloc (size_t __count) __attribute__ ((__nothrow__ , __leaf__)) ;
extern void __sched_cpufree (cpu_set_t *__set) __attribute__ ((__nothrow__ , __leaf__));











extern int sched_setparam (__pid_t __pid, const struct sched_param *__param)
     __attribute__ ((__nothrow__ , __leaf__));


extern int sched_getparam (__pid_t __pid, struct sched_param *__param) __attribute__ ((__nothrow__ , __leaf__));


extern int sched_setscheduler (__pid_t __pid, int __policy,
          const struct sched_param *__param) __attribute__ ((__nothrow__ , __leaf__));


extern int sched_getscheduler (__pid_t __pid) __attribute__ ((__nothrow__ , __leaf__));


extern int sched_yield (void) __attribute__ ((__nothrow__ , __leaf__));


extern int sched_get_priority_max (int __algorithm) __attribute__ ((__nothrow__ , __leaf__));


extern int sched_get_priority_min (int __algorithm) __attribute__ ((__nothrow__ , __leaf__));



extern int sched_rr_get_interval (__pid_t __pid, struct timespec *__t) __attribute__ ((__nothrow__ , __leaf__));
extern int sched_setaffinity (__pid_t __pid, size_t __cpusetsize,
         const cpu_set_t *__cpuset) __attribute__ ((__nothrow__ , __leaf__));


extern int sched_getaffinity (__pid_t __pid, size_t __cpusetsize,
         cpu_set_t *__cpuset) __attribute__ ((__nothrow__ , __leaf__));











typedef long int __jmp_buf[8];


struct __jmp_buf_tag
  {




    __jmp_buf __jmpbuf;
    int __mask_was_saved;
    __sigset_t __saved_mask;
  };





enum
{
  PTHREAD_CREATE_JOINABLE,

  PTHREAD_CREATE_DETACHED

};



enum
{
  PTHREAD_MUTEX_TIMED_NP,
  PTHREAD_MUTEX_RECURSIVE_NP,
  PTHREAD_MUTEX_ERRORCHECK_NP,
  PTHREAD_MUTEX_ADAPTIVE_NP

  ,
  PTHREAD_MUTEX_NORMAL = PTHREAD_MUTEX_TIMED_NP,
  PTHREAD_MUTEX_RECURSIVE = PTHREAD_MUTEX_RECURSIVE_NP,
  PTHREAD_MUTEX_ERRORCHECK = PTHREAD_MUTEX_ERRORCHECK_NP,
  PTHREAD_MUTEX_DEFAULT = PTHREAD_MUTEX_NORMAL



  , PTHREAD_MUTEX_FAST_NP = PTHREAD_MUTEX_TIMED_NP

};




enum
{
  PTHREAD_MUTEX_STALLED,
  PTHREAD_MUTEX_STALLED_NP = PTHREAD_MUTEX_STALLED,
  PTHREAD_MUTEX_ROBUST,
  PTHREAD_MUTEX_ROBUST_NP = PTHREAD_MUTEX_ROBUST
};





enum
{
  PTHREAD_PRIO_NONE,
  PTHREAD_PRIO_INHERIT,
  PTHREAD_PRIO_PROTECT
};
enum
{
  PTHREAD_RWLOCK_PREFER_READER_NP,
  PTHREAD_RWLOCK_PREFER_WRITER_NP,
  PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP,
  PTHREAD_RWLOCK_DEFAULT_NP = PTHREAD_RWLOCK_PREFER_READER_NP
};
enum
{
  PTHREAD_INHERIT_SCHED,

  PTHREAD_EXPLICIT_SCHED

};



enum
{
  PTHREAD_SCOPE_SYSTEM,

  PTHREAD_SCOPE_PROCESS

};



enum
{
  PTHREAD_PROCESS_PRIVATE,

  PTHREAD_PROCESS_SHARED

};
struct _pthread_cleanup_buffer
{
  void (*__routine) (void *);
  void *__arg;
  int __canceltype;
  struct _pthread_cleanup_buffer *__prev;
};


enum
{
  PTHREAD_CANCEL_ENABLE,

  PTHREAD_CANCEL_DISABLE

};
enum
{
  PTHREAD_CANCEL_DEFERRED,

  PTHREAD_CANCEL_ASYNCHRONOUS

};





extern int pthread_create (pthread_t *__restrict __newthread,
      const pthread_attr_t *__restrict __attr,
      void *(*__start_routine) (void *),
      void *__restrict __arg) __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1, 3)));





extern void pthread_exit (void *__retval) __attribute__ ((__noreturn__));







extern int pthread_join (pthread_t __th, void **__thread_return);




extern int pthread_tryjoin_np (pthread_t __th, void **__thread_return) __attribute__ ((__nothrow__ , __leaf__));
extern int pthread_timedjoin_np (pthread_t __th, void **__thread_return,
     const struct timespec *__abstime);
extern int pthread_clockjoin_np (pthread_t __th, void **__thread_return,
                                 clockid_t __clockid,
     const struct timespec *__abstime);
extern int pthread_detach (pthread_t __th) __attribute__ ((__nothrow__ , __leaf__));



extern pthread_t pthread_self (void) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__));


extern int pthread_equal (pthread_t __thread1, pthread_t __thread2)
  __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__const__));







extern int pthread_attr_init (pthread_attr_t *__attr) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_attr_destroy (pthread_attr_t *__attr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_attr_getdetachstate (const pthread_attr_t *__attr,
     int *__detachstate)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int pthread_attr_setdetachstate (pthread_attr_t *__attr,
     int __detachstate)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



extern int pthread_attr_getguardsize (const pthread_attr_t *__attr,
          size_t *__guardsize)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int pthread_attr_setguardsize (pthread_attr_t *__attr,
          size_t __guardsize)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



extern int pthread_attr_getschedparam (const pthread_attr_t *__restrict __attr,
           struct sched_param *__restrict __param)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int pthread_attr_setschedparam (pthread_attr_t *__restrict __attr,
           const struct sched_param *__restrict
           __param) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int pthread_attr_getschedpolicy (const pthread_attr_t *__restrict
     __attr, int *__restrict __policy)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int pthread_attr_setschedpolicy (pthread_attr_t *__attr, int __policy)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_attr_getinheritsched (const pthread_attr_t *__restrict
      __attr, int *__restrict __inherit)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int pthread_attr_setinheritsched (pthread_attr_t *__attr,
      int __inherit)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



extern int pthread_attr_getscope (const pthread_attr_t *__restrict __attr,
      int *__restrict __scope)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int pthread_attr_setscope (pthread_attr_t *__attr, int __scope)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_attr_getstackaddr (const pthread_attr_t *__restrict
          __attr, void **__restrict __stackaddr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2))) __attribute__ ((__deprecated__));





extern int pthread_attr_setstackaddr (pthread_attr_t *__attr,
          void *__stackaddr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1))) __attribute__ ((__deprecated__));


extern int pthread_attr_getstacksize (const pthread_attr_t *__restrict
          __attr, size_t *__restrict __stacksize)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));




extern int pthread_attr_setstacksize (pthread_attr_t *__attr,
          size_t __stacksize)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



extern int pthread_attr_getstack (const pthread_attr_t *__restrict __attr,
      void **__restrict __stackaddr,
      size_t *__restrict __stacksize)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2, 3)));




extern int pthread_attr_setstack (pthread_attr_t *__attr, void *__stackaddr,
      size_t __stacksize) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));





extern int pthread_attr_setaffinity_np (pthread_attr_t *__attr,
     size_t __cpusetsize,
     const cpu_set_t *__cpuset)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 3)));



extern int pthread_attr_getaffinity_np (const pthread_attr_t *__attr,
     size_t __cpusetsize,
     cpu_set_t *__cpuset)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 3)));


extern int pthread_getattr_default_np (pthread_attr_t *__attr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_attr_setsigmask_np (pthread_attr_t *__attr,
           const __sigset_t *sigmask);




extern int pthread_attr_getsigmask_np (const pthread_attr_t *__attr,
           __sigset_t *sigmask);







extern int pthread_setattr_default_np (const pthread_attr_t *__attr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));




extern int pthread_getattr_np (pthread_t __th, pthread_attr_t *__attr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2)));







extern int pthread_setschedparam (pthread_t __target_thread, int __policy,
      const struct sched_param *__param)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3)));


extern int pthread_getschedparam (pthread_t __target_thread,
      int *__restrict __policy,
      struct sched_param *__restrict __param)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2, 3)));


extern int pthread_setschedprio (pthread_t __target_thread, int __prio)
     __attribute__ ((__nothrow__ , __leaf__));




extern int pthread_getname_np (pthread_t __target_thread, char *__buf,
          size_t __buflen)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2)));


extern int pthread_setname_np (pthread_t __target_thread, const char *__name)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2)));





extern int pthread_getconcurrency (void) __attribute__ ((__nothrow__ , __leaf__));


extern int pthread_setconcurrency (int __level) __attribute__ ((__nothrow__ , __leaf__));



extern int pthread_yield (void) __attribute__ ((__nothrow__ , __leaf__));

extern int pthread_yield (void) __asm__ ("" "sched_yield") __attribute__ ((__nothrow__ , __leaf__))
  __attribute__ ((__deprecated__ ("pthread_yield is deprecated, use sched_yield instead")))
                                                      ;







extern int pthread_setaffinity_np (pthread_t __th, size_t __cpusetsize,
       const cpu_set_t *__cpuset)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3)));


extern int pthread_getaffinity_np (pthread_t __th, size_t __cpusetsize,
       cpu_set_t *__cpuset)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (3)));
extern int pthread_once (pthread_once_t *__once_control,
    void (*__init_routine) (void)) __attribute__ ((__nonnull__ (1, 2)));
extern int pthread_setcancelstate (int __state, int *__oldstate);



extern int pthread_setcanceltype (int __type, int *__oldtype);


extern int pthread_cancel (pthread_t __th);




extern void pthread_testcancel (void);




struct __cancel_jmp_buf_tag
{
  __jmp_buf __cancel_jmp_buf;
  int __mask_was_saved;
};

typedef struct
{
  struct __cancel_jmp_buf_tag __cancel_jmp_buf[1];
  void *__pad[4];
} __pthread_unwind_buf_t __attribute__ ((__aligned__));
struct __pthread_cleanup_frame
{
  void (*__cancel_routine) (void *);
  void *__cancel_arg;
  int __do_it;
  int __cancel_type;
};
extern void __pthread_register_cancel (__pthread_unwind_buf_t *__buf)
     ;
extern void __pthread_unregister_cancel (__pthread_unwind_buf_t *__buf)
  ;
extern void __pthread_register_cancel_defer (__pthread_unwind_buf_t *__buf)
     ;
extern void __pthread_unregister_cancel_restore (__pthread_unwind_buf_t *__buf)
  ;



extern void __pthread_unwind_next (__pthread_unwind_buf_t *__buf)
     __attribute__ ((__noreturn__))

     __attribute__ ((__weak__))

     ;
extern int __sigsetjmp_cancel (struct __cancel_jmp_buf_tag __env[1], int __savemask) __asm__ ("" "__sigsetjmp") __attribute__ ((__nothrow__))


                     __attribute__ ((__returns_twice__));
extern int pthread_mutex_init (pthread_mutex_t *__mutex,
          const pthread_mutexattr_t *__mutexattr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_mutex_destroy (pthread_mutex_t *__mutex)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_mutex_trylock (pthread_mutex_t *__mutex)
     __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_mutex_lock (pthread_mutex_t *__mutex)
     __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1)));




extern int pthread_mutex_timedlock (pthread_mutex_t *__restrict __mutex,
        const struct timespec *__restrict
        __abstime) __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1, 2)));
extern int pthread_mutex_clocklock (pthread_mutex_t *__restrict __mutex,
        clockid_t __clockid,
        const struct timespec *__restrict
        __abstime) __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1, 3)));
extern int pthread_mutex_unlock (pthread_mutex_t *__mutex)
     __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1)));



extern int pthread_mutex_getprioceiling (const pthread_mutex_t *
      __restrict __mutex,
      int *__restrict __prioceiling)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));



extern int pthread_mutex_setprioceiling (pthread_mutex_t *__restrict __mutex,
      int __prioceiling,
      int *__restrict __old_ceiling)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 3)));




extern int pthread_mutex_consistent (pthread_mutex_t *__mutex)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_mutex_consistent_np (pthread_mutex_t *) __asm__ ("" "pthread_mutex_consistent") __attribute__ ((__nothrow__ , __leaf__))
                                __attribute__ ((__nonnull__ (1)))
  __attribute__ ((__deprecated__ ("pthread_mutex_consistent_np is deprecated, use pthread_mutex_consistent")))
                                                                         ;
extern int pthread_mutexattr_init (pthread_mutexattr_t *__attr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_mutexattr_destroy (pthread_mutexattr_t *__attr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_mutexattr_getpshared (const pthread_mutexattr_t *
      __restrict __attr,
      int *__restrict __pshared)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int pthread_mutexattr_setpshared (pthread_mutexattr_t *__attr,
      int __pshared)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



extern int pthread_mutexattr_gettype (const pthread_mutexattr_t *__restrict
          __attr, int *__restrict __kind)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));




extern int pthread_mutexattr_settype (pthread_mutexattr_t *__attr, int __kind)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



extern int pthread_mutexattr_getprotocol (const pthread_mutexattr_t *
       __restrict __attr,
       int *__restrict __protocol)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));



extern int pthread_mutexattr_setprotocol (pthread_mutexattr_t *__attr,
       int __protocol)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_mutexattr_getprioceiling (const pthread_mutexattr_t *
          __restrict __attr,
          int *__restrict __prioceiling)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int pthread_mutexattr_setprioceiling (pthread_mutexattr_t *__attr,
          int __prioceiling)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



extern int pthread_mutexattr_getrobust (const pthread_mutexattr_t *__attr,
     int *__robustness)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int pthread_mutexattr_getrobust_np (pthread_mutexattr_t *, int *) __asm__ ("" "pthread_mutexattr_getrobust") __attribute__ ((__nothrow__ , __leaf__))

                                   __attribute__ ((__nonnull__ (1)))
  __attribute__ ((__deprecated__ ("pthread_mutexattr_getrobust_np is deprecated, use pthread_mutexattr_getrobust")))
                                                                               ;






extern int pthread_mutexattr_setrobust (pthread_mutexattr_t *__attr,
     int __robustness)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_mutexattr_setrobust_np (pthread_mutexattr_t *, int) __asm__ ("" "pthread_mutexattr_setrobust") __attribute__ ((__nothrow__ , __leaf__))

                                   __attribute__ ((__nonnull__ (1)))
  __attribute__ ((__deprecated__ ("pthread_mutexattr_setrobust_np is deprecated, use pthread_mutexattr_setrobust")))
                                                                               ;
extern int pthread_rwlock_init (pthread_rwlock_t *__restrict __rwlock,
    const pthread_rwlockattr_t *__restrict
    __attr) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_rwlock_destroy (pthread_rwlock_t *__rwlock)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_rwlock_rdlock (pthread_rwlock_t *__rwlock)
     __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_rwlock_tryrdlock (pthread_rwlock_t *__rwlock)
  __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1)));




extern int pthread_rwlock_timedrdlock (pthread_rwlock_t *__restrict __rwlock,
           const struct timespec *__restrict
           __abstime) __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1, 2)));
extern int pthread_rwlock_clockrdlock (pthread_rwlock_t *__restrict __rwlock,
           clockid_t __clockid,
           const struct timespec *__restrict
           __abstime) __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1, 3)));
extern int pthread_rwlock_wrlock (pthread_rwlock_t *__rwlock)
     __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_rwlock_trywrlock (pthread_rwlock_t *__rwlock)
     __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1)));




extern int pthread_rwlock_timedwrlock (pthread_rwlock_t *__restrict __rwlock,
           const struct timespec *__restrict
           __abstime) __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1, 2)));
extern int pthread_rwlock_clockwrlock (pthread_rwlock_t *__restrict __rwlock,
           clockid_t __clockid,
           const struct timespec *__restrict
           __abstime) __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1, 3)));
extern int pthread_rwlock_unlock (pthread_rwlock_t *__rwlock)
     __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1)));





extern int pthread_rwlockattr_init (pthread_rwlockattr_t *__attr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_rwlockattr_destroy (pthread_rwlockattr_t *__attr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_rwlockattr_getpshared (const pthread_rwlockattr_t *
       __restrict __attr,
       int *__restrict __pshared)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int pthread_rwlockattr_setpshared (pthread_rwlockattr_t *__attr,
       int __pshared)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_rwlockattr_getkind_np (const pthread_rwlockattr_t *
       __restrict __attr,
       int *__restrict __pref)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int pthread_rwlockattr_setkind_np (pthread_rwlockattr_t *__attr,
       int __pref) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));







extern int pthread_cond_init (pthread_cond_t *__restrict __cond,
         const pthread_condattr_t *__restrict __cond_attr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_cond_destroy (pthread_cond_t *__cond)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_cond_signal (pthread_cond_t *__cond)
     __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_cond_broadcast (pthread_cond_t *__cond)
     __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1)));






extern int pthread_cond_wait (pthread_cond_t *__restrict __cond,
         pthread_mutex_t *__restrict __mutex)
     __attribute__ ((__nonnull__ (1, 2)));
extern int pthread_cond_timedwait (pthread_cond_t *__restrict __cond,
       pthread_mutex_t *__restrict __mutex,
       const struct timespec *__restrict __abstime)
     __attribute__ ((__nonnull__ (1, 2, 3)));
extern int pthread_cond_clockwait (pthread_cond_t *__restrict __cond,
       pthread_mutex_t *__restrict __mutex,
       __clockid_t __clock_id,
       const struct timespec *__restrict __abstime)
     __attribute__ ((__nonnull__ (1, 2, 4)));
extern int pthread_condattr_init (pthread_condattr_t *__attr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_condattr_destroy (pthread_condattr_t *__attr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_condattr_getpshared (const pthread_condattr_t *
     __restrict __attr,
     int *__restrict __pshared)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int pthread_condattr_setpshared (pthread_condattr_t *__attr,
     int __pshared) __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));



extern int pthread_condattr_getclock (const pthread_condattr_t *
          __restrict __attr,
          __clockid_t *__restrict __clock_id)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int pthread_condattr_setclock (pthread_condattr_t *__attr,
          __clockid_t __clock_id)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));
extern int pthread_spin_init (pthread_spinlock_t *__lock, int __pshared)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_spin_destroy (pthread_spinlock_t *__lock)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_spin_lock (pthread_spinlock_t *__lock)
     __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_spin_trylock (pthread_spinlock_t *__lock)
     __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_spin_unlock (pthread_spinlock_t *__lock)
     __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1)));






extern int pthread_barrier_init (pthread_barrier_t *__restrict __barrier,
     const pthread_barrierattr_t *__restrict
     __attr, unsigned int __count)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_barrier_destroy (pthread_barrier_t *__barrier)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_barrier_wait (pthread_barrier_t *__barrier)
     __attribute__ ((__nothrow__)) __attribute__ ((__nonnull__ (1)));



extern int pthread_barrierattr_init (pthread_barrierattr_t *__attr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_barrierattr_destroy (pthread_barrierattr_t *__attr)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_barrierattr_getpshared (const pthread_barrierattr_t *
        __restrict __attr,
        int *__restrict __pshared)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1, 2)));


extern int pthread_barrierattr_setpshared (pthread_barrierattr_t *__attr,
        int __pshared)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));
extern int pthread_key_create (pthread_key_t *__key,
          void (*__destr_function) (void *))
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (1)));


extern int pthread_key_delete (pthread_key_t __key) __attribute__ ((__nothrow__ , __leaf__));


extern void *pthread_getspecific (pthread_key_t __key) __attribute__ ((__nothrow__ , __leaf__));


extern int pthread_setspecific (pthread_key_t __key,
    const void *__pointer)
  __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__access__ (__none__, 2)));




extern int pthread_getcpuclockid (pthread_t __thread_id,
      __clockid_t *__clock_id)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__nonnull__ (2)));




extern pid_t pthread_gettid_np (pthread_t __thread_id);
extern int pthread_atfork (void (*__prepare) (void),
      void (*__parent) (void),
      void (*__child) (void)) __attribute__ ((__nothrow__ , __leaf__));


typedef pthread_once_t CRYPTO_ONCE;
typedef pthread_key_t CRYPTO_THREAD_LOCAL;
typedef pthread_t CRYPTO_THREAD_ID;
int CRYPTO_THREAD_run_once(CRYPTO_ONCE *once, void (*init)(void));

int CRYPTO_THREAD_init_local(CRYPTO_THREAD_LOCAL *key, void (*cleanup)(void *));
void *CRYPTO_THREAD_get_local(CRYPTO_THREAD_LOCAL *key);
int CRYPTO_THREAD_set_local(CRYPTO_THREAD_LOCAL *key, void *val);
int CRYPTO_THREAD_cleanup_local(CRYPTO_THREAD_LOCAL *key);

CRYPTO_THREAD_ID CRYPTO_THREAD_get_current_id(void);
int CRYPTO_THREAD_compare_id(CRYPTO_THREAD_ID a, CRYPTO_THREAD_ID b);

OSSL_LIB_CTX *OSSL_LIB_CTX_new(void);
OSSL_LIB_CTX *OSSL_LIB_CTX_new_from_dispatch(const OSSL_CORE_HANDLE *handle,
    const OSSL_DISPATCH *in);
OSSL_LIB_CTX *OSSL_LIB_CTX_new_child(const OSSL_CORE_HANDLE *handle,
    const OSSL_DISPATCH *in);
int OSSL_LIB_CTX_load_config(OSSL_LIB_CTX *ctx, const char *config_file);
void OSSL_LIB_CTX_free(OSSL_LIB_CTX *);
OSSL_LIB_CTX *OSSL_LIB_CTX_get0_global_default(void);
OSSL_LIB_CTX *OSSL_LIB_CTX_set0_default(OSSL_LIB_CTX *libctx);
int OSSL_LIB_CTX_get_conf_diagnostics(OSSL_LIB_CTX *ctx);
void OSSL_LIB_CTX_set_conf_diagnostics(OSSL_LIB_CTX *ctx, int value);

void OSSL_sleep(uint64_t millis);

void *OSSL_LIB_CTX_get_data(OSSL_LIB_CTX *ctx, int index);
       
void BN_set_flags(BIGNUM *b, int n);
int BN_get_flags(const BIGNUM *b, int n);
void BN_with_flags(BIGNUM *dest, const BIGNUM *b, int flags);


int BN_GENCB_call(BN_GENCB *cb, int a, int b);

BN_GENCB *BN_GENCB_new(void);
void BN_GENCB_free(BN_GENCB *cb);


void BN_GENCB_set_old(BN_GENCB *gencb, void (*callback)(int, int, void *),
    void *cb_arg);


void BN_GENCB_set(BN_GENCB *gencb, int (*callback)(int, int, BN_GENCB *),
    void *cb_arg);

void *BN_GENCB_get_arg(BN_GENCB *cb);
int BN_abs_is_word(const BIGNUM *a, const unsigned long w);
int BN_is_zero(const BIGNUM *a);
int BN_is_one(const BIGNUM *a);
int BN_is_word(const BIGNUM *a, const unsigned long w);
int BN_is_odd(const BIGNUM *a);



void BN_zero_ex(BIGNUM *a);







const BIGNUM *BN_value_one(void);
char *BN_options(void);
BN_CTX *BN_CTX_new_ex(OSSL_LIB_CTX *ctx);
BN_CTX *BN_CTX_new(void);
BN_CTX *BN_CTX_secure_new_ex(OSSL_LIB_CTX *ctx);
BN_CTX *BN_CTX_secure_new(void);
void BN_CTX_free(BN_CTX *c);
void BN_CTX_start(BN_CTX *ctx);
BIGNUM *BN_CTX_get(BN_CTX *ctx);
void BN_CTX_end(BN_CTX *ctx);
int BN_rand_ex(BIGNUM *rnd, int bits, int top, int bottom,
    unsigned int strength, BN_CTX *ctx);
int BN_rand(BIGNUM *rnd, int bits, int top, int bottom);
int BN_priv_rand_ex(BIGNUM *rnd, int bits, int top, int bottom,
    unsigned int strength, BN_CTX *ctx);
int BN_priv_rand(BIGNUM *rnd, int bits, int top, int bottom);
int BN_rand_range_ex(BIGNUM *r, const BIGNUM *range, unsigned int strength,
    BN_CTX *ctx);
int BN_rand_range(BIGNUM *rnd, const BIGNUM *range);
int BN_priv_rand_range_ex(BIGNUM *r, const BIGNUM *range,
    unsigned int strength, BN_CTX *ctx);
int BN_priv_rand_range(BIGNUM *rnd, const BIGNUM *range);

__attribute__((deprecated("Since OpenSSL " "3.0")))
int BN_pseudo_rand(BIGNUM *rnd, int bits, int top, int bottom);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int BN_pseudo_rand_range(BIGNUM *rnd, const BIGNUM *range);

int BN_num_bits(const BIGNUM *a);
int BN_num_bits_word(unsigned long l);
int BN_security_bits(int L, int N);
BIGNUM *BN_new(void);
BIGNUM *BN_secure_new(void);
void BN_clear_free(BIGNUM *a);
BIGNUM *BN_copy(BIGNUM *a, const BIGNUM *b);
void BN_swap(BIGNUM *a, BIGNUM *b);
BIGNUM *BN_bin2bn(const unsigned char *s, int len, BIGNUM *ret);
BIGNUM *BN_signed_bin2bn(const unsigned char *s, int len, BIGNUM *ret);
int BN_bn2bin(const BIGNUM *a, unsigned char *to);
int BN_bn2binpad(const BIGNUM *a, unsigned char *to, int tolen);
int BN_signed_bn2bin(const BIGNUM *a, unsigned char *to, int tolen);
BIGNUM *BN_lebin2bn(const unsigned char *s, int len, BIGNUM *ret);
BIGNUM *BN_signed_lebin2bn(const unsigned char *s, int len, BIGNUM *ret);
int BN_bn2lebinpad(const BIGNUM *a, unsigned char *to, int tolen);
int BN_signed_bn2lebin(const BIGNUM *a, unsigned char *to, int tolen);
BIGNUM *BN_native2bn(const unsigned char *s, int len, BIGNUM *ret);
BIGNUM *BN_signed_native2bn(const unsigned char *s, int len, BIGNUM *ret);
int BN_bn2nativepad(const BIGNUM *a, unsigned char *to, int tolen);
int BN_signed_bn2native(const BIGNUM *a, unsigned char *to, int tolen);
BIGNUM *BN_mpi2bn(const unsigned char *s, int len, BIGNUM *ret);
int BN_bn2mpi(const BIGNUM *a, unsigned char *to);
int BN_sub(BIGNUM *r, const BIGNUM *a, const BIGNUM *b);
int BN_usub(BIGNUM *r, const BIGNUM *a, const BIGNUM *b);
int BN_uadd(BIGNUM *r, const BIGNUM *a, const BIGNUM *b);
int BN_add(BIGNUM *r, const BIGNUM *a, const BIGNUM *b);
int BN_mul(BIGNUM *r, const BIGNUM *a, const BIGNUM *b, BN_CTX *ctx);
int BN_sqr(BIGNUM *r, const BIGNUM *a, BN_CTX *ctx);




void BN_set_negative(BIGNUM *b, int n);




int BN_is_negative(const BIGNUM *b);

int BN_div(BIGNUM *dv, BIGNUM *rem, const BIGNUM *m, const BIGNUM *d,
    BN_CTX *ctx);

int BN_nnmod(BIGNUM *r, const BIGNUM *a, const BIGNUM *m, BN_CTX *ctx);
int BN_mod_add(BIGNUM *r, const BIGNUM *a, const BIGNUM *b, const BIGNUM *m,
    BN_CTX *ctx);
int BN_mod_add_quick(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
    const BIGNUM *m);
int BN_mod_sub(BIGNUM *r, const BIGNUM *a, const BIGNUM *b, const BIGNUM *m,
    BN_CTX *ctx);
int BN_mod_sub_quick(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
    const BIGNUM *m);
int BN_mod_mul(BIGNUM *r, const BIGNUM *a, const BIGNUM *b, const BIGNUM *m,
    BN_CTX *ctx);
int BN_mod_sqr(BIGNUM *r, const BIGNUM *a, const BIGNUM *m, BN_CTX *ctx);
int BN_mod_lshift1(BIGNUM *r, const BIGNUM *a, const BIGNUM *m, BN_CTX *ctx);
int BN_mod_lshift1_quick(BIGNUM *r, const BIGNUM *a, const BIGNUM *m);
int BN_mod_lshift(BIGNUM *r, const BIGNUM *a, int n, const BIGNUM *m,
    BN_CTX *ctx);
int BN_mod_lshift_quick(BIGNUM *r, const BIGNUM *a, int n, const BIGNUM *m);

unsigned long BN_mod_word(const BIGNUM *a, unsigned long w);
unsigned long BN_div_word(BIGNUM *a, unsigned long w);
int BN_mul_word(BIGNUM *a, unsigned long w);
int BN_add_word(BIGNUM *a, unsigned long w);
int BN_sub_word(BIGNUM *a, unsigned long w);
int BN_set_word(BIGNUM *a, unsigned long w);
unsigned long BN_get_word(const BIGNUM *a);

int BN_cmp(const BIGNUM *a, const BIGNUM *b);
void BN_free(BIGNUM *a);
int BN_is_bit_set(const BIGNUM *a, int n);
int BN_lshift(BIGNUM *r, const BIGNUM *a, int n);
int BN_lshift1(BIGNUM *r, const BIGNUM *a);
int BN_exp(BIGNUM *r, const BIGNUM *a, const BIGNUM *p, BN_CTX *ctx);

int BN_mod_exp(BIGNUM *r, const BIGNUM *a, const BIGNUM *p,
    const BIGNUM *m, BN_CTX *ctx);
int BN_mod_exp_mont(BIGNUM *r, const BIGNUM *a, const BIGNUM *p,
    const BIGNUM *m, BN_CTX *ctx, BN_MONT_CTX *m_ctx);
int BN_mod_exp_mont_consttime(BIGNUM *rr, const BIGNUM *a, const BIGNUM *p,
    const BIGNUM *m, BN_CTX *ctx,
    BN_MONT_CTX *in_mont);
int BN_mod_exp_mont_word(BIGNUM *r, unsigned long a, const BIGNUM *p,
    const BIGNUM *m, BN_CTX *ctx, BN_MONT_CTX *m_ctx);
int BN_mod_exp2_mont(BIGNUM *r, const BIGNUM *a1, const BIGNUM *p1,
    const BIGNUM *a2, const BIGNUM *p2, const BIGNUM *m,
    BN_CTX *ctx, BN_MONT_CTX *m_ctx);
int BN_mod_exp_simple(BIGNUM *r, const BIGNUM *a, const BIGNUM *p,
    const BIGNUM *m, BN_CTX *ctx);
int BN_mod_exp_mont_consttime_x2(BIGNUM *rr1, const BIGNUM *a1, const BIGNUM *p1,
    const BIGNUM *m1, BN_MONT_CTX *in_mont1,
    BIGNUM *rr2, const BIGNUM *a2, const BIGNUM *p2,
    const BIGNUM *m2, BN_MONT_CTX *in_mont2,
    BN_CTX *ctx);

int BN_mask_bits(BIGNUM *a, int n);

int BN_print_fp(FILE *fp, const BIGNUM *a);

int BN_print(BIO *bio, const BIGNUM *a);
int BN_reciprocal(BIGNUM *r, const BIGNUM *m, int len, BN_CTX *ctx);
int BN_rshift(BIGNUM *r, const BIGNUM *a, int n);
int BN_rshift1(BIGNUM *r, const BIGNUM *a);
void BN_clear(BIGNUM *a);
BIGNUM *BN_dup(const BIGNUM *a);
int BN_ucmp(const BIGNUM *a, const BIGNUM *b);
int BN_set_bit(BIGNUM *a, int n);
int BN_clear_bit(BIGNUM *a, int n);
char *BN_bn2hex(const BIGNUM *a);
char *BN_bn2dec(const BIGNUM *a);
int BN_hex2bn(BIGNUM **a, const char *str);
int BN_dec2bn(BIGNUM **a, const char *str);
int BN_asc2bn(BIGNUM **a, const char *str);
int BN_gcd(BIGNUM *r, const BIGNUM *a, const BIGNUM *b, BN_CTX *ctx);
int BN_kronecker(const BIGNUM *a, const BIGNUM *b, BN_CTX *ctx);


int BN_are_coprime(BIGNUM *a, const BIGNUM *b, BN_CTX *ctx);
BIGNUM *BN_mod_inverse(BIGNUM *ret,
    const BIGNUM *a, const BIGNUM *n, BN_CTX *ctx);
BIGNUM *BN_mod_sqrt(BIGNUM *ret,
    const BIGNUM *a, const BIGNUM *n, BN_CTX *ctx);

void BN_consttime_swap(unsigned long swap, BIGNUM *a, BIGNUM *b, int nwords);



__attribute__((deprecated("Since OpenSSL " "0.9.8")))
BIGNUM *BN_generate_prime(BIGNUM *ret, int bits, int safe,
    const BIGNUM *add, const BIGNUM *rem,
    void (*callback)(int, int, void *),
    void *cb_arg);
__attribute__((deprecated("Since OpenSSL " "0.9.8")))
int BN_is_prime(const BIGNUM *p, int nchecks,
    void (*callback)(int, int, void *),
    BN_CTX *ctx, void *cb_arg);
__attribute__((deprecated("Since OpenSSL " "0.9.8")))
int BN_is_prime_fasttest(const BIGNUM *p, int nchecks,
    void (*callback)(int, int, void *),
    BN_CTX *ctx, void *cb_arg,
    int do_trial_division);


__attribute__((deprecated("Since OpenSSL " "3.0")))
int BN_is_prime_ex(const BIGNUM *p, int nchecks, BN_CTX *ctx, BN_GENCB *cb);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int BN_is_prime_fasttest_ex(const BIGNUM *p, int nchecks, BN_CTX *ctx,
    int do_trial_division, BN_GENCB *cb);


int BN_generate_prime_ex2(BIGNUM *ret, int bits, int safe,
    const BIGNUM *add, const BIGNUM *rem, BN_GENCB *cb,
    BN_CTX *ctx);
int BN_generate_prime_ex(BIGNUM *ret, int bits, int safe, const BIGNUM *add,
    const BIGNUM *rem, BN_GENCB *cb);
int BN_check_prime(const BIGNUM *p, BN_CTX *ctx, BN_GENCB *cb);


__attribute__((deprecated("Since OpenSSL " "3.0")))
int BN_X931_generate_Xpq(BIGNUM *Xp, BIGNUM *Xq, int nbits, BN_CTX *ctx);

__attribute__((deprecated("Since OpenSSL " "3.0")))
int BN_X931_derive_prime_ex(BIGNUM *p, BIGNUM *p1, BIGNUM *p2,
    const BIGNUM *Xp, const BIGNUM *Xp1,
    const BIGNUM *Xp2, const BIGNUM *e, BN_CTX *ctx,
    BN_GENCB *cb);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int BN_X931_generate_prime_ex(BIGNUM *p, BIGNUM *p1, BIGNUM *p2, BIGNUM *Xp1,
    BIGNUM *Xp2, const BIGNUM *Xp, const BIGNUM *e,
    BN_CTX *ctx, BN_GENCB *cb);


BN_MONT_CTX *BN_MONT_CTX_new(void);
int BN_mod_mul_montgomery(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
    BN_MONT_CTX *mont, BN_CTX *ctx);
int BN_to_montgomery(BIGNUM *r, const BIGNUM *a, BN_MONT_CTX *mont,
    BN_CTX *ctx);
int BN_from_montgomery(BIGNUM *r, const BIGNUM *a, BN_MONT_CTX *mont,
    BN_CTX *ctx);
void BN_MONT_CTX_free(BN_MONT_CTX *mont);
int BN_MONT_CTX_set(BN_MONT_CTX *mont, const BIGNUM *mod, BN_CTX *ctx);
BN_MONT_CTX *BN_MONT_CTX_copy(BN_MONT_CTX *to, BN_MONT_CTX *from);
BN_MONT_CTX *BN_MONT_CTX_set_locked(BN_MONT_CTX **pmont, CRYPTO_RWLOCK *lock,
    const BIGNUM *mod, BN_CTX *ctx);





BN_BLINDING *BN_BLINDING_new(const BIGNUM *A, const BIGNUM *Ai, BIGNUM *mod);
void BN_BLINDING_free(BN_BLINDING *b);
int BN_BLINDING_update(BN_BLINDING *b, BN_CTX *ctx);
int BN_BLINDING_convert(BIGNUM *n, BN_BLINDING *b, BN_CTX *ctx);
int BN_BLINDING_invert(BIGNUM *n, BN_BLINDING *b, BN_CTX *ctx);
int BN_BLINDING_convert_ex(BIGNUM *n, BIGNUM *r, BN_BLINDING *b, BN_CTX *);
int BN_BLINDING_invert_ex(BIGNUM *n, const BIGNUM *r, BN_BLINDING *b,
    BN_CTX *);

int BN_BLINDING_is_current_thread(BN_BLINDING *b);
void BN_BLINDING_set_current_thread(BN_BLINDING *b);
int BN_BLINDING_lock(BN_BLINDING *b);
int BN_BLINDING_unlock(BN_BLINDING *b);

unsigned long BN_BLINDING_get_flags(const BN_BLINDING *);
void BN_BLINDING_set_flags(BN_BLINDING *, unsigned long);
BN_BLINDING *BN_BLINDING_create_param(BN_BLINDING *b,
    const BIGNUM *e, BIGNUM *m, BN_CTX *ctx,
    int (*bn_mod_exp)(BIGNUM *r,
        const BIGNUM *a,
        const BIGNUM *p,
        const BIGNUM *m,
        BN_CTX *ctx,
        BN_MONT_CTX *m_ctx),
    BN_MONT_CTX *m_ctx);

__attribute__((deprecated("Since OpenSSL " "0.9.8")))
void BN_set_params(int mul, int high, int low, int mont);
__attribute__((deprecated("Since OpenSSL " "0.9.8")))
int BN_get_params(int which);


BN_RECP_CTX *BN_RECP_CTX_new(void);
void BN_RECP_CTX_free(BN_RECP_CTX *recp);
int BN_RECP_CTX_set(BN_RECP_CTX *recp, const BIGNUM *rdiv, BN_CTX *ctx);
int BN_mod_mul_reciprocal(BIGNUM *r, const BIGNUM *x, const BIGNUM *y,
    BN_RECP_CTX *recp, BN_CTX *ctx);
int BN_mod_exp_recp(BIGNUM *r, const BIGNUM *a, const BIGNUM *p,
    const BIGNUM *m, BN_CTX *ctx);
int BN_div_recp(BIGNUM *dv, BIGNUM *rem, const BIGNUM *m,
    BN_RECP_CTX *recp, BN_CTX *ctx);
int BN_GF2m_add(BIGNUM *r, const BIGNUM *a, const BIGNUM *b);




int BN_GF2m_mod(BIGNUM *r, const BIGNUM *a, const BIGNUM *p);

int BN_GF2m_mod_mul(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
    const BIGNUM *p, BN_CTX *ctx);

int BN_GF2m_mod_sqr(BIGNUM *r, const BIGNUM *a, const BIGNUM *p, BN_CTX *ctx);

int BN_GF2m_mod_inv(BIGNUM *r, const BIGNUM *b, const BIGNUM *p, BN_CTX *ctx);

int BN_GF2m_mod_div(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
    const BIGNUM *p, BN_CTX *ctx);

int BN_GF2m_mod_exp(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
    const BIGNUM *p, BN_CTX *ctx);

int BN_GF2m_mod_sqrt(BIGNUM *r, const BIGNUM *a, const BIGNUM *p,
    BN_CTX *ctx);

int BN_GF2m_mod_solve_quad(BIGNUM *r, const BIGNUM *a, const BIGNUM *p,
    BN_CTX *ctx);
int BN_GF2m_mod_arr(BIGNUM *r, const BIGNUM *a, const int p[]);

int BN_GF2m_mod_mul_arr(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
    const int p[], BN_CTX *ctx);

int BN_GF2m_mod_sqr_arr(BIGNUM *r, const BIGNUM *a, const int p[],
    BN_CTX *ctx);

int BN_GF2m_mod_inv_arr(BIGNUM *r, const BIGNUM *b, const int p[],
    BN_CTX *ctx);

int BN_GF2m_mod_div_arr(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
    const int p[], BN_CTX *ctx);

int BN_GF2m_mod_exp_arr(BIGNUM *r, const BIGNUM *a, const BIGNUM *b,
    const int p[], BN_CTX *ctx);

int BN_GF2m_mod_sqrt_arr(BIGNUM *r, const BIGNUM *a,
    const int p[], BN_CTX *ctx);

int BN_GF2m_mod_solve_quad_arr(BIGNUM *r, const BIGNUM *a,
    const int p[], BN_CTX *ctx);
int BN_GF2m_poly2arr(const BIGNUM *a, int p[], int max);
int BN_GF2m_arr2poly(const int p[], BIGNUM *a);






int BN_nist_mod_192(BIGNUM *r, const BIGNUM *a, const BIGNUM *p, BN_CTX *ctx);
int BN_nist_mod_224(BIGNUM *r, const BIGNUM *a, const BIGNUM *p, BN_CTX *ctx);
int BN_nist_mod_256(BIGNUM *r, const BIGNUM *a, const BIGNUM *p, BN_CTX *ctx);
int BN_nist_mod_384(BIGNUM *r, const BIGNUM *a, const BIGNUM *p, BN_CTX *ctx);
int BN_nist_mod_521(BIGNUM *r, const BIGNUM *a, const BIGNUM *p, BN_CTX *ctx);

const BIGNUM *BN_get0_nist_prime_192(void);
const BIGNUM *BN_get0_nist_prime_224(void);
const BIGNUM *BN_get0_nist_prime_256(void);
const BIGNUM *BN_get0_nist_prime_384(void);
const BIGNUM *BN_get0_nist_prime_521(void);

int (*BN_nist_mod_func(const BIGNUM *p))(BIGNUM *r, const BIGNUM *a,
    const BIGNUM *field, BN_CTX *ctx);

int BN_generate_dsa_nonce(BIGNUM *out, const BIGNUM *range,
    const BIGNUM *priv, const unsigned char *message,
    size_t message_len, BN_CTX *ctx);


BIGNUM *BN_get_rfc2409_prime_768(BIGNUM *bn);
BIGNUM *BN_get_rfc2409_prime_1024(BIGNUM *bn);


BIGNUM *BN_get_rfc3526_prime_1536(BIGNUM *bn);
BIGNUM *BN_get_rfc3526_prime_2048(BIGNUM *bn);
BIGNUM *BN_get_rfc3526_prime_3072(BIGNUM *bn);
BIGNUM *BN_get_rfc3526_prime_4096(BIGNUM *bn);
BIGNUM *BN_get_rfc3526_prime_6144(BIGNUM *bn);
BIGNUM *BN_get_rfc3526_prime_8192(BIGNUM *bn);
int BN_bntest_rand(BIGNUM *rnd, int bits, int top, int bottom);
OSSL_PARAM *OSSL_PARAM_locate(OSSL_PARAM *p, const char *key);
const OSSL_PARAM *OSSL_PARAM_locate_const(const OSSL_PARAM *p, const char *key);


OSSL_PARAM OSSL_PARAM_construct_int(const char *key, int *buf);
OSSL_PARAM OSSL_PARAM_construct_uint(const char *key, unsigned int *buf);
OSSL_PARAM OSSL_PARAM_construct_long(const char *key, long int *buf);
OSSL_PARAM OSSL_PARAM_construct_ulong(const char *key, unsigned long int *buf);
OSSL_PARAM OSSL_PARAM_construct_int32(const char *key, int32_t *buf);
OSSL_PARAM OSSL_PARAM_construct_uint32(const char *key, uint32_t *buf);
OSSL_PARAM OSSL_PARAM_construct_int64(const char *key, int64_t *buf);
OSSL_PARAM OSSL_PARAM_construct_uint64(const char *key, uint64_t *buf);
OSSL_PARAM OSSL_PARAM_construct_size_t(const char *key, size_t *buf);
OSSL_PARAM OSSL_PARAM_construct_time_t(const char *key, time_t *buf);
OSSL_PARAM OSSL_PARAM_construct_BN(const char *key, unsigned char *buf,
    size_t bsize);
OSSL_PARAM OSSL_PARAM_construct_double(const char *key, double *buf);
OSSL_PARAM OSSL_PARAM_construct_utf8_string(const char *key, char *buf,
    size_t bsize);
OSSL_PARAM OSSL_PARAM_construct_utf8_ptr(const char *key, char **buf,
    size_t bsize);
OSSL_PARAM OSSL_PARAM_construct_octet_string(const char *key, void *buf,
    size_t bsize);
OSSL_PARAM OSSL_PARAM_construct_octet_ptr(const char *key, void **buf,
    size_t bsize);
OSSL_PARAM OSSL_PARAM_construct_end(void);

int OSSL_PARAM_allocate_from_text(OSSL_PARAM *to,
    const OSSL_PARAM *paramdefs,
    const char *key, const char *value,
    size_t value_n, int *found);

int OSSL_PARAM_print_to_bio(const OSSL_PARAM *params, BIO *bio,
    int print_values);

int OSSL_PARAM_get_int(const OSSL_PARAM *p, int *val);
int OSSL_PARAM_get_uint(const OSSL_PARAM *p, unsigned int *val);
int OSSL_PARAM_get_long(const OSSL_PARAM *p, long int *val);
int OSSL_PARAM_get_ulong(const OSSL_PARAM *p, unsigned long int *val);
int OSSL_PARAM_get_int32(const OSSL_PARAM *p, int32_t *val);
int OSSL_PARAM_get_uint32(const OSSL_PARAM *p, uint32_t *val);
int OSSL_PARAM_get_int64(const OSSL_PARAM *p, int64_t *val);
int OSSL_PARAM_get_uint64(const OSSL_PARAM *p, uint64_t *val);
int OSSL_PARAM_get_size_t(const OSSL_PARAM *p, size_t *val);
int OSSL_PARAM_get_time_t(const OSSL_PARAM *p, time_t *val);

int OSSL_PARAM_set_int(OSSL_PARAM *p, int val);
int OSSL_PARAM_set_uint(OSSL_PARAM *p, unsigned int val);
int OSSL_PARAM_set_long(OSSL_PARAM *p, long int val);
int OSSL_PARAM_set_ulong(OSSL_PARAM *p, unsigned long int val);
int OSSL_PARAM_set_int32(OSSL_PARAM *p, int32_t val);
int OSSL_PARAM_set_uint32(OSSL_PARAM *p, uint32_t val);
int OSSL_PARAM_set_int64(OSSL_PARAM *p, int64_t val);
int OSSL_PARAM_set_uint64(OSSL_PARAM *p, uint64_t val);
int OSSL_PARAM_set_size_t(OSSL_PARAM *p, size_t val);
int OSSL_PARAM_set_time_t(OSSL_PARAM *p, time_t val);

int OSSL_PARAM_get_double(const OSSL_PARAM *p, double *val);
int OSSL_PARAM_set_double(OSSL_PARAM *p, double val);

int OSSL_PARAM_get_BN(const OSSL_PARAM *p, BIGNUM **val);
int OSSL_PARAM_set_BN(OSSL_PARAM *p, const BIGNUM *val);

int OSSL_PARAM_get_utf8_string(const OSSL_PARAM *p, char **val, size_t max_len);
int OSSL_PARAM_set_utf8_string(OSSL_PARAM *p, const char *val);

int OSSL_PARAM_get_octet_string(const OSSL_PARAM *p, void **val, size_t max_len,
    size_t *used_len);
int OSSL_PARAM_set_octet_string(OSSL_PARAM *p, const void *val, size_t len);

int OSSL_PARAM_get_utf8_ptr(const OSSL_PARAM *p, const char **val);
int OSSL_PARAM_set_utf8_ptr(OSSL_PARAM *p, const char *val);

int OSSL_PARAM_get_octet_ptr(const OSSL_PARAM *p, const void **val,
    size_t *used_len);
int OSSL_PARAM_set_octet_ptr(OSSL_PARAM *p, const void *val,
    size_t used_len);

int OSSL_PARAM_get_utf8_string_ptr(const OSSL_PARAM *p, const char **val);
int OSSL_PARAM_get_octet_string_ptr(const OSSL_PARAM *p, const void **val,
    size_t *used_len);

int OSSL_PARAM_modified(const OSSL_PARAM *p);
void OSSL_PARAM_set_all_unmodified(OSSL_PARAM *p);

OSSL_PARAM *OSSL_PARAM_dup(const OSSL_PARAM *p);
OSSL_PARAM *OSSL_PARAM_merge(const OSSL_PARAM *p1, const OSSL_PARAM *p2);
void OSSL_PARAM_free(OSSL_PARAM *p);

int OSSL_PARAM_set_octet_string_or_ptr(OSSL_PARAM *p, const void *val,
    size_t len);

typedef int(OSSL_INDICATOR_CALLBACK)(const char *type, const char *desc,
    const OSSL_PARAM params[]);

void OSSL_INDICATOR_set_callback(OSSL_LIB_CTX *libctx,
    OSSL_INDICATOR_CALLBACK *cb);
void OSSL_INDICATOR_get_callback(OSSL_LIB_CTX *libctx,
    OSSL_INDICATOR_CALLBACK **cb);
typedef void (*OSSL_FUNC)(void);
typedef const OSSL_PARAM *(OSSL_FUNC_core_gettable_params_fn) (const OSSL_CORE_HANDLE *prov); static __attribute__((unused)) inline OSSL_FUNC_core_gettable_params_fn * OSSL_FUNC_core_gettable_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_core_gettable_params_fn *)opf->function; }


typedef int(OSSL_FUNC_core_get_params_fn) (const OSSL_CORE_HANDLE *prov, OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_core_get_params_fn * OSSL_FUNC_core_get_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_core_get_params_fn *)opf->function; }

typedef int(OSSL_FUNC_core_thread_start_fn) (const OSSL_CORE_HANDLE *prov, OSSL_thread_stop_handler_fn handfn, void *arg); static __attribute__((unused)) inline OSSL_FUNC_core_thread_start_fn * OSSL_FUNC_core_thread_start(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_core_thread_start_fn *)opf->function; }

typedef OPENSSL_CORE_CTX *(OSSL_FUNC_core_get_libctx_fn) (const OSSL_CORE_HANDLE *prov); static __attribute__((unused)) inline OSSL_FUNC_core_get_libctx_fn * OSSL_FUNC_core_get_libctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_core_get_libctx_fn *)opf->function; }


typedef void(OSSL_FUNC_core_new_error_fn) (const OSSL_CORE_HANDLE *prov); static __attribute__((unused)) inline OSSL_FUNC_core_new_error_fn * OSSL_FUNC_core_new_error(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_core_new_error_fn *)opf->function; }

typedef void(OSSL_FUNC_core_set_error_debug_fn) (const OSSL_CORE_HANDLE *prov, const char *file, int line, const char *func); static __attribute__((unused)) inline OSSL_FUNC_core_set_error_debug_fn * OSSL_FUNC_core_set_error_debug(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_core_set_error_debug_fn *)opf->function; }



typedef void(OSSL_FUNC_core_vset_error_fn) (const OSSL_CORE_HANDLE *prov, uint32_t reason, const char *fmt, va_list args); static __attribute__((unused)) inline OSSL_FUNC_core_vset_error_fn * OSSL_FUNC_core_vset_error(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_core_vset_error_fn *)opf->function; }



typedef int(OSSL_FUNC_core_set_error_mark_fn) (const OSSL_CORE_HANDLE *prov); static __attribute__((unused)) inline OSSL_FUNC_core_set_error_mark_fn * OSSL_FUNC_core_set_error_mark(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_core_set_error_mark_fn *)opf->function; }

typedef int(OSSL_FUNC_core_clear_last_error_mark_fn) (const OSSL_CORE_HANDLE *prov); static __attribute__((unused)) inline OSSL_FUNC_core_clear_last_error_mark_fn * OSSL_FUNC_core_clear_last_error_mark(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_core_clear_last_error_mark_fn *)opf->function; }


typedef int(OSSL_FUNC_core_pop_error_to_mark_fn) (const OSSL_CORE_HANDLE *prov); static __attribute__((unused)) inline OSSL_FUNC_core_pop_error_to_mark_fn * OSSL_FUNC_core_pop_error_to_mark(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_core_pop_error_to_mark_fn *)opf->function; }






typedef int(OSSL_FUNC_core_obj_add_sigid_fn) (const OSSL_CORE_HANDLE *prov, const char *sign_name, const char *digest_name, const char *pkey_name); static __attribute__((unused)) inline OSSL_FUNC_core_obj_add_sigid_fn * OSSL_FUNC_core_obj_add_sigid(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_core_obj_add_sigid_fn *)opf->function; }


typedef int(OSSL_FUNC_core_obj_create_fn) (const OSSL_CORE_HANDLE *prov, const char *oid, const char *sn, const char *ln); static __attribute__((unused)) inline OSSL_FUNC_core_obj_create_fn * OSSL_FUNC_core_obj_create(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_core_obj_create_fn *)opf->function; }





typedef void *(OSSL_FUNC_CRYPTO_malloc_fn) (size_t num, const char *file, int line); static __attribute__((unused)) inline OSSL_FUNC_CRYPTO_malloc_fn * OSSL_FUNC_CRYPTO_malloc(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_CRYPTO_malloc_fn *)opf->function; }


typedef void *(OSSL_FUNC_CRYPTO_zalloc_fn) (size_t num, const char *file, int line); static __attribute__((unused)) inline OSSL_FUNC_CRYPTO_zalloc_fn * OSSL_FUNC_CRYPTO_zalloc(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_CRYPTO_zalloc_fn *)opf->function; }


typedef void(OSSL_FUNC_CRYPTO_free_fn) (void *ptr, const char *file, int line); static __attribute__((unused)) inline OSSL_FUNC_CRYPTO_free_fn * OSSL_FUNC_CRYPTO_free(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_CRYPTO_free_fn *)opf->function; }


typedef void(OSSL_FUNC_CRYPTO_clear_free_fn) (void *ptr, size_t num, const char *file, int line); static __attribute__((unused)) inline OSSL_FUNC_CRYPTO_clear_free_fn * OSSL_FUNC_CRYPTO_clear_free(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_CRYPTO_clear_free_fn *)opf->function; }


typedef void *(OSSL_FUNC_CRYPTO_realloc_fn) (void *addr, size_t num, const char *file, int line); static __attribute__((unused)) inline OSSL_FUNC_CRYPTO_realloc_fn * OSSL_FUNC_CRYPTO_realloc(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_CRYPTO_realloc_fn *)opf->function; }


typedef void *(OSSL_FUNC_CRYPTO_clear_realloc_fn) (void *addr, size_t old_num, size_t num, const char *file, int line); static __attribute__((unused)) inline OSSL_FUNC_CRYPTO_clear_realloc_fn * OSSL_FUNC_CRYPTO_clear_realloc(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_CRYPTO_clear_realloc_fn *)opf->function; }


typedef void *(OSSL_FUNC_CRYPTO_secure_malloc_fn) (size_t num, const char *file, int line); static __attribute__((unused)) inline OSSL_FUNC_CRYPTO_secure_malloc_fn * OSSL_FUNC_CRYPTO_secure_malloc(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_CRYPTO_secure_malloc_fn *)opf->function; }


typedef void *(OSSL_FUNC_CRYPTO_secure_zalloc_fn) (size_t num, const char *file, int line); static __attribute__((unused)) inline OSSL_FUNC_CRYPTO_secure_zalloc_fn * OSSL_FUNC_CRYPTO_secure_zalloc(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_CRYPTO_secure_zalloc_fn *)opf->function; }


typedef void(OSSL_FUNC_CRYPTO_secure_free_fn) (void *ptr, const char *file, int line); static __attribute__((unused)) inline OSSL_FUNC_CRYPTO_secure_free_fn * OSSL_FUNC_CRYPTO_secure_free(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_CRYPTO_secure_free_fn *)opf->function; }


typedef void(OSSL_FUNC_CRYPTO_secure_clear_free_fn) (void *ptr, size_t num, const char *file, int line); static __attribute__((unused)) inline OSSL_FUNC_CRYPTO_secure_clear_free_fn * OSSL_FUNC_CRYPTO_secure_clear_free(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_CRYPTO_secure_clear_free_fn *)opf->function; }


typedef int(OSSL_FUNC_CRYPTO_secure_allocated_fn) (const void *ptr); static __attribute__((unused)) inline OSSL_FUNC_CRYPTO_secure_allocated_fn * OSSL_FUNC_CRYPTO_secure_allocated(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_CRYPTO_secure_allocated_fn *)opf->function; }


typedef void(OSSL_FUNC_OPENSSL_cleanse_fn) (void *ptr, size_t len); static __attribute__((unused)) inline OSSL_FUNC_OPENSSL_cleanse_fn * OSSL_FUNC_OPENSSL_cleanse(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_OPENSSL_cleanse_fn *)opf->function; }
typedef OSSL_CORE_BIO *(OSSL_FUNC_BIO_new_file_fn) (const char *filename, const char *mode); static __attribute__((unused)) inline OSSL_FUNC_BIO_new_file_fn * OSSL_FUNC_BIO_new_file(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_BIO_new_file_fn *)opf->function; }
typedef OSSL_CORE_BIO *(OSSL_FUNC_BIO_new_membuf_fn) (const void *buf, int len); static __attribute__((unused)) inline OSSL_FUNC_BIO_new_membuf_fn * OSSL_FUNC_BIO_new_membuf(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_BIO_new_membuf_fn *)opf->function; }
typedef int(OSSL_FUNC_BIO_read_ex_fn) (OSSL_CORE_BIO *bio, void *data, size_t data_len, size_t *bytes_read); static __attribute__((unused)) inline OSSL_FUNC_BIO_read_ex_fn * OSSL_FUNC_BIO_read_ex(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_BIO_read_ex_fn *)opf->function; }
typedef int(OSSL_FUNC_BIO_write_ex_fn) (OSSL_CORE_BIO *bio, const void *data, size_t data_len, size_t *written); static __attribute__((unused)) inline OSSL_FUNC_BIO_write_ex_fn * OSSL_FUNC_BIO_write_ex(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_BIO_write_ex_fn *)opf->function; }
typedef int(OSSL_FUNC_BIO_gets_fn) (OSSL_CORE_BIO *bio, char *buf, int size); static __attribute__((unused)) inline OSSL_FUNC_BIO_gets_fn * OSSL_FUNC_BIO_gets(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_BIO_gets_fn *)opf->function; }
typedef int(OSSL_FUNC_BIO_puts_fn) (OSSL_CORE_BIO *bio, const char *str); static __attribute__((unused)) inline OSSL_FUNC_BIO_puts_fn * OSSL_FUNC_BIO_puts(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_BIO_puts_fn *)opf->function; }
typedef int(OSSL_FUNC_BIO_up_ref_fn) (OSSL_CORE_BIO *bio); static __attribute__((unused)) inline OSSL_FUNC_BIO_up_ref_fn * OSSL_FUNC_BIO_up_ref(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_BIO_up_ref_fn *)opf->function; }
typedef int(OSSL_FUNC_BIO_free_fn) (OSSL_CORE_BIO *bio); static __attribute__((unused)) inline OSSL_FUNC_BIO_free_fn * OSSL_FUNC_BIO_free(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_BIO_free_fn *)opf->function; }
typedef int(OSSL_FUNC_BIO_vprintf_fn) (OSSL_CORE_BIO *bio, const char *format, va_list args); static __attribute__((unused)) inline OSSL_FUNC_BIO_vprintf_fn * OSSL_FUNC_BIO_vprintf(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_BIO_vprintf_fn *)opf->function; }
typedef int(OSSL_FUNC_BIO_vsnprintf_fn) (char *buf, size_t n, const char *fmt, va_list args); static __attribute__((unused)) inline OSSL_FUNC_BIO_vsnprintf_fn * OSSL_FUNC_BIO_vsnprintf(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_BIO_vsnprintf_fn *)opf->function; }

typedef int(OSSL_FUNC_BIO_ctrl_fn) (OSSL_CORE_BIO *bio, int cmd, long num, void *ptr); static __attribute__((unused)) inline OSSL_FUNC_BIO_ctrl_fn * OSSL_FUNC_BIO_ctrl(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_BIO_ctrl_fn *)opf->function; }
typedef void(OSSL_FUNC_indicator_cb_fn) (OPENSSL_CORE_CTX *ctx, OSSL_INDICATOR_CALLBACK **cb); static __attribute__((unused)) inline OSSL_FUNC_indicator_cb_fn * OSSL_FUNC_indicator_cb(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_indicator_cb_fn *)opf->function; }

typedef void(OSSL_FUNC_self_test_cb_fn) (OPENSSL_CORE_CTX *ctx, OSSL_CALLBACK **cb, void **cbarg); static __attribute__((unused)) inline OSSL_FUNC_self_test_cb_fn * OSSL_FUNC_self_test_cb(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_self_test_cb_fn *)opf->function; }






typedef size_t(OSSL_FUNC_get_entropy_fn) (const OSSL_CORE_HANDLE *handle, unsigned char **pout, int entropy, size_t min_len, size_t max_len); static __attribute__((unused)) inline OSSL_FUNC_get_entropy_fn * OSSL_FUNC_get_entropy(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_get_entropy_fn *)opf->function; }
typedef size_t(OSSL_FUNC_get_user_entropy_fn) (const OSSL_CORE_HANDLE *handle, unsigned char **pout, int entropy, size_t min_len, size_t max_len); static __attribute__((unused)) inline OSSL_FUNC_get_user_entropy_fn * OSSL_FUNC_get_user_entropy(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_get_user_entropy_fn *)opf->function; }
typedef void(OSSL_FUNC_cleanup_entropy_fn) (const OSSL_CORE_HANDLE *handle, unsigned char *buf, size_t len); static __attribute__((unused)) inline OSSL_FUNC_cleanup_entropy_fn * OSSL_FUNC_cleanup_entropy(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cleanup_entropy_fn *)opf->function; }
typedef void(OSSL_FUNC_cleanup_user_entropy_fn) (const OSSL_CORE_HANDLE *handle, unsigned char *buf, size_t len); static __attribute__((unused)) inline OSSL_FUNC_cleanup_user_entropy_fn * OSSL_FUNC_cleanup_user_entropy(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cleanup_user_entropy_fn *)opf->function; }
typedef size_t(OSSL_FUNC_get_nonce_fn) (const OSSL_CORE_HANDLE *handle, unsigned char **pout, size_t min_len, size_t max_len, const void *salt, size_t salt_len); static __attribute__((unused)) inline OSSL_FUNC_get_nonce_fn * OSSL_FUNC_get_nonce(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_get_nonce_fn *)opf->function; }
typedef size_t(OSSL_FUNC_get_user_nonce_fn) (const OSSL_CORE_HANDLE *handle, unsigned char **pout, size_t min_len, size_t max_len, const void *salt, size_t salt_len); static __attribute__((unused)) inline OSSL_FUNC_get_user_nonce_fn * OSSL_FUNC_get_user_nonce(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_get_user_nonce_fn *)opf->function; }
typedef void(OSSL_FUNC_cleanup_nonce_fn) (const OSSL_CORE_HANDLE *handle, unsigned char *buf, size_t len); static __attribute__((unused)) inline OSSL_FUNC_cleanup_nonce_fn * OSSL_FUNC_cleanup_nonce(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cleanup_nonce_fn *)opf->function; }
typedef void(OSSL_FUNC_cleanup_user_nonce_fn) (const OSSL_CORE_HANDLE *handle, unsigned char *buf, size_t len); static __attribute__((unused)) inline OSSL_FUNC_cleanup_user_nonce_fn * OSSL_FUNC_cleanup_user_nonce(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cleanup_user_nonce_fn *)opf->function; }
typedef int(OSSL_FUNC_provider_register_child_cb_fn) (const OSSL_CORE_HANDLE *handle, int (*create_cb)(const OSSL_CORE_HANDLE *provider, void *cbdata), int (*remove_cb)(const OSSL_CORE_HANDLE *provider, void *cbdata), int (*global_props_cb)(const char *props, void *cbdata), void *cbdata); static __attribute__((unused)) inline OSSL_FUNC_provider_register_child_cb_fn * OSSL_FUNC_provider_register_child_cb(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_provider_register_child_cb_fn *)opf->function; }





typedef void(OSSL_FUNC_provider_deregister_child_cb_fn) (const OSSL_CORE_HANDLE *handle); static __attribute__((unused)) inline OSSL_FUNC_provider_deregister_child_cb_fn * OSSL_FUNC_provider_deregister_child_cb(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_provider_deregister_child_cb_fn *)opf->function; }

typedef const char *(OSSL_FUNC_provider_name_fn) (const OSSL_CORE_HANDLE *prov); static __attribute__((unused)) inline OSSL_FUNC_provider_name_fn * OSSL_FUNC_provider_name(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_provider_name_fn *)opf->function; }

typedef void *(OSSL_FUNC_provider_get0_provider_ctx_fn) (const OSSL_CORE_HANDLE *prov); static __attribute__((unused)) inline OSSL_FUNC_provider_get0_provider_ctx_fn * OSSL_FUNC_provider_get0_provider_ctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_provider_get0_provider_ctx_fn *)opf->function; }

typedef const OSSL_DISPATCH *(OSSL_FUNC_provider_get0_dispatch_fn) (const OSSL_CORE_HANDLE *prov); static __attribute__((unused)) inline OSSL_FUNC_provider_get0_dispatch_fn * OSSL_FUNC_provider_get0_dispatch(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_provider_get0_dispatch_fn *)opf->function; }

typedef int(OSSL_FUNC_provider_up_ref_fn) (const OSSL_CORE_HANDLE *prov, int activate); static __attribute__((unused)) inline OSSL_FUNC_provider_up_ref_fn * OSSL_FUNC_provider_up_ref(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_provider_up_ref_fn *)opf->function; }

typedef int(OSSL_FUNC_provider_free_fn) (const OSSL_CORE_HANDLE *prov, int deactivate); static __attribute__((unused)) inline OSSL_FUNC_provider_free_fn * OSSL_FUNC_provider_free(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_provider_free_fn *)opf->function; }




typedef int(OSSL_FUNC_core_count_to_mark_fn) (const OSSL_CORE_HANDLE *prov); static __attribute__((unused)) inline OSSL_FUNC_core_count_to_mark_fn * OSSL_FUNC_core_count_to_mark(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_core_count_to_mark_fn *)opf->function; }



typedef void(OSSL_FUNC_provider_teardown_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_provider_teardown_fn * OSSL_FUNC_provider_teardown(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_provider_teardown_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_provider_gettable_params_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_provider_gettable_params_fn * OSSL_FUNC_provider_gettable_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_provider_gettable_params_fn *)opf->function; }


typedef int(OSSL_FUNC_provider_get_params_fn) (void *provctx, OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_provider_get_params_fn * OSSL_FUNC_provider_get_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_provider_get_params_fn *)opf->function; }

typedef const OSSL_ALGORITHM *(OSSL_FUNC_provider_query_operation_fn) (void *provctx, int operation_id, int *no_store); static __attribute__((unused)) inline OSSL_FUNC_provider_query_operation_fn * OSSL_FUNC_provider_query_operation(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_provider_query_operation_fn *)opf->function; }


typedef void(OSSL_FUNC_provider_unquery_operation_fn) (void *provctx, int operation_id, const OSSL_ALGORITHM *); static __attribute__((unused)) inline OSSL_FUNC_provider_unquery_operation_fn * OSSL_FUNC_provider_unquery_operation(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_provider_unquery_operation_fn *)opf->function; }


typedef const OSSL_ITEM *(OSSL_FUNC_provider_get_reason_strings_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_provider_get_reason_strings_fn * OSSL_FUNC_provider_get_reason_strings(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_provider_get_reason_strings_fn *)opf->function; }


typedef int(OSSL_FUNC_provider_get_capabilities_fn) (void *provctx, const char *capability, OSSL_CALLBACK *cb, void *arg); static __attribute__((unused)) inline OSSL_FUNC_provider_get_capabilities_fn * OSSL_FUNC_provider_get_capabilities(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_provider_get_capabilities_fn *)opf->function; }

typedef int(OSSL_FUNC_provider_self_test_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_provider_self_test_fn * OSSL_FUNC_provider_self_test(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_provider_self_test_fn *)opf->function; }

typedef int(OSSL_FUNC_provider_random_bytes_fn) (void *provctx, int which, void *buf, size_t n, unsigned int strength); static __attribute__((unused)) inline OSSL_FUNC_provider_random_bytes_fn * OSSL_FUNC_provider_random_bytes(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_provider_random_bytes_fn *)opf->function; }



typedef int(OSSL_FUNC_SSL_QUIC_TLS_crypto_send_fn) (SSL *s, const unsigned char *buf, size_t buf_len, size_t *consumed, void *arg); static __attribute__((unused)) inline OSSL_FUNC_SSL_QUIC_TLS_crypto_send_fn * OSSL_FUNC_SSL_QUIC_TLS_crypto_send(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_SSL_QUIC_TLS_crypto_send_fn *)opf->function; }



typedef int(OSSL_FUNC_SSL_QUIC_TLS_crypto_recv_rcd_fn) (SSL *s, const unsigned char **buf, size_t *bytes_read, void *arg); static __attribute__((unused)) inline OSSL_FUNC_SSL_QUIC_TLS_crypto_recv_rcd_fn * OSSL_FUNC_SSL_QUIC_TLS_crypto_recv_rcd(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_SSL_QUIC_TLS_crypto_recv_rcd_fn *)opf->function; }



typedef int(OSSL_FUNC_SSL_QUIC_TLS_crypto_release_rcd_fn) (SSL *s, size_t bytes_read, void *arg); static __attribute__((unused)) inline OSSL_FUNC_SSL_QUIC_TLS_crypto_release_rcd_fn * OSSL_FUNC_SSL_QUIC_TLS_crypto_release_rcd(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_SSL_QUIC_TLS_crypto_release_rcd_fn *)opf->function; }


typedef int(OSSL_FUNC_SSL_QUIC_TLS_yield_secret_fn) (SSL *s, uint32_t prot_level, int direction, const unsigned char *secret, size_t secret_len, void *arg); static __attribute__((unused)) inline OSSL_FUNC_SSL_QUIC_TLS_yield_secret_fn * OSSL_FUNC_SSL_QUIC_TLS_yield_secret(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_SSL_QUIC_TLS_yield_secret_fn *)opf->function; }



typedef int(OSSL_FUNC_SSL_QUIC_TLS_got_transport_params_fn) (SSL *s, const unsigned char *params, size_t params_len, void *arg); static __attribute__((unused)) inline OSSL_FUNC_SSL_QUIC_TLS_got_transport_params_fn * OSSL_FUNC_SSL_QUIC_TLS_got_transport_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_SSL_QUIC_TLS_got_transport_params_fn *)opf->function; }



typedef int(OSSL_FUNC_SSL_QUIC_TLS_alert_fn) (SSL *s, unsigned char alert_code, void *arg); static __attribute__((unused)) inline OSSL_FUNC_SSL_QUIC_TLS_alert_fn * OSSL_FUNC_SSL_QUIC_TLS_alert(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_SSL_QUIC_TLS_alert_fn *)opf->function; }
typedef void *(OSSL_FUNC_digest_newctx_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_digest_newctx_fn * OSSL_FUNC_digest_newctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_digest_newctx_fn *)opf->function; }
typedef int(OSSL_FUNC_digest_init_fn) (void *dctx, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_digest_init_fn * OSSL_FUNC_digest_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_digest_init_fn *)opf->function; }
typedef int(OSSL_FUNC_digest_update_fn) (void *dctx, const unsigned char *in, size_t inl); static __attribute__((unused)) inline OSSL_FUNC_digest_update_fn * OSSL_FUNC_digest_update(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_digest_update_fn *)opf->function; }

typedef int(OSSL_FUNC_digest_final_fn) (void *dctx, unsigned char *out, size_t *outl, size_t outsz); static __attribute__((unused)) inline OSSL_FUNC_digest_final_fn * OSSL_FUNC_digest_final(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_digest_final_fn *)opf->function; }


typedef int(OSSL_FUNC_digest_squeeze_fn) (void *dctx, unsigned char *out, size_t *outl, size_t outsz); static __attribute__((unused)) inline OSSL_FUNC_digest_squeeze_fn * OSSL_FUNC_digest_squeeze(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_digest_squeeze_fn *)opf->function; }


typedef int(OSSL_FUNC_digest_digest_fn) (void *provctx, const unsigned char *in, size_t inl, unsigned char *out, size_t *outl, size_t outsz); static __attribute__((unused)) inline OSSL_FUNC_digest_digest_fn * OSSL_FUNC_digest_digest(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_digest_digest_fn *)opf->function; }



typedef void(OSSL_FUNC_digest_freectx_fn) (void *dctx); static __attribute__((unused)) inline OSSL_FUNC_digest_freectx_fn * OSSL_FUNC_digest_freectx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_digest_freectx_fn *)opf->function; }
typedef void *(OSSL_FUNC_digest_dupctx_fn) (void *dctx); static __attribute__((unused)) inline OSSL_FUNC_digest_dupctx_fn * OSSL_FUNC_digest_dupctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_digest_dupctx_fn *)opf->function; }
typedef void(OSSL_FUNC_digest_copyctx_fn) (void *outctx, void *inctx); static __attribute__((unused)) inline OSSL_FUNC_digest_copyctx_fn * OSSL_FUNC_digest_copyctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_digest_copyctx_fn *)opf->function; }

typedef int(OSSL_FUNC_digest_get_params_fn) (OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_digest_get_params_fn * OSSL_FUNC_digest_get_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_digest_get_params_fn *)opf->function; }
typedef int(OSSL_FUNC_digest_set_ctx_params_fn) (void *vctx, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_digest_set_ctx_params_fn * OSSL_FUNC_digest_set_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_digest_set_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_digest_get_ctx_params_fn) (void *vctx, OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_digest_get_ctx_params_fn * OSSL_FUNC_digest_get_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_digest_get_ctx_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_digest_gettable_params_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_digest_gettable_params_fn * OSSL_FUNC_digest_gettable_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_digest_gettable_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_digest_settable_ctx_params_fn) (void *dctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_digest_settable_ctx_params_fn * OSSL_FUNC_digest_settable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_digest_settable_ctx_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_digest_gettable_ctx_params_fn) (void *dctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_digest_gettable_ctx_params_fn * OSSL_FUNC_digest_gettable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_digest_gettable_ctx_params_fn *)opf->function; }
typedef void *(OSSL_FUNC_cipher_newctx_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_cipher_newctx_fn * OSSL_FUNC_cipher_newctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_newctx_fn *)opf->function; }
typedef int(OSSL_FUNC_cipher_encrypt_init_fn) (void *cctx, const unsigned char *key, size_t keylen, const unsigned char *iv, size_t ivlen, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_cipher_encrypt_init_fn * OSSL_FUNC_cipher_encrypt_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_encrypt_init_fn *)opf->function; }
typedef int(OSSL_FUNC_cipher_decrypt_init_fn) (void *cctx, const unsigned char *key, size_t keylen, const unsigned char *iv, size_t ivlen, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_cipher_decrypt_init_fn * OSSL_FUNC_cipher_decrypt_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_decrypt_init_fn *)opf->function; }
typedef int(OSSL_FUNC_cipher_update_fn) (void *cctx, unsigned char *out, size_t *outl, size_t outsize, const unsigned char *in, size_t inl); static __attribute__((unused)) inline OSSL_FUNC_cipher_update_fn * OSSL_FUNC_cipher_update(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_update_fn *)opf->function; }



typedef int(OSSL_FUNC_cipher_final_fn) (void *cctx, unsigned char *out, size_t *outl, size_t outsize); static __attribute__((unused)) inline OSSL_FUNC_cipher_final_fn * OSSL_FUNC_cipher_final(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_final_fn *)opf->function; }


typedef int(OSSL_FUNC_cipher_cipher_fn) (void *cctx, unsigned char *out, size_t *outl, size_t outsize, const unsigned char *in, size_t inl); static __attribute__((unused)) inline OSSL_FUNC_cipher_cipher_fn * OSSL_FUNC_cipher_cipher(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_cipher_fn *)opf->function; }



typedef int(OSSL_FUNC_cipher_pipeline_encrypt_init_fn) (void *cctx, const unsigned char *key, size_t keylen, size_t numpipes, const unsigned char **iv, size_t ivlen, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_cipher_pipeline_encrypt_init_fn * OSSL_FUNC_cipher_pipeline_encrypt_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_pipeline_encrypt_init_fn *)opf->function; }




typedef int(OSSL_FUNC_cipher_pipeline_decrypt_init_fn) (void *cctx, const unsigned char *key, size_t keylen, size_t numpipes, const unsigned char **iv, size_t ivlen, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_cipher_pipeline_decrypt_init_fn * OSSL_FUNC_cipher_pipeline_decrypt_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_pipeline_decrypt_init_fn *)opf->function; }




typedef int(OSSL_FUNC_cipher_pipeline_update_fn) (void *cctx, size_t numpipes, unsigned char **out, size_t *outl, const size_t *outsize, const unsigned char **in, const size_t *inl); static __attribute__((unused)) inline OSSL_FUNC_cipher_pipeline_update_fn * OSSL_FUNC_cipher_pipeline_update(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_pipeline_update_fn *)opf->function; }



typedef int(OSSL_FUNC_cipher_pipeline_final_fn) (void *cctx, size_t numpipes, unsigned char **out, size_t *outl, const size_t *outsize); static __attribute__((unused)) inline OSSL_FUNC_cipher_pipeline_final_fn * OSSL_FUNC_cipher_pipeline_final(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_pipeline_final_fn *)opf->function; }


typedef void(OSSL_FUNC_cipher_freectx_fn) (void *cctx); static __attribute__((unused)) inline OSSL_FUNC_cipher_freectx_fn * OSSL_FUNC_cipher_freectx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_freectx_fn *)opf->function; }
typedef void *(OSSL_FUNC_cipher_dupctx_fn) (void *cctx); static __attribute__((unused)) inline OSSL_FUNC_cipher_dupctx_fn * OSSL_FUNC_cipher_dupctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_dupctx_fn *)opf->function; }
typedef int(OSSL_FUNC_cipher_get_params_fn) (OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_cipher_get_params_fn * OSSL_FUNC_cipher_get_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_get_params_fn *)opf->function; }
typedef int(OSSL_FUNC_cipher_get_ctx_params_fn) (void *cctx, OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_cipher_get_ctx_params_fn * OSSL_FUNC_cipher_get_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_get_ctx_params_fn *)opf->function; }
typedef int(OSSL_FUNC_cipher_set_ctx_params_fn) (void *cctx, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_cipher_set_ctx_params_fn * OSSL_FUNC_cipher_set_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_set_ctx_params_fn *)opf->function; }
typedef const OSSL_PARAM *(OSSL_FUNC_cipher_gettable_params_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_cipher_gettable_params_fn * OSSL_FUNC_cipher_gettable_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_gettable_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_cipher_settable_ctx_params_fn) (void *cctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_cipher_settable_ctx_params_fn * OSSL_FUNC_cipher_settable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_settable_ctx_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_cipher_gettable_ctx_params_fn) (void *cctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_cipher_gettable_ctx_params_fn * OSSL_FUNC_cipher_gettable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_gettable_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_cipher_encrypt_skey_init_fn) (void *cctx, void *skeydata, const unsigned char *iv, size_t ivlen, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_cipher_encrypt_skey_init_fn * OSSL_FUNC_cipher_encrypt_skey_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_encrypt_skey_init_fn *)opf->function; }
typedef int(OSSL_FUNC_cipher_decrypt_skey_init_fn) (void *cctx, void *skeydata, const unsigned char *iv, size_t ivlen, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_cipher_decrypt_skey_init_fn * OSSL_FUNC_cipher_decrypt_skey_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_cipher_decrypt_skey_init_fn *)opf->function; }
typedef void *(OSSL_FUNC_mac_newctx_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_mac_newctx_fn * OSSL_FUNC_mac_newctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_mac_newctx_fn *)opf->function; }
typedef void *(OSSL_FUNC_mac_dupctx_fn) (void *src); static __attribute__((unused)) inline OSSL_FUNC_mac_dupctx_fn * OSSL_FUNC_mac_dupctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_mac_dupctx_fn *)opf->function; }
typedef void(OSSL_FUNC_mac_freectx_fn) (void *mctx); static __attribute__((unused)) inline OSSL_FUNC_mac_freectx_fn * OSSL_FUNC_mac_freectx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_mac_freectx_fn *)opf->function; }
typedef int(OSSL_FUNC_mac_init_fn) (void *mctx, const unsigned char *key, size_t keylen, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_mac_init_fn * OSSL_FUNC_mac_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_mac_init_fn *)opf->function; }
typedef int(OSSL_FUNC_mac_update_fn) (void *mctx, const unsigned char *in, size_t inl); static __attribute__((unused)) inline OSSL_FUNC_mac_update_fn * OSSL_FUNC_mac_update(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_mac_update_fn *)opf->function; }

typedef int(OSSL_FUNC_mac_final_fn) (void *mctx, unsigned char *out, size_t *outl, size_t outsize); static __attribute__((unused)) inline OSSL_FUNC_mac_final_fn * OSSL_FUNC_mac_final(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_mac_final_fn *)opf->function; }


typedef const OSSL_PARAM *(OSSL_FUNC_mac_gettable_params_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_mac_gettable_params_fn * OSSL_FUNC_mac_gettable_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_mac_gettable_params_fn *)opf->function; }
typedef const OSSL_PARAM *(OSSL_FUNC_mac_gettable_ctx_params_fn) (void *mctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_mac_gettable_ctx_params_fn * OSSL_FUNC_mac_gettable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_mac_gettable_ctx_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_mac_settable_ctx_params_fn) (void *mctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_mac_settable_ctx_params_fn * OSSL_FUNC_mac_settable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_mac_settable_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_mac_get_params_fn) (OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_mac_get_params_fn * OSSL_FUNC_mac_get_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_mac_get_params_fn *)opf->function; }
typedef int(OSSL_FUNC_mac_get_ctx_params_fn) (void *mctx, OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_mac_get_ctx_params_fn * OSSL_FUNC_mac_get_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_mac_get_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_mac_set_ctx_params_fn) (void *mctx, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_mac_set_ctx_params_fn * OSSL_FUNC_mac_set_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_mac_set_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_mac_init_skey_fn) (void *mctx, void *key, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_mac_init_skey_fn * OSSL_FUNC_mac_init_skey(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_mac_init_skey_fn *)opf->function; }
typedef void(OSSL_FUNC_skeymgmt_free_fn) (void *keydata); static __attribute__((unused)) inline OSSL_FUNC_skeymgmt_free_fn * OSSL_FUNC_skeymgmt_free(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_skeymgmt_free_fn *)opf->function; }
typedef const OSSL_PARAM *(OSSL_FUNC_skeymgmt_imp_settable_params_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_skeymgmt_imp_settable_params_fn * OSSL_FUNC_skeymgmt_imp_settable_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_skeymgmt_imp_settable_params_fn *)opf->function; }

typedef void *(OSSL_FUNC_skeymgmt_import_fn) (void *provctx, int selection, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_skeymgmt_import_fn * OSSL_FUNC_skeymgmt_import(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_skeymgmt_import_fn *)opf->function; }
typedef int(OSSL_FUNC_skeymgmt_export_fn) (void *keydata, int selection, OSSL_CALLBACK *param_cb, void *cbarg); static __attribute__((unused)) inline OSSL_FUNC_skeymgmt_export_fn * OSSL_FUNC_skeymgmt_export(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_skeymgmt_export_fn *)opf->function; }


typedef const OSSL_PARAM *(OSSL_FUNC_skeymgmt_gen_settable_params_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_skeymgmt_gen_settable_params_fn * OSSL_FUNC_skeymgmt_gen_settable_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_skeymgmt_gen_settable_params_fn *)opf->function; }

typedef void *(OSSL_FUNC_skeymgmt_generate_fn) (void *provctx, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_skeymgmt_generate_fn * OSSL_FUNC_skeymgmt_generate(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_skeymgmt_generate_fn *)opf->function; }
typedef const char *(OSSL_FUNC_skeymgmt_get_key_id_fn) (void *keydata); static __attribute__((unused)) inline OSSL_FUNC_skeymgmt_get_key_id_fn * OSSL_FUNC_skeymgmt_get_key_id(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_skeymgmt_get_key_id_fn *)opf->function; }
typedef void *(OSSL_FUNC_kdf_newctx_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_kdf_newctx_fn * OSSL_FUNC_kdf_newctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kdf_newctx_fn *)opf->function; }
typedef void *(OSSL_FUNC_kdf_dupctx_fn) (void *src); static __attribute__((unused)) inline OSSL_FUNC_kdf_dupctx_fn * OSSL_FUNC_kdf_dupctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kdf_dupctx_fn *)opf->function; }
typedef void(OSSL_FUNC_kdf_freectx_fn) (void *kctx); static __attribute__((unused)) inline OSSL_FUNC_kdf_freectx_fn * OSSL_FUNC_kdf_freectx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kdf_freectx_fn *)opf->function; }
typedef void(OSSL_FUNC_kdf_reset_fn) (void *kctx); static __attribute__((unused)) inline OSSL_FUNC_kdf_reset_fn * OSSL_FUNC_kdf_reset(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kdf_reset_fn *)opf->function; }
typedef int(OSSL_FUNC_kdf_derive_fn) (void *kctx, unsigned char *key, size_t keylen, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_kdf_derive_fn * OSSL_FUNC_kdf_derive(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kdf_derive_fn *)opf->function; }
typedef const OSSL_PARAM *(OSSL_FUNC_kdf_gettable_params_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_kdf_gettable_params_fn * OSSL_FUNC_kdf_gettable_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kdf_gettable_params_fn *)opf->function; }
typedef const OSSL_PARAM *(OSSL_FUNC_kdf_gettable_ctx_params_fn) (void *kctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_kdf_gettable_ctx_params_fn * OSSL_FUNC_kdf_gettable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kdf_gettable_ctx_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_kdf_settable_ctx_params_fn) (void *kctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_kdf_settable_ctx_params_fn * OSSL_FUNC_kdf_settable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kdf_settable_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_kdf_get_params_fn) (OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_kdf_get_params_fn * OSSL_FUNC_kdf_get_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kdf_get_params_fn *)opf->function; }
typedef int(OSSL_FUNC_kdf_get_ctx_params_fn) (void *kctx, OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_kdf_get_ctx_params_fn * OSSL_FUNC_kdf_get_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kdf_get_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_kdf_set_ctx_params_fn) (void *kctx, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_kdf_set_ctx_params_fn * OSSL_FUNC_kdf_set_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kdf_set_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_kdf_set_skey_fn) (void *kctx, void *skeydata, const char *paramname); static __attribute__((unused)) inline OSSL_FUNC_kdf_set_skey_fn * OSSL_FUNC_kdf_set_skey(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kdf_set_skey_fn *)opf->function; }

typedef void *(OSSL_FUNC_kdf_derive_skey_fn) (void *ctx, const char *key_type, void *provctx, OSSL_FUNC_skeymgmt_import_fn *import, size_t keylen, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_kdf_derive_skey_fn * OSSL_FUNC_kdf_derive_skey(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kdf_derive_skey_fn *)opf->function; }
typedef void *(OSSL_FUNC_rand_newctx_fn) (void *provctx, void *parent, const OSSL_DISPATCH *parent_calls); static __attribute__((unused)) inline OSSL_FUNC_rand_newctx_fn * OSSL_FUNC_rand_newctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_newctx_fn *)opf->function; }


typedef void(OSSL_FUNC_rand_freectx_fn) (void *vctx); static __attribute__((unused)) inline OSSL_FUNC_rand_freectx_fn * OSSL_FUNC_rand_freectx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_freectx_fn *)opf->function; }
typedef int(OSSL_FUNC_rand_instantiate_fn) (void *vdrbg, unsigned int strength, int prediction_resistance, const unsigned char *pstr, size_t pstr_len, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_rand_instantiate_fn * OSSL_FUNC_rand_instantiate(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_instantiate_fn *)opf->function; }




typedef int(OSSL_FUNC_rand_uninstantiate_fn) (void *vdrbg); static __attribute__((unused)) inline OSSL_FUNC_rand_uninstantiate_fn * OSSL_FUNC_rand_uninstantiate(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_uninstantiate_fn *)opf->function; }
typedef int(OSSL_FUNC_rand_generate_fn) (void *vctx, unsigned char *out, size_t outlen, unsigned int strength, int prediction_resistance, const unsigned char *addin, size_t addin_len); static __attribute__((unused)) inline OSSL_FUNC_rand_generate_fn * OSSL_FUNC_rand_generate(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_generate_fn *)opf->function; }



typedef int(OSSL_FUNC_rand_reseed_fn) (void *vctx, int prediction_resistance, const unsigned char *ent, size_t ent_len, const unsigned char *addin, size_t addin_len); static __attribute__((unused)) inline OSSL_FUNC_rand_reseed_fn * OSSL_FUNC_rand_reseed(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_reseed_fn *)opf->function; }



typedef size_t(OSSL_FUNC_rand_nonce_fn) (void *vctx, unsigned char *out, unsigned int strength, size_t min_noncelen, size_t max_noncelen); static __attribute__((unused)) inline OSSL_FUNC_rand_nonce_fn * OSSL_FUNC_rand_nonce(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_nonce_fn *)opf->function; }


typedef int(OSSL_FUNC_rand_enable_locking_fn) (void *vctx); static __attribute__((unused)) inline OSSL_FUNC_rand_enable_locking_fn * OSSL_FUNC_rand_enable_locking(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_enable_locking_fn *)opf->function; }
typedef int(OSSL_FUNC_rand_lock_fn) (void *vctx); static __attribute__((unused)) inline OSSL_FUNC_rand_lock_fn * OSSL_FUNC_rand_lock(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_lock_fn *)opf->function; }
typedef void(OSSL_FUNC_rand_unlock_fn) (void *vctx); static __attribute__((unused)) inline OSSL_FUNC_rand_unlock_fn * OSSL_FUNC_rand_unlock(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_unlock_fn *)opf->function; }
typedef const OSSL_PARAM *(OSSL_FUNC_rand_gettable_params_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_rand_gettable_params_fn * OSSL_FUNC_rand_gettable_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_gettable_params_fn *)opf->function; }
typedef const OSSL_PARAM *(OSSL_FUNC_rand_gettable_ctx_params_fn) (void *vctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_rand_gettable_ctx_params_fn * OSSL_FUNC_rand_gettable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_gettable_ctx_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_rand_settable_ctx_params_fn) (void *vctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_rand_settable_ctx_params_fn * OSSL_FUNC_rand_settable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_settable_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_rand_get_params_fn) (OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_rand_get_params_fn * OSSL_FUNC_rand_get_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_get_params_fn *)opf->function; }
typedef int(OSSL_FUNC_rand_get_ctx_params_fn) (void *vctx, OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_rand_get_ctx_params_fn * OSSL_FUNC_rand_get_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_get_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_rand_set_ctx_params_fn) (void *vctx, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_rand_set_ctx_params_fn * OSSL_FUNC_rand_set_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_set_ctx_params_fn *)opf->function; }

typedef void(OSSL_FUNC_rand_set_callbacks_fn) (void *vctx, OSSL_INOUT_CALLBACK *get_entropy, OSSL_CALLBACK *cleanup_entropy, OSSL_INOUT_CALLBACK *get_nonce, OSSL_CALLBACK *cleanup_nonce, void *arg); static __attribute__((unused)) inline OSSL_FUNC_rand_set_callbacks_fn * OSSL_FUNC_rand_set_callbacks(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_set_callbacks_fn *)opf->function; }




typedef int(OSSL_FUNC_rand_verify_zeroization_fn) (void *vctx); static __attribute__((unused)) inline OSSL_FUNC_rand_verify_zeroization_fn * OSSL_FUNC_rand_verify_zeroization(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_verify_zeroization_fn *)opf->function; }

typedef size_t(OSSL_FUNC_rand_get_seed_fn) (void *vctx, unsigned char **buffer, int entropy, size_t min_len, size_t max_len, int prediction_resistance, const unsigned char *adin, size_t adin_len); static __attribute__((unused)) inline OSSL_FUNC_rand_get_seed_fn * OSSL_FUNC_rand_get_seed(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_get_seed_fn *)opf->function; }




typedef void(OSSL_FUNC_rand_clear_seed_fn) (void *vctx, unsigned char *buffer, size_t b_len); static __attribute__((unused)) inline OSSL_FUNC_rand_clear_seed_fn * OSSL_FUNC_rand_clear_seed(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_rand_clear_seed_fn *)opf->function; }
typedef void *(OSSL_FUNC_keymgmt_new_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_new_fn * OSSL_FUNC_keymgmt_new(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_new_fn *)opf->function; }
typedef void *(OSSL_FUNC_keymgmt_gen_init_fn) (void *provctx, int selection, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_gen_init_fn * OSSL_FUNC_keymgmt_gen_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_gen_init_fn *)opf->function; }

typedef int(OSSL_FUNC_keymgmt_gen_set_template_fn) (void *genctx, void *templ); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_gen_set_template_fn * OSSL_FUNC_keymgmt_gen_set_template(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_gen_set_template_fn *)opf->function; }

typedef int(OSSL_FUNC_keymgmt_gen_set_params_fn) (void *genctx, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_gen_set_params_fn * OSSL_FUNC_keymgmt_gen_set_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_gen_set_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_keymgmt_gen_settable_params_fn) (void *genctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_gen_settable_params_fn * OSSL_FUNC_keymgmt_gen_settable_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_gen_settable_params_fn *)opf->function; }


typedef int(OSSL_FUNC_keymgmt_gen_get_params_fn) (void *genctx, OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_gen_get_params_fn * OSSL_FUNC_keymgmt_gen_get_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_gen_get_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_keymgmt_gen_gettable_params_fn) (void *genctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_gen_gettable_params_fn * OSSL_FUNC_keymgmt_gen_gettable_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_gen_gettable_params_fn *)opf->function; }

typedef void *(OSSL_FUNC_keymgmt_gen_fn) (void *genctx, OSSL_CALLBACK *cb, void *cbarg); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_gen_fn * OSSL_FUNC_keymgmt_gen(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_gen_fn *)opf->function; }

typedef void(OSSL_FUNC_keymgmt_gen_cleanup_fn) (void *genctx); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_gen_cleanup_fn * OSSL_FUNC_keymgmt_gen_cleanup(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_gen_cleanup_fn *)opf->function; }



typedef void *(OSSL_FUNC_keymgmt_load_fn) (const void *reference, size_t reference_sz); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_load_fn * OSSL_FUNC_keymgmt_load(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_load_fn *)opf->function; }




typedef void(OSSL_FUNC_keymgmt_free_fn) (void *keydata); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_free_fn * OSSL_FUNC_keymgmt_free(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_free_fn *)opf->function; }




typedef int(OSSL_FUNC_keymgmt_get_params_fn) (void *keydata, OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_get_params_fn * OSSL_FUNC_keymgmt_get_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_get_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_keymgmt_gettable_params_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_gettable_params_fn * OSSL_FUNC_keymgmt_gettable_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_gettable_params_fn *)opf->function; }




typedef int(OSSL_FUNC_keymgmt_set_params_fn) (void *keydata, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_set_params_fn * OSSL_FUNC_keymgmt_set_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_set_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_keymgmt_settable_params_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_settable_params_fn * OSSL_FUNC_keymgmt_settable_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_settable_params_fn *)opf->function; }




typedef const char *(OSSL_FUNC_keymgmt_query_operation_name_fn) (int operation_id); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_query_operation_name_fn * OSSL_FUNC_keymgmt_query_operation_name(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_query_operation_name_fn *)opf->function; }




typedef int(OSSL_FUNC_keymgmt_has_fn) (const void *keydata, int selection); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_has_fn * OSSL_FUNC_keymgmt_has(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_has_fn *)opf->function; }



typedef int(OSSL_FUNC_keymgmt_validate_fn) (const void *keydata, int selection, int checktype); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_validate_fn * OSSL_FUNC_keymgmt_validate(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_validate_fn *)opf->function; }



typedef int(OSSL_FUNC_keymgmt_match_fn) (const void *keydata1, const void *keydata2, int selection); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_match_fn * OSSL_FUNC_keymgmt_match(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_match_fn *)opf->function; }
typedef int(OSSL_FUNC_keymgmt_import_fn) (void *keydata, int selection, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_import_fn * OSSL_FUNC_keymgmt_import(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_import_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_keymgmt_import_types_fn) (int selection); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_import_types_fn * OSSL_FUNC_keymgmt_import_types(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_import_types_fn *)opf->function; }

typedef int(OSSL_FUNC_keymgmt_export_fn) (void *keydata, int selection, OSSL_CALLBACK *param_cb, void *cbarg); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_export_fn * OSSL_FUNC_keymgmt_export(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_export_fn *)opf->function; }


typedef const OSSL_PARAM *(OSSL_FUNC_keymgmt_export_types_fn) (int selection); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_export_types_fn * OSSL_FUNC_keymgmt_export_types(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_export_types_fn *)opf->function; }




typedef void *(OSSL_FUNC_keymgmt_dup_fn) (const void *keydata_from, int selection); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_dup_fn * OSSL_FUNC_keymgmt_dup(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_dup_fn *)opf->function; }





typedef const OSSL_PARAM *(OSSL_FUNC_keymgmt_import_types_ex_fn) (void *provctx, int selection); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_import_types_ex_fn * OSSL_FUNC_keymgmt_import_types_ex(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_import_types_ex_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_keymgmt_export_types_ex_fn) (void *provctx, int selection); static __attribute__((unused)) inline OSSL_FUNC_keymgmt_export_types_ex_fn * OSSL_FUNC_keymgmt_export_types_ex(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keymgmt_export_types_ex_fn *)opf->function; }
typedef void *(OSSL_FUNC_keyexch_newctx_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_keyexch_newctx_fn * OSSL_FUNC_keyexch_newctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keyexch_newctx_fn *)opf->function; }
typedef int(OSSL_FUNC_keyexch_init_fn) (void *ctx, void *provkey, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_keyexch_init_fn * OSSL_FUNC_keyexch_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keyexch_init_fn *)opf->function; }
typedef int(OSSL_FUNC_keyexch_derive_fn) (void *ctx, unsigned char *secret, size_t *secretlen, size_t outlen); static __attribute__((unused)) inline OSSL_FUNC_keyexch_derive_fn * OSSL_FUNC_keyexch_derive(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keyexch_derive_fn *)opf->function; }
typedef int(OSSL_FUNC_keyexch_set_peer_fn) (void *ctx, void *provkey); static __attribute__((unused)) inline OSSL_FUNC_keyexch_set_peer_fn * OSSL_FUNC_keyexch_set_peer(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keyexch_set_peer_fn *)opf->function; }
typedef void(OSSL_FUNC_keyexch_freectx_fn) (void *ctx); static __attribute__((unused)) inline OSSL_FUNC_keyexch_freectx_fn * OSSL_FUNC_keyexch_freectx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keyexch_freectx_fn *)opf->function; }
typedef void *(OSSL_FUNC_keyexch_dupctx_fn) (void *ctx); static __attribute__((unused)) inline OSSL_FUNC_keyexch_dupctx_fn * OSSL_FUNC_keyexch_dupctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keyexch_dupctx_fn *)opf->function; }
typedef int(OSSL_FUNC_keyexch_set_ctx_params_fn) (void *ctx, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_keyexch_set_ctx_params_fn * OSSL_FUNC_keyexch_set_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keyexch_set_ctx_params_fn *)opf->function; }
typedef const OSSL_PARAM *(OSSL_FUNC_keyexch_settable_ctx_params_fn) (void *ctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_keyexch_settable_ctx_params_fn * OSSL_FUNC_keyexch_settable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keyexch_settable_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_keyexch_get_ctx_params_fn) (void *ctx, OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_keyexch_get_ctx_params_fn * OSSL_FUNC_keyexch_get_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keyexch_get_ctx_params_fn *)opf->function; }
typedef const OSSL_PARAM *(OSSL_FUNC_keyexch_gettable_ctx_params_fn) (void *ctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_keyexch_gettable_ctx_params_fn * OSSL_FUNC_keyexch_gettable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keyexch_gettable_ctx_params_fn *)opf->function; }

typedef void *(OSSL_FUNC_keyexch_derive_skey_fn) (void *ctx, const char *key_type, void *provctx, OSSL_FUNC_skeymgmt_import_fn *import, size_t keylen, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_keyexch_derive_skey_fn * OSSL_FUNC_keyexch_derive_skey(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_keyexch_derive_skey_fn *)opf->function; }
typedef void *(OSSL_FUNC_signature_newctx_fn) (void *provctx, const char *propq); static __attribute__((unused)) inline OSSL_FUNC_signature_newctx_fn * OSSL_FUNC_signature_newctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_newctx_fn *)opf->function; }
typedef int(OSSL_FUNC_signature_sign_init_fn) (void *ctx, void *provkey, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_signature_sign_init_fn * OSSL_FUNC_signature_sign_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_sign_init_fn *)opf->function; }
typedef int(OSSL_FUNC_signature_sign_fn) (void *ctx, unsigned char *sig, size_t *siglen, size_t sigsize, const unsigned char *tbs, size_t tbslen); static __attribute__((unused)) inline OSSL_FUNC_signature_sign_fn * OSSL_FUNC_signature_sign(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_sign_fn *)opf->function; }
typedef int(OSSL_FUNC_signature_sign_message_init_fn) (void *ctx, void *provkey, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_signature_sign_message_init_fn * OSSL_FUNC_signature_sign_message_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_sign_message_init_fn *)opf->function; }

typedef int(OSSL_FUNC_signature_sign_message_update_fn) (void *ctx, const unsigned char *in, size_t inlen); static __attribute__((unused)) inline OSSL_FUNC_signature_sign_message_update_fn * OSSL_FUNC_signature_sign_message_update(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_sign_message_update_fn *)opf->function; }

typedef int(OSSL_FUNC_signature_sign_message_final_fn) (void *ctx, unsigned char *sig, size_t *siglen, size_t sigsize); static __attribute__((unused)) inline OSSL_FUNC_signature_sign_message_final_fn * OSSL_FUNC_signature_sign_message_final(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_sign_message_final_fn *)opf->function; }


typedef int(OSSL_FUNC_signature_verify_init_fn) (void *ctx, void *provkey, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_signature_verify_init_fn * OSSL_FUNC_signature_verify_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_verify_init_fn *)opf->function; }
typedef int(OSSL_FUNC_signature_verify_fn) (void *ctx, const unsigned char *sig, size_t siglen, const unsigned char *tbs, size_t tbslen); static __attribute__((unused)) inline OSSL_FUNC_signature_verify_fn * OSSL_FUNC_signature_verify(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_verify_fn *)opf->function; }
typedef int(OSSL_FUNC_signature_verify_message_init_fn) (void *ctx, void *provkey, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_signature_verify_message_init_fn * OSSL_FUNC_signature_verify_message_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_verify_message_init_fn *)opf->function; }

typedef int(OSSL_FUNC_signature_verify_message_update_fn) (void *ctx, const unsigned char *in, size_t inlen); static __attribute__((unused)) inline OSSL_FUNC_signature_verify_message_update_fn * OSSL_FUNC_signature_verify_message_update(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_verify_message_update_fn *)opf->function; }





typedef int(OSSL_FUNC_signature_verify_message_final_fn) (void *ctx); static __attribute__((unused)) inline OSSL_FUNC_signature_verify_message_final_fn * OSSL_FUNC_signature_verify_message_final(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_verify_message_final_fn *)opf->function; }
typedef int(OSSL_FUNC_signature_verify_recover_init_fn) (void *ctx, void *provkey, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_signature_verify_recover_init_fn * OSSL_FUNC_signature_verify_recover_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_verify_recover_init_fn *)opf->function; }

typedef int(OSSL_FUNC_signature_verify_recover_fn) (void *ctx, unsigned char *rout, size_t *routlen, size_t routsize, const unsigned char *sig, size_t siglen); static __attribute__((unused)) inline OSSL_FUNC_signature_verify_recover_fn * OSSL_FUNC_signature_verify_recover(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_verify_recover_fn *)opf->function; }


typedef int(OSSL_FUNC_signature_digest_sign_init_fn) (void *ctx, const char *mdname, void *provkey, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_signature_digest_sign_init_fn * OSSL_FUNC_signature_digest_sign_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_digest_sign_init_fn *)opf->function; }


typedef int(OSSL_FUNC_signature_digest_sign_update_fn) (void *ctx, const unsigned char *data, size_t datalen); static __attribute__((unused)) inline OSSL_FUNC_signature_digest_sign_update_fn * OSSL_FUNC_signature_digest_sign_update(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_digest_sign_update_fn *)opf->function; }

typedef int(OSSL_FUNC_signature_digest_sign_final_fn) (void *ctx, unsigned char *sig, size_t *siglen, size_t sigsize); static __attribute__((unused)) inline OSSL_FUNC_signature_digest_sign_final_fn * OSSL_FUNC_signature_digest_sign_final(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_digest_sign_final_fn *)opf->function; }


typedef int(OSSL_FUNC_signature_digest_sign_fn) (void *ctx, unsigned char *sigret, size_t *siglen, size_t sigsize, const unsigned char *tbs, size_t tbslen); static __attribute__((unused)) inline OSSL_FUNC_signature_digest_sign_fn * OSSL_FUNC_signature_digest_sign(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_digest_sign_fn *)opf->function; }


typedef int(OSSL_FUNC_signature_digest_verify_init_fn) (void *ctx, const char *mdname, void *provkey, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_signature_digest_verify_init_fn * OSSL_FUNC_signature_digest_verify_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_digest_verify_init_fn *)opf->function; }


typedef int(OSSL_FUNC_signature_digest_verify_update_fn) (void *ctx, const unsigned char *data, size_t datalen); static __attribute__((unused)) inline OSSL_FUNC_signature_digest_verify_update_fn * OSSL_FUNC_signature_digest_verify_update(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_digest_verify_update_fn *)opf->function; }

typedef int(OSSL_FUNC_signature_digest_verify_final_fn) (void *ctx, const unsigned char *sig, size_t siglen); static __attribute__((unused)) inline OSSL_FUNC_signature_digest_verify_final_fn * OSSL_FUNC_signature_digest_verify_final(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_digest_verify_final_fn *)opf->function; }

typedef int(OSSL_FUNC_signature_digest_verify_fn) (void *ctx, const unsigned char *sig, size_t siglen, const unsigned char *tbs, size_t tbslen); static __attribute__((unused)) inline OSSL_FUNC_signature_digest_verify_fn * OSSL_FUNC_signature_digest_verify(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_digest_verify_fn *)opf->function; }


typedef void(OSSL_FUNC_signature_freectx_fn) (void *ctx); static __attribute__((unused)) inline OSSL_FUNC_signature_freectx_fn * OSSL_FUNC_signature_freectx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_freectx_fn *)opf->function; }
typedef void *(OSSL_FUNC_signature_dupctx_fn) (void *ctx); static __attribute__((unused)) inline OSSL_FUNC_signature_dupctx_fn * OSSL_FUNC_signature_dupctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_dupctx_fn *)opf->function; }
typedef int(OSSL_FUNC_signature_get_ctx_params_fn) (void *ctx, OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_signature_get_ctx_params_fn * OSSL_FUNC_signature_get_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_get_ctx_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_signature_gettable_ctx_params_fn) (void *ctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_signature_gettable_ctx_params_fn * OSSL_FUNC_signature_gettable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_gettable_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_signature_set_ctx_params_fn) (void *ctx, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_signature_set_ctx_params_fn * OSSL_FUNC_signature_set_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_set_ctx_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_signature_settable_ctx_params_fn) (void *ctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_signature_settable_ctx_params_fn * OSSL_FUNC_signature_settable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_settable_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_signature_get_ctx_md_params_fn) (void *ctx, OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_signature_get_ctx_md_params_fn * OSSL_FUNC_signature_get_ctx_md_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_get_ctx_md_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_signature_gettable_ctx_md_params_fn) (void *ctx); static __attribute__((unused)) inline OSSL_FUNC_signature_gettable_ctx_md_params_fn * OSSL_FUNC_signature_gettable_ctx_md_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_gettable_ctx_md_params_fn *)opf->function; }

typedef int(OSSL_FUNC_signature_set_ctx_md_params_fn) (void *ctx, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_signature_set_ctx_md_params_fn * OSSL_FUNC_signature_set_ctx_md_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_set_ctx_md_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_signature_settable_ctx_md_params_fn) (void *ctx); static __attribute__((unused)) inline OSSL_FUNC_signature_settable_ctx_md_params_fn * OSSL_FUNC_signature_settable_ctx_md_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_settable_ctx_md_params_fn *)opf->function; }

typedef const char **(OSSL_FUNC_signature_query_key_types_fn) (void); static __attribute__((unused)) inline OSSL_FUNC_signature_query_key_types_fn * OSSL_FUNC_signature_query_key_types(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_signature_query_key_types_fn *)opf->function; }
typedef void *(OSSL_FUNC_asym_cipher_newctx_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_asym_cipher_newctx_fn * OSSL_FUNC_asym_cipher_newctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_asym_cipher_newctx_fn *)opf->function; }
typedef int(OSSL_FUNC_asym_cipher_encrypt_init_fn) (void *ctx, void *provkey, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_asym_cipher_encrypt_init_fn * OSSL_FUNC_asym_cipher_encrypt_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_asym_cipher_encrypt_init_fn *)opf->function; }
typedef int(OSSL_FUNC_asym_cipher_encrypt_fn) (void *ctx, unsigned char *out, size_t *outlen, size_t outsize, const unsigned char *in, size_t inlen); static __attribute__((unused)) inline OSSL_FUNC_asym_cipher_encrypt_fn * OSSL_FUNC_asym_cipher_encrypt(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_asym_cipher_encrypt_fn *)opf->function; }
typedef int(OSSL_FUNC_asym_cipher_decrypt_init_fn) (void *ctx, void *provkey, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_asym_cipher_decrypt_init_fn * OSSL_FUNC_asym_cipher_decrypt_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_asym_cipher_decrypt_init_fn *)opf->function; }
typedef int(OSSL_FUNC_asym_cipher_decrypt_fn) (void *ctx, unsigned char *out, size_t *outlen, size_t outsize, const unsigned char *in, size_t inlen); static __attribute__((unused)) inline OSSL_FUNC_asym_cipher_decrypt_fn * OSSL_FUNC_asym_cipher_decrypt(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_asym_cipher_decrypt_fn *)opf->function; }
typedef void(OSSL_FUNC_asym_cipher_freectx_fn) (void *ctx); static __attribute__((unused)) inline OSSL_FUNC_asym_cipher_freectx_fn * OSSL_FUNC_asym_cipher_freectx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_asym_cipher_freectx_fn *)opf->function; }
typedef void *(OSSL_FUNC_asym_cipher_dupctx_fn) (void *ctx); static __attribute__((unused)) inline OSSL_FUNC_asym_cipher_dupctx_fn * OSSL_FUNC_asym_cipher_dupctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_asym_cipher_dupctx_fn *)opf->function; }
typedef int(OSSL_FUNC_asym_cipher_get_ctx_params_fn) (void *ctx, OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_asym_cipher_get_ctx_params_fn * OSSL_FUNC_asym_cipher_get_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_asym_cipher_get_ctx_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_asym_cipher_gettable_ctx_params_fn) (void *ctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_asym_cipher_gettable_ctx_params_fn * OSSL_FUNC_asym_cipher_gettable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_asym_cipher_gettable_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_asym_cipher_set_ctx_params_fn) (void *ctx, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_asym_cipher_set_ctx_params_fn * OSSL_FUNC_asym_cipher_set_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_asym_cipher_set_ctx_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_asym_cipher_settable_ctx_params_fn) (void *ctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_asym_cipher_settable_ctx_params_fn * OSSL_FUNC_asym_cipher_settable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_asym_cipher_settable_ctx_params_fn *)opf->function; }
typedef void *(OSSL_FUNC_kem_newctx_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_kem_newctx_fn * OSSL_FUNC_kem_newctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kem_newctx_fn *)opf->function; }
typedef int(OSSL_FUNC_kem_encapsulate_init_fn) (void *ctx, void *provkey, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_kem_encapsulate_init_fn * OSSL_FUNC_kem_encapsulate_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kem_encapsulate_init_fn *)opf->function; }
typedef int(OSSL_FUNC_kem_auth_encapsulate_init_fn) (void *ctx, void *provkey, void *authprivkey, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_kem_auth_encapsulate_init_fn * OSSL_FUNC_kem_auth_encapsulate_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kem_auth_encapsulate_init_fn *)opf->function; }
typedef int(OSSL_FUNC_kem_encapsulate_fn) (void *ctx, unsigned char *out, size_t *outlen, unsigned char *secret, size_t *secretlen); static __attribute__((unused)) inline OSSL_FUNC_kem_encapsulate_fn * OSSL_FUNC_kem_encapsulate(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kem_encapsulate_fn *)opf->function; }
typedef int(OSSL_FUNC_kem_decapsulate_init_fn) (void *ctx, void *provkey, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_kem_decapsulate_init_fn * OSSL_FUNC_kem_decapsulate_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kem_decapsulate_init_fn *)opf->function; }
typedef int(OSSL_FUNC_kem_auth_decapsulate_init_fn) (void *ctx, void *provkey, void *authpubkey, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_kem_auth_decapsulate_init_fn * OSSL_FUNC_kem_auth_decapsulate_init(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kem_auth_decapsulate_init_fn *)opf->function; }
typedef int(OSSL_FUNC_kem_decapsulate_fn) (void *ctx, unsigned char *out, size_t *outlen, const unsigned char *in, size_t inlen); static __attribute__((unused)) inline OSSL_FUNC_kem_decapsulate_fn * OSSL_FUNC_kem_decapsulate(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kem_decapsulate_fn *)opf->function; }
typedef void(OSSL_FUNC_kem_freectx_fn) (void *ctx); static __attribute__((unused)) inline OSSL_FUNC_kem_freectx_fn * OSSL_FUNC_kem_freectx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kem_freectx_fn *)opf->function; }
typedef void *(OSSL_FUNC_kem_dupctx_fn) (void *ctx); static __attribute__((unused)) inline OSSL_FUNC_kem_dupctx_fn * OSSL_FUNC_kem_dupctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kem_dupctx_fn *)opf->function; }
typedef int(OSSL_FUNC_kem_get_ctx_params_fn) (void *ctx, OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_kem_get_ctx_params_fn * OSSL_FUNC_kem_get_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kem_get_ctx_params_fn *)opf->function; }
typedef const OSSL_PARAM *(OSSL_FUNC_kem_gettable_ctx_params_fn) (void *ctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_kem_gettable_ctx_params_fn * OSSL_FUNC_kem_gettable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kem_gettable_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_kem_set_ctx_params_fn) (void *ctx, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_kem_set_ctx_params_fn * OSSL_FUNC_kem_set_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kem_set_ctx_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_kem_settable_ctx_params_fn) (void *ctx, void *provctx); static __attribute__((unused)) inline OSSL_FUNC_kem_settable_ctx_params_fn * OSSL_FUNC_kem_settable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_kem_settable_ctx_params_fn *)opf->function; }
typedef void *(OSSL_FUNC_encoder_newctx_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_encoder_newctx_fn * OSSL_FUNC_encoder_newctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_encoder_newctx_fn *)opf->function; }
typedef void(OSSL_FUNC_encoder_freectx_fn) (void *ctx); static __attribute__((unused)) inline OSSL_FUNC_encoder_freectx_fn * OSSL_FUNC_encoder_freectx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_encoder_freectx_fn *)opf->function; }
typedef int(OSSL_FUNC_encoder_get_params_fn) (OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_encoder_get_params_fn * OSSL_FUNC_encoder_get_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_encoder_get_params_fn *)opf->function; }
typedef const OSSL_PARAM *(OSSL_FUNC_encoder_gettable_params_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_encoder_gettable_params_fn * OSSL_FUNC_encoder_gettable_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_encoder_gettable_params_fn *)opf->function; }

typedef int(OSSL_FUNC_encoder_set_ctx_params_fn) (void *ctx, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_encoder_set_ctx_params_fn * OSSL_FUNC_encoder_set_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_encoder_set_ctx_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_encoder_settable_ctx_params_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_encoder_settable_ctx_params_fn * OSSL_FUNC_encoder_settable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_encoder_settable_ctx_params_fn *)opf->function; }


typedef int(OSSL_FUNC_encoder_does_selection_fn) (void *provctx, int selection); static __attribute__((unused)) inline OSSL_FUNC_encoder_does_selection_fn * OSSL_FUNC_encoder_does_selection(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_encoder_does_selection_fn *)opf->function; }

typedef int(OSSL_FUNC_encoder_encode_fn) (void *ctx, OSSL_CORE_BIO *out, const void *obj_raw, const OSSL_PARAM obj_abstract[], int selection, OSSL_PASSPHRASE_CALLBACK *cb, void *cbarg); static __attribute__((unused)) inline OSSL_FUNC_encoder_encode_fn * OSSL_FUNC_encoder_encode(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_encoder_encode_fn *)opf->function; }





typedef void *(OSSL_FUNC_encoder_import_object_fn) (void *ctx, int selection, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_encoder_import_object_fn * OSSL_FUNC_encoder_import_object(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_encoder_import_object_fn *)opf->function; }

typedef void(OSSL_FUNC_encoder_free_object_fn) (void *obj); static __attribute__((unused)) inline OSSL_FUNC_encoder_free_object_fn * OSSL_FUNC_encoder_free_object(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_encoder_free_object_fn *)opf->function; }
typedef void *(OSSL_FUNC_decoder_newctx_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_decoder_newctx_fn * OSSL_FUNC_decoder_newctx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_decoder_newctx_fn *)opf->function; }
typedef void(OSSL_FUNC_decoder_freectx_fn) (void *ctx); static __attribute__((unused)) inline OSSL_FUNC_decoder_freectx_fn * OSSL_FUNC_decoder_freectx(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_decoder_freectx_fn *)opf->function; }
typedef int(OSSL_FUNC_decoder_get_params_fn) (OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_decoder_get_params_fn * OSSL_FUNC_decoder_get_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_decoder_get_params_fn *)opf->function; }
typedef const OSSL_PARAM *(OSSL_FUNC_decoder_gettable_params_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_decoder_gettable_params_fn * OSSL_FUNC_decoder_gettable_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_decoder_gettable_params_fn *)opf->function; }

typedef int(OSSL_FUNC_decoder_set_ctx_params_fn) (void *ctx, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_decoder_set_ctx_params_fn * OSSL_FUNC_decoder_set_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_decoder_set_ctx_params_fn *)opf->function; }

typedef const OSSL_PARAM *(OSSL_FUNC_decoder_settable_ctx_params_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_decoder_settable_ctx_params_fn * OSSL_FUNC_decoder_settable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_decoder_settable_ctx_params_fn *)opf->function; }


typedef int(OSSL_FUNC_decoder_does_selection_fn) (void *provctx, int selection); static __attribute__((unused)) inline OSSL_FUNC_decoder_does_selection_fn * OSSL_FUNC_decoder_does_selection(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_decoder_does_selection_fn *)opf->function; }

typedef int(OSSL_FUNC_decoder_decode_fn) (void *ctx, OSSL_CORE_BIO *in, int selection, OSSL_CALLBACK *data_cb, void *data_cbarg, OSSL_PASSPHRASE_CALLBACK *pw_cb, void *pw_cbarg); static __attribute__((unused)) inline OSSL_FUNC_decoder_decode_fn * OSSL_FUNC_decoder_decode(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_decoder_decode_fn *)opf->function; }



typedef int(OSSL_FUNC_decoder_export_object_fn) (void *ctx, const void *objref, size_t objref_sz, OSSL_CALLBACK *export_cb, void *export_cbarg); static __attribute__((unused)) inline OSSL_FUNC_decoder_export_object_fn * OSSL_FUNC_decoder_export_object(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_decoder_export_object_fn *)opf->function; }
typedef void *(OSSL_FUNC_store_open_fn) (void *provctx, const char *uri); static __attribute__((unused)) inline OSSL_FUNC_store_open_fn * OSSL_FUNC_store_open(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_store_open_fn *)opf->function; }
typedef void *(OSSL_FUNC_store_attach_fn) (void *provctx, OSSL_CORE_BIO *in); static __attribute__((unused)) inline OSSL_FUNC_store_attach_fn * OSSL_FUNC_store_attach(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_store_attach_fn *)opf->function; }
typedef const OSSL_PARAM *(OSSL_FUNC_store_settable_ctx_params_fn) (void *provctx); static __attribute__((unused)) inline OSSL_FUNC_store_settable_ctx_params_fn * OSSL_FUNC_store_settable_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_store_settable_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_store_set_ctx_params_fn) (void *loaderctx, const OSSL_PARAM params[]); static __attribute__((unused)) inline OSSL_FUNC_store_set_ctx_params_fn * OSSL_FUNC_store_set_ctx_params(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_store_set_ctx_params_fn *)opf->function; }

typedef int(OSSL_FUNC_store_load_fn) (void *loaderctx, OSSL_CALLBACK *object_cb, void *object_cbarg, OSSL_PASSPHRASE_CALLBACK *pw_cb, void *pw_cbarg); static __attribute__((unused)) inline OSSL_FUNC_store_load_fn * OSSL_FUNC_store_load(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_store_load_fn *)opf->function; }



typedef int(OSSL_FUNC_store_eof_fn) (void *loaderctx); static __attribute__((unused)) inline OSSL_FUNC_store_eof_fn * OSSL_FUNC_store_eof(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_store_eof_fn *)opf->function; }
typedef int(OSSL_FUNC_store_close_fn) (void *loaderctx); static __attribute__((unused)) inline OSSL_FUNC_store_close_fn * OSSL_FUNC_store_close(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_store_close_fn *)opf->function; }
typedef int(OSSL_FUNC_store_export_object_fn) (void *loaderctx, const void *objref, size_t objref_sz, OSSL_CALLBACK *export_cb, void *export_cbarg); static __attribute__((unused)) inline OSSL_FUNC_store_export_object_fn * OSSL_FUNC_store_export_object(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_store_export_object_fn *)opf->function; }


typedef int(OSSL_FUNC_store_delete_fn) (void *provctx, const char *uri, const OSSL_PARAM params[], OSSL_PASSPHRASE_CALLBACK *pw_cb, void *pw_cbarg); static __attribute__((unused)) inline OSSL_FUNC_store_delete_fn * OSSL_FUNC_store_delete(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_store_delete_fn *)opf->function; }


typedef void *(OSSL_FUNC_store_open_ex_fn) (void *provctx, const char *uri, const OSSL_PARAM params[], OSSL_PASSPHRASE_CALLBACK *pw_cb, void *pw_cbarg); static __attribute__((unused)) inline OSSL_FUNC_store_open_ex_fn * OSSL_FUNC_store_open_ex(const OSSL_DISPATCH *opf) { return (OSSL_FUNC_store_open_ex_fn *)opf->function; }

       
       
typedef union bio_addr_st BIO_ADDR;
typedef struct bio_addrinfo_st BIO_ADDRINFO;

int BIO_get_new_index(void);
void BIO_set_flags(BIO *b, int flags);
int BIO_test_flags(const BIO *b, int flags);
void BIO_clear_flags(BIO *b, int flags);
typedef long (*BIO_callback_fn)(BIO *b, int oper, const char *argp, int argi,
    long argl, long ret);
__attribute__((deprecated("Since OpenSSL " "3.0"))) BIO_callback_fn BIO_get_callback(const BIO *b);
__attribute__((deprecated("Since OpenSSL " "3.0"))) void BIO_set_callback(BIO *b, BIO_callback_fn callback);
__attribute__((deprecated("Since OpenSSL " "3.0"))) long BIO_debug_callback(BIO *bio, int cmd,
    const char *argp, int argi,
    long argl, long ret);


typedef long (*BIO_callback_fn_ex)(BIO *b, int oper, const char *argp,
    size_t len, int argi,
    long argl, int ret, size_t *processed);
BIO_callback_fn_ex BIO_get_callback_ex(const BIO *b);
void BIO_set_callback_ex(BIO *b, BIO_callback_fn_ex callback);
long BIO_debug_callback_ex(BIO *bio, int oper, const char *argp, size_t len,
    int argi, long argl, int ret, size_t *processed);

char *BIO_get_callback_arg(const BIO *b);
void BIO_set_callback_arg(BIO *b, char *arg);

typedef struct bio_method_st BIO_METHOD;

const char *BIO_method_name(const BIO *b);
int BIO_method_type(const BIO *b);

typedef int BIO_info_cb(BIO *, int, int);
typedef BIO_info_cb bio_info_cb;


struct stack_st_BIO; typedef int (*sk_BIO_compfunc)(const BIO *const *a, const BIO *const *b); typedef void (*sk_BIO_freefunc)(BIO * a); typedef BIO *(*sk_BIO_copyfunc)(const BIO *a); static inline void sk_BIO_freefunc_thunk(OPENSSL_sk_freefunc freefunc_arg, void *ptr) { sk_BIO_freefunc freefunc = (sk_BIO_freefunc)freefunc_arg; freefunc((BIO *)ptr); } static __attribute__((unused)) inline BIO *ossl_check_BIO_type(BIO *ptr) { return ptr; } static __attribute__((unused)) inline const OPENSSL_STACK *ossl_check_const_BIO_sk_type(const struct stack_st_BIO *sk) { return (const OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_STACK *ossl_check_BIO_sk_type(struct stack_st_BIO *sk) { return (OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_sk_compfunc ossl_check_BIO_compfunc_type(sk_BIO_compfunc cmp) { return (OPENSSL_sk_compfunc)cmp; } static __attribute__((unused)) inline OPENSSL_sk_copyfunc ossl_check_BIO_copyfunc_type(sk_BIO_copyfunc cpy) { return (OPENSSL_sk_copyfunc)cpy; } static __attribute__((unused)) inline OPENSSL_sk_freefunc ossl_check_BIO_freefunc_type(sk_BIO_freefunc fr) { return (OPENSSL_sk_freefunc)fr; }
typedef int asn1_ps_func(BIO *b, unsigned char **pbuf, int *plen,
    void *parg);

typedef void (*BIO_dgram_sctp_notification_handler_fn)(BIO *b,
    void *context,
    void *buf);
typedef struct bio_msg_st {
    void *data;
    size_t data_len;
    BIO_ADDR *peer, *local;
    uint64_t flags;
} BIO_MSG;

typedef struct bio_mmsg_cb_args_st {
    BIO_MSG *msg;
    size_t stride, num_msg;
    uint64_t flags;
    size_t *msgs_processed;
} BIO_MMSG_CB_ARGS;






typedef struct bio_poll_descriptor_st {
    uint32_t type;
    union {
        int fd;
        void *custom;
        uintptr_t custom_ui;
        SSL *ssl;
    } value;
} BIO_POLL_DESCRIPTOR;
size_t BIO_ctrl_pending(BIO *b);
size_t BIO_ctrl_wpending(BIO *b);
size_t BIO_ctrl_get_write_guarantee(BIO *b);
size_t BIO_ctrl_get_read_request(BIO *b);
int BIO_ctrl_reset_read_request(BIO *b);
int BIO_set_ex_data(BIO *bio, int idx, void *data);
void *BIO_get_ex_data(const BIO *bio, int idx);
uint64_t BIO_number_read(BIO *bio);
uint64_t BIO_number_written(BIO *bio);


int BIO_asn1_set_prefix(BIO *b, asn1_ps_func *prefix,
    asn1_ps_func *prefix_free);
int BIO_asn1_get_prefix(BIO *b, asn1_ps_func **pprefix,
    asn1_ps_func **pprefix_free);
int BIO_asn1_set_suffix(BIO *b, asn1_ps_func *suffix,
    asn1_ps_func *suffix_free);
int BIO_asn1_get_suffix(BIO *b, asn1_ps_func **psuffix,
    asn1_ps_func **psuffix_free);

const BIO_METHOD *BIO_s_file(void);
BIO *BIO_new_file(const char *filename, const char *mode);
BIO *BIO_new_from_core_bio(OSSL_LIB_CTX *libctx, OSSL_CORE_BIO *corebio);

BIO *BIO_new_fp(FILE *stream, int close_flag);

BIO *BIO_new_ex(OSSL_LIB_CTX *libctx, const BIO_METHOD *method);
BIO *BIO_new(const BIO_METHOD *type);
int BIO_free(BIO *a);
void BIO_set_data(BIO *a, void *ptr);
void *BIO_get_data(BIO *a);
void BIO_set_init(BIO *a, int init);
int BIO_get_init(BIO *a);
void BIO_set_shutdown(BIO *a, int shut);
int BIO_get_shutdown(BIO *a);
void BIO_vfree(BIO *a);
int BIO_up_ref(BIO *a);
int BIO_read(BIO *b, void *data, int dlen);
int BIO_read_ex(BIO *b, void *data, size_t dlen, size_t *readbytes);
 int BIO_recvmmsg(BIO *b, BIO_MSG *msg,
    size_t stride, size_t num_msg, uint64_t flags,
    size_t *msgs_processed);
int BIO_gets(BIO *bp, char *buf, int size);
int BIO_get_line(BIO *bio, char *buf, int size);
int BIO_write(BIO *b, const void *data, int dlen);
int BIO_write_ex(BIO *b, const void *data, size_t dlen, size_t *written);
 int BIO_sendmmsg(BIO *b, BIO_MSG *msg,
    size_t stride, size_t num_msg, uint64_t flags,
    size_t *msgs_processed);
 int BIO_get_rpoll_descriptor(BIO *b, BIO_POLL_DESCRIPTOR *desc);
 int BIO_get_wpoll_descriptor(BIO *b, BIO_POLL_DESCRIPTOR *desc);
int BIO_puts(BIO *bp, const char *buf);
int BIO_indent(BIO *b, int indent, int max);
long BIO_ctrl(BIO *bp, int cmd, long larg, void *parg);
long BIO_callback_ctrl(BIO *b, int cmd, BIO_info_cb *fp);
void *BIO_ptr_ctrl(BIO *bp, int cmd, long larg);
long BIO_int_ctrl(BIO *bp, int cmd, long larg, int iarg);
BIO *BIO_push(BIO *b, BIO *append);
BIO *BIO_pop(BIO *b);
void BIO_free_all(BIO *a);
BIO *BIO_find_type(BIO *b, int bio_type);
BIO *BIO_next(BIO *b);
void BIO_set_next(BIO *b, BIO *next);
BIO *BIO_get_retry_BIO(BIO *bio, int *reason);
int BIO_get_retry_reason(BIO *bio);
void BIO_set_retry_reason(BIO *bio, int reason);
BIO *BIO_dup_chain(BIO *in);

int BIO_nread0(BIO *bio, char **buf);
int BIO_nread(BIO *bio, char **buf, int num);
int BIO_nwrite0(BIO *bio, char **buf);
int BIO_nwrite(BIO *bio, char **buf, int num);

const BIO_METHOD *BIO_s_mem(void);

const BIO_METHOD *BIO_s_dgram_mem(void);

const BIO_METHOD *BIO_s_secmem(void);
BIO *BIO_new_mem_buf(const void *buf, int len);

const BIO_METHOD *BIO_s_socket(void);
const BIO_METHOD *BIO_s_connect(void);
const BIO_METHOD *BIO_s_accept(void);

const BIO_METHOD *BIO_s_fd(void);
const BIO_METHOD *BIO_s_log(void);
const BIO_METHOD *BIO_s_bio(void);
const BIO_METHOD *BIO_s_null(void);
const BIO_METHOD *BIO_f_null(void);
const BIO_METHOD *BIO_f_buffer(void);
const BIO_METHOD *BIO_f_readbuffer(void);
const BIO_METHOD *BIO_f_linebuffer(void);
const BIO_METHOD *BIO_f_nbio_test(void);
const BIO_METHOD *BIO_f_prefix(void);
const BIO_METHOD *BIO_s_core(void);

const BIO_METHOD *BIO_s_dgram_pair(void);
const BIO_METHOD *BIO_s_datagram(void);
int BIO_dgram_non_fatal_error(int error);
BIO *BIO_new_dgram(int fd, int close_flag);
int BIO_sock_should_retry(int i);
int BIO_sock_non_fatal_error(int error);
int BIO_err_is_non_fatal(unsigned int errcode);
int BIO_socket_wait(int fd, int for_read, time_t max_time);

int BIO_wait(BIO *bio, time_t max_time, unsigned int nap_milliseconds);
int BIO_do_connect_retry(BIO *bio, int timeout, int nap_milliseconds);

int BIO_fd_should_retry(int i);
int BIO_fd_non_fatal_error(int error);
int BIO_dump_cb(int (*cb)(const void *data, size_t len, void *u),
    void *u, const void *s, int len);
int BIO_dump_indent_cb(int (*cb)(const void *data, size_t len, void *u),
    void *u, const void *s, int len, int indent);
int BIO_dump(BIO *b, const void *bytes, int len);
int BIO_dump_indent(BIO *b, const void *bytes, int len, int indent);

int BIO_dump_fp(FILE *fp, const void *s, int len);
int BIO_dump_indent_fp(FILE *fp, const void *s, int len, int indent);

int BIO_hex_string(BIO *out, int indent, int width, const void *data,
    int datalen);


BIO_ADDR *BIO_ADDR_new(void);
int BIO_ADDR_copy(BIO_ADDR *dst, const BIO_ADDR *src);
BIO_ADDR *BIO_ADDR_dup(const BIO_ADDR *ap);
int BIO_ADDR_rawmake(BIO_ADDR *ap, int family,
    const void *where, size_t wherelen, unsigned short port);
void BIO_ADDR_free(BIO_ADDR *);
void BIO_ADDR_clear(BIO_ADDR *ap);
int BIO_ADDR_family(const BIO_ADDR *ap);
int BIO_ADDR_rawaddress(const BIO_ADDR *ap, void *p, size_t *l);
unsigned short BIO_ADDR_rawport(const BIO_ADDR *ap);
char *BIO_ADDR_hostname_string(const BIO_ADDR *ap, int numeric);
char *BIO_ADDR_service_string(const BIO_ADDR *ap, int numeric);
char *BIO_ADDR_path_string(const BIO_ADDR *ap);

const BIO_ADDRINFO *BIO_ADDRINFO_next(const BIO_ADDRINFO *bai);
int BIO_ADDRINFO_family(const BIO_ADDRINFO *bai);
int BIO_ADDRINFO_socktype(const BIO_ADDRINFO *bai);
int BIO_ADDRINFO_protocol(const BIO_ADDRINFO *bai);
const BIO_ADDR *BIO_ADDRINFO_address(const BIO_ADDRINFO *bai);
void BIO_ADDRINFO_free(BIO_ADDRINFO *bai);

enum BIO_hostserv_priorities {
    BIO_PARSE_PRIO_HOST,
    BIO_PARSE_PRIO_SERV
};
int BIO_parse_hostserv(const char *hostserv, char **host, char **service,
    enum BIO_hostserv_priorities hostserv_prio);
enum BIO_lookup_type {
    BIO_LOOKUP_CLIENT,
    BIO_LOOKUP_SERVER
};
int BIO_lookup(const char *host, const char *service,
    enum BIO_lookup_type lookup_type,
    int family, int socktype, BIO_ADDRINFO **res);
int BIO_lookup_ex(const char *host, const char *service,
    int lookup_type, int family, int socktype, int protocol,
    BIO_ADDRINFO **res);
int BIO_sock_error(int sock);
int BIO_socket_ioctl(int fd, long type, void *arg);
int BIO_socket_nbio(int fd, int mode);
int BIO_sock_init(void);





int BIO_set_tcp_ndelay(int sock, int turn_on);

__attribute__((deprecated("Since OpenSSL " "1.1.0"))) struct hostent *BIO_gethostbyname(const char *name);
__attribute__((deprecated("Since OpenSSL " "1.1.0"))) int BIO_get_port(const char *str, unsigned short *port_ptr);
__attribute__((deprecated("Since OpenSSL " "1.1.0"))) int BIO_get_host_ip(const char *str, unsigned char *ip);
__attribute__((deprecated("Since OpenSSL " "1.1.0"))) int BIO_get_accept_socket(char *host_port, int mode);
__attribute__((deprecated("Since OpenSSL " "1.1.0"))) int BIO_accept(int sock, char **ip_port);


union BIO_sock_info_u {
    BIO_ADDR *addr;
};
enum BIO_sock_info_type {
    BIO_SOCK_INFO_ADDRESS
};
int BIO_sock_info(int sock,
    enum BIO_sock_info_type type, union BIO_sock_info_u *info);
int BIO_socket(int domain, int socktype, int protocol, int options);
int BIO_connect(int sock, const BIO_ADDR *addr, int options);
int BIO_bind(int sock, const BIO_ADDR *addr, int options);
int BIO_listen(int sock, const BIO_ADDR *addr, int options);
int BIO_accept_ex(int accept_sock, BIO_ADDR *addr, int options);
int BIO_closesocket(int sock);

BIO *BIO_new_socket(int sock, int close_flag);
BIO *BIO_new_connect(const char *host_port);
BIO *BIO_new_accept(const char *host_port);


BIO *BIO_new_fd(int fd, int close_flag);

int BIO_new_bio_pair(BIO **bio1, size_t writebuf1,
    BIO **bio2, size_t writebuf2);

int BIO_new_bio_dgram_pair(BIO **bio1, size_t writebuf1,
    BIO **bio2, size_t writebuf2);
void BIO_copy_next_retry(BIO *b);
int BIO_printf(BIO *bio, const char *format, ...)
    __attribute__((__format__(__gnu_printf__, 2, 3)));
int BIO_vprintf(BIO *bio, const char *format, va_list args)
    __attribute__((__format__(__gnu_printf__, 2, 0)));
int BIO_snprintf(char *buf, size_t n, const char *format, ...)
    __attribute__((__format__(__gnu_printf__, 3, 4)));
int BIO_vsnprintf(char *buf, size_t n, const char *format, va_list args)
    __attribute__((__format__(__gnu_printf__, 3, 0)));



BIO_METHOD *BIO_meth_new(int type, const char *name);
void BIO_meth_free(BIO_METHOD *biom);
int BIO_meth_set_write(BIO_METHOD *biom,
    int (*write)(BIO *, const char *, int));
int BIO_meth_set_write_ex(BIO_METHOD *biom,
    int (*bwrite)(BIO *, const char *, size_t, size_t *));
int BIO_meth_set_sendmmsg(BIO_METHOD *biom,
    int (*f)(BIO *, BIO_MSG *, size_t, size_t,
        uint64_t, size_t *));
int BIO_meth_set_read(BIO_METHOD *biom,
    int (*read)(BIO *, char *, int));
int BIO_meth_set_read_ex(BIO_METHOD *biom,
    int (*bread)(BIO *, char *, size_t, size_t *));
int BIO_meth_set_recvmmsg(BIO_METHOD *biom,
    int (*f)(BIO *, BIO_MSG *, size_t, size_t,
        uint64_t, size_t *));
int BIO_meth_set_puts(BIO_METHOD *biom,
    int (*puts)(BIO *, const char *));
int BIO_meth_set_gets(BIO_METHOD *biom,
    int (*ossl_gets)(BIO *, char *, int));
int BIO_meth_set_ctrl(BIO_METHOD *biom,
    long (*ctrl)(BIO *, int, long, void *));
int BIO_meth_set_create(BIO_METHOD *biom, int (*create)(BIO *));
int BIO_meth_set_destroy(BIO_METHOD *biom, int (*destroy)(BIO *));
int BIO_meth_set_callback_ctrl(BIO_METHOD *biom,
    long (*callback_ctrl)(BIO *, int,
        BIO_info_cb *));

__attribute__((deprecated("Since OpenSSL " "3.5"))) int (*BIO_meth_get_write(const BIO_METHOD *biom))(BIO *, const char *,
    int);
__attribute__((deprecated("Since OpenSSL " "3.5"))) int (*BIO_meth_get_write_ex(const BIO_METHOD *biom))(BIO *, const char *,
    size_t, size_t *);
__attribute__((deprecated("Since OpenSSL " "3.5"))) int (*BIO_meth_get_sendmmsg(const BIO_METHOD *biom))(BIO *, BIO_MSG *,
    size_t, size_t,
    uint64_t, size_t *);
__attribute__((deprecated("Since OpenSSL " "3.5"))) int (*BIO_meth_get_read(const BIO_METHOD *biom))(BIO *, char *, int);
__attribute__((deprecated("Since OpenSSL " "3.5"))) int (*BIO_meth_get_read_ex(const BIO_METHOD *biom))(BIO *, char *,
    size_t, size_t *);
__attribute__((deprecated("Since OpenSSL " "3.5"))) int (*BIO_meth_get_recvmmsg(const BIO_METHOD *biom))(BIO *, BIO_MSG *,
    size_t, size_t,
    uint64_t, size_t *);
__attribute__((deprecated("Since OpenSSL " "3.5"))) int (*BIO_meth_get_puts(const BIO_METHOD *biom))(BIO *, const char *);
__attribute__((deprecated("Since OpenSSL " "3.5"))) int (*BIO_meth_get_gets(const BIO_METHOD *biom))(BIO *, char *, int);
__attribute__((deprecated("Since OpenSSL " "3.5"))) long (*BIO_meth_get_ctrl(const BIO_METHOD *biom))(BIO *, int,
    long, void *);
__attribute__((deprecated("Since OpenSSL " "3.5"))) int (*BIO_meth_get_create(const BIO_METHOD *bion))(BIO *);
__attribute__((deprecated("Since OpenSSL " "3.5"))) int (*BIO_meth_get_destroy(const BIO_METHOD *biom))(BIO *);
__attribute__((deprecated("Since OpenSSL " "3.5"))) long (*BIO_meth_get_callback_ctrl(const BIO_METHOD *biom))(BIO *, int,
    BIO_info_cb *);
       
       






        

       
       


struct stack_st_X509_ALGOR; typedef int (*sk_X509_ALGOR_compfunc)(const X509_ALGOR *const *a, const X509_ALGOR *const *b); typedef void (*sk_X509_ALGOR_freefunc)(X509_ALGOR * a); typedef X509_ALGOR *(*sk_X509_ALGOR_copyfunc)(const X509_ALGOR *a); static inline void sk_X509_ALGOR_freefunc_thunk(OPENSSL_sk_freefunc freefunc_arg, void *ptr) { sk_X509_ALGOR_freefunc freefunc = (sk_X509_ALGOR_freefunc)freefunc_arg; freefunc((X509_ALGOR *)ptr); } static __attribute__((unused)) inline X509_ALGOR *ossl_check_X509_ALGOR_type(X509_ALGOR *ptr) { return ptr; } static __attribute__((unused)) inline const OPENSSL_STACK *ossl_check_const_X509_ALGOR_sk_type(const struct stack_st_X509_ALGOR *sk) { return (const OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_STACK *ossl_check_X509_ALGOR_sk_type(struct stack_st_X509_ALGOR *sk) { return (OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_sk_compfunc ossl_check_X509_ALGOR_compfunc_type(sk_X509_ALGOR_compfunc cmp) { return (OPENSSL_sk_compfunc)cmp; } static __attribute__((unused)) inline OPENSSL_sk_copyfunc ossl_check_X509_ALGOR_copyfunc_type(sk_X509_ALGOR_copyfunc cpy) { return (OPENSSL_sk_copyfunc)cpy; } static __attribute__((unused)) inline OPENSSL_sk_freefunc ossl_check_X509_ALGOR_freefunc_type(sk_X509_ALGOR_freefunc fr) { return (OPENSSL_sk_freefunc)fr; }
struct asn1_string_st {
    int length;
    int type;
    unsigned char *data;





    long flags;
};







typedef struct ASN1_ENCODING_st {
    unsigned char *enc;
    long len;
    int modified;
} ASN1_ENCODING;
struct asn1_string_table_st {
    int nid;
    long minsize;
    long maxsize;
    unsigned long mask;
    unsigned long flags;
};


struct stack_st_ASN1_STRING_TABLE; typedef int (*sk_ASN1_STRING_TABLE_compfunc)(const ASN1_STRING_TABLE *const *a, const ASN1_STRING_TABLE *const *b); typedef void (*sk_ASN1_STRING_TABLE_freefunc)(ASN1_STRING_TABLE * a); typedef ASN1_STRING_TABLE *(*sk_ASN1_STRING_TABLE_copyfunc)(const ASN1_STRING_TABLE *a); static inline void sk_ASN1_STRING_TABLE_freefunc_thunk(OPENSSL_sk_freefunc freefunc_arg, void *ptr) { sk_ASN1_STRING_TABLE_freefunc freefunc = (sk_ASN1_STRING_TABLE_freefunc)freefunc_arg; freefunc((ASN1_STRING_TABLE *)ptr); } static __attribute__((unused)) inline ASN1_STRING_TABLE *ossl_check_ASN1_STRING_TABLE_type(ASN1_STRING_TABLE *ptr) { return ptr; } static __attribute__((unused)) inline const OPENSSL_STACK *ossl_check_const_ASN1_STRING_TABLE_sk_type(const struct stack_st_ASN1_STRING_TABLE *sk) { return (const OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_STACK *ossl_check_ASN1_STRING_TABLE_sk_type(struct stack_st_ASN1_STRING_TABLE *sk) { return (OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_sk_compfunc ossl_check_ASN1_STRING_TABLE_compfunc_type(sk_ASN1_STRING_TABLE_compfunc cmp) { return (OPENSSL_sk_compfunc)cmp; } static __attribute__((unused)) inline OPENSSL_sk_copyfunc ossl_check_ASN1_STRING_TABLE_copyfunc_type(sk_ASN1_STRING_TABLE_copyfunc cpy) { return (OPENSSL_sk_copyfunc)cpy; } static __attribute__((unused)) inline OPENSSL_sk_freefunc ossl_check_ASN1_STRING_TABLE_freefunc_type(sk_ASN1_STRING_TABLE_freefunc fr) { return (OPENSSL_sk_freefunc)fr; }
typedef struct ASN1_TEMPLATE_st ASN1_TEMPLATE;
typedef struct ASN1_TLC_st ASN1_TLC;

typedef struct ASN1_VALUE_st ASN1_VALUE;
typedef void *d2i_of_void(void **, const unsigned char **, long);
typedef int i2d_of_void(const void *, unsigned char **);
typedef int OSSL_i2d_of_void_ctx(const void *, unsigned char **, void *vctx);
typedef const ASN1_ITEM *ASN1_ITEM_EXP(void);
struct asn1_type_st {
    int type;
    union {
        char *ptr;
        ASN1_BOOLEAN boolean;
        ASN1_STRING *asn1_string;
        ASN1_OBJECT *object;
        ASN1_INTEGER *integer;
        ASN1_ENUMERATED *enumerated;
        ASN1_BIT_STRING *bit_string;
        ASN1_OCTET_STRING *octet_string;
        ASN1_PRINTABLESTRING *printablestring;
        ASN1_T61STRING *t61string;
        ASN1_IA5STRING *ia5string;
        ASN1_GENERALSTRING *generalstring;
        ASN1_BMPSTRING *bmpstring;
        ASN1_UNIVERSALSTRING *universalstring;
        ASN1_UTCTIME *utctime;
        ASN1_GENERALIZEDTIME *generalizedtime;
        ASN1_VISIBLESTRING *visiblestring;
        ASN1_UTF8STRING *utf8string;




        ASN1_STRING *set;
        ASN1_STRING *sequence;
        ASN1_VALUE *asn1_value;
    } value;
};


struct stack_st_ASN1_TYPE; typedef int (*sk_ASN1_TYPE_compfunc)(const ASN1_TYPE *const *a, const ASN1_TYPE *const *b); typedef void (*sk_ASN1_TYPE_freefunc)(ASN1_TYPE * a); typedef ASN1_TYPE *(*sk_ASN1_TYPE_copyfunc)(const ASN1_TYPE *a); static inline void sk_ASN1_TYPE_freefunc_thunk(OPENSSL_sk_freefunc freefunc_arg, void *ptr) { sk_ASN1_TYPE_freefunc freefunc = (sk_ASN1_TYPE_freefunc)freefunc_arg; freefunc((ASN1_TYPE *)ptr); } static __attribute__((unused)) inline ASN1_TYPE *ossl_check_ASN1_TYPE_type(ASN1_TYPE *ptr) { return ptr; } static __attribute__((unused)) inline const OPENSSL_STACK *ossl_check_const_ASN1_TYPE_sk_type(const struct stack_st_ASN1_TYPE *sk) { return (const OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_STACK *ossl_check_ASN1_TYPE_sk_type(struct stack_st_ASN1_TYPE *sk) { return (OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_sk_compfunc ossl_check_ASN1_TYPE_compfunc_type(sk_ASN1_TYPE_compfunc cmp) { return (OPENSSL_sk_compfunc)cmp; } static __attribute__((unused)) inline OPENSSL_sk_copyfunc ossl_check_ASN1_TYPE_copyfunc_type(sk_ASN1_TYPE_copyfunc cpy) { return (OPENSSL_sk_copyfunc)cpy; } static __attribute__((unused)) inline OPENSSL_sk_freefunc ossl_check_ASN1_TYPE_freefunc_type(sk_ASN1_TYPE_freefunc fr) { return (OPENSSL_sk_freefunc)fr; }
typedef struct stack_st_ASN1_TYPE ASN1_SEQUENCE_ANY;

extern ASN1_SEQUENCE_ANY *d2i_ASN1_SEQUENCE_ANY(ASN1_SEQUENCE_ANY **a, const unsigned char **in, long len); extern int i2d_ASN1_SEQUENCE_ANY(const ASN1_SEQUENCE_ANY *a, unsigned char **out); extern const ASN1_ITEM *ASN1_SEQUENCE_ANY_it(void);
extern ASN1_SEQUENCE_ANY *d2i_ASN1_SET_ANY(ASN1_SEQUENCE_ANY **a, const unsigned char **in, long len); extern int i2d_ASN1_SET_ANY(const ASN1_SEQUENCE_ANY *a, unsigned char **out); extern const ASN1_ITEM *ASN1_SET_ANY_it(void);


typedef struct BIT_STRING_BITNAME_st {
    int bitnum;
    const char *lname;
    const char *sname;
} BIT_STRING_BITNAME;
extern ASN1_TYPE *ASN1_TYPE_new(void); extern void ASN1_TYPE_free(ASN1_TYPE *a);
extern ASN1_TYPE *d2i_ASN1_TYPE(ASN1_TYPE **a, const unsigned char **in, long len); extern int i2d_ASN1_TYPE(const ASN1_TYPE *a, unsigned char **out); extern const ASN1_ITEM *ASN1_ANY_it(void);

int ASN1_TYPE_get(const ASN1_TYPE *a);
void ASN1_TYPE_set(ASN1_TYPE *a, int type, void *value);
int ASN1_TYPE_set1(ASN1_TYPE *a, int type, const void *value);
int ASN1_TYPE_cmp(const ASN1_TYPE *a, const ASN1_TYPE *b);

ASN1_TYPE *ASN1_TYPE_pack_sequence(const ASN1_ITEM *it, void *s, ASN1_TYPE **t);
void *ASN1_TYPE_unpack_sequence(const ASN1_ITEM *it, const ASN1_TYPE *t);


struct stack_st_ASN1_OBJECT; typedef int (*sk_ASN1_OBJECT_compfunc)(const ASN1_OBJECT *const *a, const ASN1_OBJECT *const *b); typedef void (*sk_ASN1_OBJECT_freefunc)(ASN1_OBJECT * a); typedef ASN1_OBJECT *(*sk_ASN1_OBJECT_copyfunc)(const ASN1_OBJECT *a); static inline void sk_ASN1_OBJECT_freefunc_thunk(OPENSSL_sk_freefunc freefunc_arg, void *ptr) { sk_ASN1_OBJECT_freefunc freefunc = (sk_ASN1_OBJECT_freefunc)freefunc_arg; freefunc((ASN1_OBJECT *)ptr); } static __attribute__((unused)) inline ASN1_OBJECT *ossl_check_ASN1_OBJECT_type(ASN1_OBJECT *ptr) { return ptr; } static __attribute__((unused)) inline const OPENSSL_STACK *ossl_check_const_ASN1_OBJECT_sk_type(const struct stack_st_ASN1_OBJECT *sk) { return (const OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_STACK *ossl_check_ASN1_OBJECT_sk_type(struct stack_st_ASN1_OBJECT *sk) { return (OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_sk_compfunc ossl_check_ASN1_OBJECT_compfunc_type(sk_ASN1_OBJECT_compfunc cmp) { return (OPENSSL_sk_compfunc)cmp; } static __attribute__((unused)) inline OPENSSL_sk_copyfunc ossl_check_ASN1_OBJECT_copyfunc_type(sk_ASN1_OBJECT_copyfunc cpy) { return (OPENSSL_sk_copyfunc)cpy; } static __attribute__((unused)) inline OPENSSL_sk_freefunc ossl_check_ASN1_OBJECT_freefunc_type(sk_ASN1_OBJECT_freefunc fr) { return (OPENSSL_sk_freefunc)fr; }
extern ASN1_OBJECT *ASN1_OBJECT_new(void); extern void ASN1_OBJECT_free(ASN1_OBJECT *a); extern ASN1_OBJECT *d2i_ASN1_OBJECT(ASN1_OBJECT **a, const unsigned char **in, long len); extern int i2d_ASN1_OBJECT(const ASN1_OBJECT *a, unsigned char **out); extern const ASN1_ITEM *ASN1_OBJECT_it(void);

ASN1_STRING *ASN1_STRING_new(void);
void ASN1_STRING_free(ASN1_STRING *a);
void ASN1_STRING_clear_free(ASN1_STRING *a);
int ASN1_STRING_copy(ASN1_STRING *dst, const ASN1_STRING *str);
extern ASN1_STRING *ASN1_STRING_dup(const ASN1_STRING *a);
ASN1_STRING *ASN1_STRING_type_new(int type);
int ASN1_STRING_cmp(const ASN1_STRING *a, const ASN1_STRING *b);




int ASN1_STRING_set(ASN1_STRING *str, const void *data, int len);
void ASN1_STRING_set0(ASN1_STRING *str, void *data, int len);
int ASN1_STRING_length(const ASN1_STRING *x);

__attribute__((deprecated("Since OpenSSL " "3.0"))) void ASN1_STRING_length_set(ASN1_STRING *x, int n);

int ASN1_STRING_type(const ASN1_STRING *x);

__attribute__((deprecated("Since OpenSSL " "1.1.0"))) unsigned char *ASN1_STRING_data(ASN1_STRING *x);

const unsigned char *ASN1_STRING_get0_data(const ASN1_STRING *x);

extern ASN1_BIT_STRING *ASN1_BIT_STRING_new(void); extern void ASN1_BIT_STRING_free(ASN1_BIT_STRING *a); extern ASN1_BIT_STRING *d2i_ASN1_BIT_STRING(ASN1_BIT_STRING **a, const unsigned char **in, long len); extern int i2d_ASN1_BIT_STRING(const ASN1_BIT_STRING *a, unsigned char **out); extern const ASN1_ITEM *ASN1_BIT_STRING_it(void);
int ASN1_BIT_STRING_set(ASN1_BIT_STRING *a, unsigned char *d, int length);
int ASN1_BIT_STRING_set_bit(ASN1_BIT_STRING *a, int n, int value);
int ASN1_BIT_STRING_get_bit(const ASN1_BIT_STRING *a, int n);
int ASN1_BIT_STRING_check(const ASN1_BIT_STRING *a,
    const unsigned char *flags, int flags_len);

int ASN1_BIT_STRING_name_print(BIO *out, ASN1_BIT_STRING *bs,
    BIT_STRING_BITNAME *tbl, int indent);
int ASN1_BIT_STRING_num_asc(const char *name, BIT_STRING_BITNAME *tbl);
int ASN1_BIT_STRING_set_asc(ASN1_BIT_STRING *bs, const char *name, int value,
    BIT_STRING_BITNAME *tbl);


struct stack_st_ASN1_INTEGER; typedef int (*sk_ASN1_INTEGER_compfunc)(const ASN1_INTEGER *const *a, const ASN1_INTEGER *const *b); typedef void (*sk_ASN1_INTEGER_freefunc)(ASN1_INTEGER * a); typedef ASN1_INTEGER *(*sk_ASN1_INTEGER_copyfunc)(const ASN1_INTEGER *a); static inline void sk_ASN1_INTEGER_freefunc_thunk(OPENSSL_sk_freefunc freefunc_arg, void *ptr) { sk_ASN1_INTEGER_freefunc freefunc = (sk_ASN1_INTEGER_freefunc)freefunc_arg; freefunc((ASN1_INTEGER *)ptr); } static __attribute__((unused)) inline ASN1_INTEGER *ossl_check_ASN1_INTEGER_type(ASN1_INTEGER *ptr) { return ptr; } static __attribute__((unused)) inline const OPENSSL_STACK *ossl_check_const_ASN1_INTEGER_sk_type(const struct stack_st_ASN1_INTEGER *sk) { return (const OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_STACK *ossl_check_ASN1_INTEGER_sk_type(struct stack_st_ASN1_INTEGER *sk) { return (OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_sk_compfunc ossl_check_ASN1_INTEGER_compfunc_type(sk_ASN1_INTEGER_compfunc cmp) { return (OPENSSL_sk_compfunc)cmp; } static __attribute__((unused)) inline OPENSSL_sk_copyfunc ossl_check_ASN1_INTEGER_copyfunc_type(sk_ASN1_INTEGER_copyfunc cpy) { return (OPENSSL_sk_copyfunc)cpy; } static __attribute__((unused)) inline OPENSSL_sk_freefunc ossl_check_ASN1_INTEGER_freefunc_type(sk_ASN1_INTEGER_freefunc fr) { return (OPENSSL_sk_freefunc)fr; }
extern ASN1_INTEGER *ASN1_INTEGER_new(void); extern void ASN1_INTEGER_free(ASN1_INTEGER *a); extern ASN1_INTEGER *d2i_ASN1_INTEGER(ASN1_INTEGER **a, const unsigned char **in, long len); extern int i2d_ASN1_INTEGER(const ASN1_INTEGER *a, unsigned char **out); extern const ASN1_ITEM *ASN1_INTEGER_it(void);
ASN1_INTEGER *d2i_ASN1_UINTEGER(ASN1_INTEGER **a, const unsigned char **pp,
    long length);
extern ASN1_INTEGER *ASN1_INTEGER_dup(const ASN1_INTEGER *a);
int ASN1_INTEGER_cmp(const ASN1_INTEGER *x, const ASN1_INTEGER *y);

extern ASN1_ENUMERATED *ASN1_ENUMERATED_new(void); extern void ASN1_ENUMERATED_free(ASN1_ENUMERATED *a); extern ASN1_ENUMERATED *d2i_ASN1_ENUMERATED(ASN1_ENUMERATED **a, const unsigned char **in, long len); extern int i2d_ASN1_ENUMERATED(const ASN1_ENUMERATED *a, unsigned char **out); extern const ASN1_ITEM *ASN1_ENUMERATED_it(void);

int ASN1_UTCTIME_check(const ASN1_UTCTIME *a);
ASN1_UTCTIME *ASN1_UTCTIME_set(ASN1_UTCTIME *s, time_t t);
ASN1_UTCTIME *ASN1_UTCTIME_adj(ASN1_UTCTIME *s, time_t t,
    int offset_day, long offset_sec);
int ASN1_UTCTIME_set_string(ASN1_UTCTIME *s, const char *str);
int ASN1_UTCTIME_cmp_time_t(const ASN1_UTCTIME *s, time_t t);

int ASN1_GENERALIZEDTIME_check(const ASN1_GENERALIZEDTIME *a);
ASN1_GENERALIZEDTIME *ASN1_GENERALIZEDTIME_set(ASN1_GENERALIZEDTIME *s,
    time_t t);
ASN1_GENERALIZEDTIME *ASN1_GENERALIZEDTIME_adj(ASN1_GENERALIZEDTIME *s,
    time_t t, int offset_day,
    long offset_sec);
int ASN1_GENERALIZEDTIME_set_string(ASN1_GENERALIZEDTIME *s, const char *str);

int ASN1_TIME_diff(int *pday, int *psec,
    const ASN1_TIME *from, const ASN1_TIME *to);

extern ASN1_OCTET_STRING *ASN1_OCTET_STRING_new(void); extern void ASN1_OCTET_STRING_free(ASN1_OCTET_STRING *a); extern ASN1_OCTET_STRING *d2i_ASN1_OCTET_STRING(ASN1_OCTET_STRING **a, const unsigned char **in, long len); extern int i2d_ASN1_OCTET_STRING(const ASN1_OCTET_STRING *a, unsigned char **out); extern const ASN1_ITEM *ASN1_OCTET_STRING_it(void);
extern ASN1_OCTET_STRING *ASN1_OCTET_STRING_dup(const ASN1_OCTET_STRING *a);
int ASN1_OCTET_STRING_cmp(const ASN1_OCTET_STRING *a,
    const ASN1_OCTET_STRING *b);
int ASN1_OCTET_STRING_set(ASN1_OCTET_STRING *str, const unsigned char *data,
    int len);


struct stack_st_ASN1_UTF8STRING; typedef int (*sk_ASN1_UTF8STRING_compfunc)(const ASN1_UTF8STRING *const *a, const ASN1_UTF8STRING *const *b); typedef void (*sk_ASN1_UTF8STRING_freefunc)(ASN1_UTF8STRING * a); typedef ASN1_UTF8STRING *(*sk_ASN1_UTF8STRING_copyfunc)(const ASN1_UTF8STRING *a); static inline void sk_ASN1_UTF8STRING_freefunc_thunk(OPENSSL_sk_freefunc freefunc_arg, void *ptr) { sk_ASN1_UTF8STRING_freefunc freefunc = (sk_ASN1_UTF8STRING_freefunc)freefunc_arg; freefunc((ASN1_UTF8STRING *)ptr); } static __attribute__((unused)) inline ASN1_UTF8STRING *ossl_check_ASN1_UTF8STRING_type(ASN1_UTF8STRING *ptr) { return ptr; } static __attribute__((unused)) inline const OPENSSL_STACK *ossl_check_const_ASN1_UTF8STRING_sk_type(const struct stack_st_ASN1_UTF8STRING *sk) { return (const OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_STACK *ossl_check_ASN1_UTF8STRING_sk_type(struct stack_st_ASN1_UTF8STRING *sk) { return (OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_sk_compfunc ossl_check_ASN1_UTF8STRING_compfunc_type(sk_ASN1_UTF8STRING_compfunc cmp) { return (OPENSSL_sk_compfunc)cmp; } static __attribute__((unused)) inline OPENSSL_sk_copyfunc ossl_check_ASN1_UTF8STRING_copyfunc_type(sk_ASN1_UTF8STRING_copyfunc cpy) { return (OPENSSL_sk_copyfunc)cpy; } static __attribute__((unused)) inline OPENSSL_sk_freefunc ossl_check_ASN1_UTF8STRING_freefunc_type(sk_ASN1_UTF8STRING_freefunc fr) { return (OPENSSL_sk_freefunc)fr; }
extern ASN1_VISIBLESTRING *ASN1_VISIBLESTRING_new(void); extern void ASN1_VISIBLESTRING_free(ASN1_VISIBLESTRING *a); extern ASN1_VISIBLESTRING *d2i_ASN1_VISIBLESTRING(ASN1_VISIBLESTRING **a, const unsigned char **in, long len); extern int i2d_ASN1_VISIBLESTRING(const ASN1_VISIBLESTRING *a, unsigned char **out); extern const ASN1_ITEM *ASN1_VISIBLESTRING_it(void);
extern ASN1_UNIVERSALSTRING *ASN1_UNIVERSALSTRING_new(void); extern void ASN1_UNIVERSALSTRING_free(ASN1_UNIVERSALSTRING *a); extern ASN1_UNIVERSALSTRING *d2i_ASN1_UNIVERSALSTRING(ASN1_UNIVERSALSTRING **a, const unsigned char **in, long len); extern int i2d_ASN1_UNIVERSALSTRING(const ASN1_UNIVERSALSTRING *a, unsigned char **out); extern const ASN1_ITEM *ASN1_UNIVERSALSTRING_it(void);
extern ASN1_UTF8STRING *ASN1_UTF8STRING_new(void); extern void ASN1_UTF8STRING_free(ASN1_UTF8STRING *a); extern ASN1_UTF8STRING *d2i_ASN1_UTF8STRING(ASN1_UTF8STRING **a, const unsigned char **in, long len); extern int i2d_ASN1_UTF8STRING(const ASN1_UTF8STRING *a, unsigned char **out); extern const ASN1_ITEM *ASN1_UTF8STRING_it(void);
extern ASN1_NULL *ASN1_NULL_new(void); extern void ASN1_NULL_free(ASN1_NULL *a); extern ASN1_NULL *d2i_ASN1_NULL(ASN1_NULL **a, const unsigned char **in, long len); extern int i2d_ASN1_NULL(const ASN1_NULL *a, unsigned char **out); extern const ASN1_ITEM *ASN1_NULL_it(void);
extern ASN1_BMPSTRING *ASN1_BMPSTRING_new(void); extern void ASN1_BMPSTRING_free(ASN1_BMPSTRING *a); extern ASN1_BMPSTRING *d2i_ASN1_BMPSTRING(ASN1_BMPSTRING **a, const unsigned char **in, long len); extern int i2d_ASN1_BMPSTRING(const ASN1_BMPSTRING *a, unsigned char **out); extern const ASN1_ITEM *ASN1_BMPSTRING_it(void);

int UTF8_getc(const unsigned char *str, int len, unsigned long *val);
int UTF8_putc(unsigned char *str, int len, unsigned long value);


struct stack_st_ASN1_GENERALSTRING; typedef int (*sk_ASN1_GENERALSTRING_compfunc)(const ASN1_GENERALSTRING *const *a, const ASN1_GENERALSTRING *const *b); typedef void (*sk_ASN1_GENERALSTRING_freefunc)(ASN1_GENERALSTRING * a); typedef ASN1_GENERALSTRING *(*sk_ASN1_GENERALSTRING_copyfunc)(const ASN1_GENERALSTRING *a); static inline void sk_ASN1_GENERALSTRING_freefunc_thunk(OPENSSL_sk_freefunc freefunc_arg, void *ptr) { sk_ASN1_GENERALSTRING_freefunc freefunc = (sk_ASN1_GENERALSTRING_freefunc)freefunc_arg; freefunc((ASN1_GENERALSTRING *)ptr); } static __attribute__((unused)) inline ASN1_GENERALSTRING *ossl_check_ASN1_GENERALSTRING_type(ASN1_GENERALSTRING *ptr) { return ptr; } static __attribute__((unused)) inline const OPENSSL_STACK *ossl_check_const_ASN1_GENERALSTRING_sk_type(const struct stack_st_ASN1_GENERALSTRING *sk) { return (const OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_STACK *ossl_check_ASN1_GENERALSTRING_sk_type(struct stack_st_ASN1_GENERALSTRING *sk) { return (OPENSSL_STACK *)sk; } static __attribute__((unused)) inline OPENSSL_sk_compfunc ossl_check_ASN1_GENERALSTRING_compfunc_type(sk_ASN1_GENERALSTRING_compfunc cmp) { return (OPENSSL_sk_compfunc)cmp; } static __attribute__((unused)) inline OPENSSL_sk_copyfunc ossl_check_ASN1_GENERALSTRING_copyfunc_type(sk_ASN1_GENERALSTRING_copyfunc cpy) { return (OPENSSL_sk_copyfunc)cpy; } static __attribute__((unused)) inline OPENSSL_sk_freefunc ossl_check_ASN1_GENERALSTRING_freefunc_type(sk_ASN1_GENERALSTRING_freefunc fr) { return (OPENSSL_sk_freefunc)fr; }
extern ASN1_STRING *ASN1_PRINTABLE_new(void); extern void ASN1_PRINTABLE_free(ASN1_STRING *a); extern ASN1_STRING *d2i_ASN1_PRINTABLE(ASN1_STRING **a, const unsigned char **in, long len); extern int i2d_ASN1_PRINTABLE(const ASN1_STRING *a, unsigned char **out); extern const ASN1_ITEM *ASN1_PRINTABLE_it(void);

extern ASN1_STRING *DIRECTORYSTRING_new(void); extern void DIRECTORYSTRING_free(ASN1_STRING *a); extern ASN1_STRING *d2i_DIRECTORYSTRING(ASN1_STRING **a, const unsigned char **in, long len); extern int i2d_DIRECTORYSTRING(const ASN1_STRING *a, unsigned char **out); extern const ASN1_ITEM *DIRECTORYSTRING_it(void);
extern ASN1_STRING *DISPLAYTEXT_new(void); extern void DISPLAYTEXT_free(ASN1_STRING *a); extern ASN1_STRING *d2i_DISPLAYTEXT(ASN1_STRING **a, const unsigned char **in, long len); extern int i2d_DISPLAYTEXT(const ASN1_STRING *a, unsigned char **out); extern const ASN1_ITEM *DISPLAYTEXT_it(void);
extern ASN1_PRINTABLESTRING *ASN1_PRINTABLESTRING_new(void); extern void ASN1_PRINTABLESTRING_free(ASN1_PRINTABLESTRING *a); extern ASN1_PRINTABLESTRING *d2i_ASN1_PRINTABLESTRING(ASN1_PRINTABLESTRING **a, const unsigned char **in, long len); extern int i2d_ASN1_PRINTABLESTRING(const ASN1_PRINTABLESTRING *a, unsigned char **out); extern const ASN1_ITEM *ASN1_PRINTABLESTRING_it(void);
extern ASN1_T61STRING *ASN1_T61STRING_new(void); extern void ASN1_T61STRING_free(ASN1_T61STRING *a); extern ASN1_T61STRING *d2i_ASN1_T61STRING(ASN1_T61STRING **a, const unsigned char **in, long len); extern int i2d_ASN1_T61STRING(const ASN1_T61STRING *a, unsigned char **out); extern const ASN1_ITEM *ASN1_T61STRING_it(void);
extern ASN1_IA5STRING *ASN1_IA5STRING_new(void); extern void ASN1_IA5STRING_free(ASN1_IA5STRING *a); extern ASN1_IA5STRING *d2i_ASN1_IA5STRING(ASN1_IA5STRING **a, const unsigned char **in, long len); extern int i2d_ASN1_IA5STRING(const ASN1_IA5STRING *a, unsigned char **out); extern const ASN1_ITEM *ASN1_IA5STRING_it(void);
extern ASN1_GENERALSTRING *ASN1_GENERALSTRING_new(void); extern void ASN1_GENERALSTRING_free(ASN1_GENERALSTRING *a); extern ASN1_GENERALSTRING *d2i_ASN1_GENERALSTRING(ASN1_GENERALSTRING **a, const unsigned char **in, long len); extern int i2d_ASN1_GENERALSTRING(const ASN1_GENERALSTRING *a, unsigned char **out); extern const ASN1_ITEM *ASN1_GENERALSTRING_it(void);
extern ASN1_UTCTIME *ASN1_UTCTIME_new(void); extern void ASN1_UTCTIME_free(ASN1_UTCTIME *a); extern ASN1_UTCTIME *d2i_ASN1_UTCTIME(ASN1_UTCTIME **a, const unsigned char **in, long len); extern int i2d_ASN1_UTCTIME(const ASN1_UTCTIME *a, unsigned char **out); extern const ASN1_ITEM *ASN1_UTCTIME_it(void);
extern ASN1_GENERALIZEDTIME *ASN1_GENERALIZEDTIME_new(void); extern void ASN1_GENERALIZEDTIME_free(ASN1_GENERALIZEDTIME *a); extern ASN1_GENERALIZEDTIME *d2i_ASN1_GENERALIZEDTIME(ASN1_GENERALIZEDTIME **a, const unsigned char **in, long len); extern int i2d_ASN1_GENERALIZEDTIME(const ASN1_GENERALIZEDTIME *a, unsigned char **out); extern const ASN1_ITEM *ASN1_GENERALIZEDTIME_it(void);
extern ASN1_TIME *ASN1_TIME_new(void); extern void ASN1_TIME_free(ASN1_TIME *a); extern ASN1_TIME *d2i_ASN1_TIME(ASN1_TIME **a, const unsigned char **in, long len); extern int i2d_ASN1_TIME(const ASN1_TIME *a, unsigned char **out); extern const ASN1_ITEM *ASN1_TIME_it(void);

extern ASN1_TIME *ASN1_TIME_dup(const ASN1_TIME *a);
extern ASN1_UTCTIME *ASN1_UTCTIME_dup(const ASN1_UTCTIME *a);
extern ASN1_GENERALIZEDTIME *ASN1_GENERALIZEDTIME_dup(const ASN1_GENERALIZEDTIME *a);

extern const ASN1_ITEM *ASN1_OCTET_STRING_NDEF_it(void);

ASN1_TIME *ASN1_TIME_set(ASN1_TIME *s, time_t t);
ASN1_TIME *ASN1_TIME_adj(ASN1_TIME *s, time_t t,
    int offset_day, long offset_sec);
int ASN1_TIME_check(const ASN1_TIME *t);
ASN1_GENERALIZEDTIME *ASN1_TIME_to_generalizedtime(const ASN1_TIME *t,
    ASN1_GENERALIZEDTIME **out);
int ASN1_TIME_set_string(ASN1_TIME *s, const char *str);
int ASN1_TIME_set_string_X509(ASN1_TIME *s, const char *str);
int ASN1_TIME_to_tm(const ASN1_TIME *s, struct tm *tm);
int ASN1_TIME_normalize(ASN1_TIME *s);
int ASN1_TIME_cmp_time_t(const ASN1_TIME *s, time_t t);
int ASN1_TIME_compare(const ASN1_TIME *a, const ASN1_TIME *b);

int i2a_ASN1_INTEGER(BIO *bp, const ASN1_INTEGER *a);
int a2i_ASN1_INTEGER(BIO *bp, ASN1_INTEGER *bs, char *buf, int size);
int i2a_ASN1_ENUMERATED(BIO *bp, const ASN1_ENUMERATED *a);
int a2i_ASN1_ENUMERATED(BIO *bp, ASN1_ENUMERATED *bs, char *buf, int size);
int i2a_ASN1_OBJECT(BIO *bp, const ASN1_OBJECT *a);
int a2i_ASN1_STRING(BIO *bp, ASN1_STRING *bs, char *buf, int size);
int i2a_ASN1_STRING(BIO *bp, const ASN1_STRING *a, int type);
int i2t_ASN1_OBJECT(char *buf, int buf_len, const ASN1_OBJECT *a);

int a2d_ASN1_OBJECT(unsigned char *out, int olen, const char *buf, int num);
ASN1_OBJECT *ASN1_OBJECT_create(int nid, unsigned char *data, int len,
    const char *sn, const char *ln);

int ASN1_INTEGER_get_int64(int64_t *pr, const ASN1_INTEGER *a);
int ASN1_INTEGER_set_int64(ASN1_INTEGER *a, int64_t r);
int ASN1_INTEGER_get_uint64(uint64_t *pr, const ASN1_INTEGER *a);
int ASN1_INTEGER_set_uint64(ASN1_INTEGER *a, uint64_t r);

int ASN1_INTEGER_set(ASN1_INTEGER *a, long v);
long ASN1_INTEGER_get(const ASN1_INTEGER *a);
ASN1_INTEGER *BN_to_ASN1_INTEGER(const BIGNUM *bn, ASN1_INTEGER *ai);
BIGNUM *ASN1_INTEGER_to_BN(const ASN1_INTEGER *ai, BIGNUM *bn);

int ASN1_ENUMERATED_get_int64(int64_t *pr, const ASN1_ENUMERATED *a);
int ASN1_ENUMERATED_set_int64(ASN1_ENUMERATED *a, int64_t r);

int ASN1_ENUMERATED_set(ASN1_ENUMERATED *a, long v);
long ASN1_ENUMERATED_get(const ASN1_ENUMERATED *a);
ASN1_ENUMERATED *BN_to_ASN1_ENUMERATED(const BIGNUM *bn, ASN1_ENUMERATED *ai);
BIGNUM *ASN1_ENUMERATED_to_BN(const ASN1_ENUMERATED *ai, BIGNUM *bn);



int ASN1_PRINTABLE_type(const unsigned char *s, int max);

unsigned long ASN1_tag2bit(int tag);


int ASN1_get_object(const unsigned char **pp, long *plength, int *ptag,
    int *pclass, long omax);
int ASN1_check_infinite_end(unsigned char **p, long len);
int ASN1_const_check_infinite_end(const unsigned char **p, long len);
void ASN1_put_object(unsigned char **pp, int constructed, int length,
    int tag, int xclass);
int ASN1_put_eoc(unsigned char **pp);
int ASN1_object_size(int constructed, int length, int tag);


void *ASN1_dup(i2d_of_void *i2d, d2i_of_void *d2i, const void *x);






void *ASN1_item_dup(const ASN1_ITEM *it, const void *x);
int ASN1_item_sign_ex(const ASN1_ITEM *it, X509_ALGOR *algor1,
    X509_ALGOR *algor2, ASN1_BIT_STRING *signature,
    const void *data, const ASN1_OCTET_STRING *id,
    EVP_PKEY *pkey, const EVP_MD *md, OSSL_LIB_CTX *libctx,
    const char *propq);
int ASN1_item_verify_ex(const ASN1_ITEM *it, const X509_ALGOR *alg,
    const ASN1_BIT_STRING *signature, const void *data,
    const ASN1_OCTET_STRING *id, EVP_PKEY *pkey,
    OSSL_LIB_CTX *libctx, const char *propq);
void *ASN1_d2i_fp(void *(*xnew)(void), d2i_of_void *d2i, FILE *in, void **x);







void *ASN1_item_d2i_fp_ex(const ASN1_ITEM *it, FILE *in, void *x,
    OSSL_LIB_CTX *libctx, const char *propq);
void *ASN1_item_d2i_fp(const ASN1_ITEM *it, FILE *in, void *x);
int ASN1_i2d_fp(i2d_of_void *i2d, FILE *out, const void *x);






int ASN1_item_i2d_fp(const ASN1_ITEM *it, FILE *out, const void *x);
int ASN1_STRING_print_ex_fp(FILE *fp, const ASN1_STRING *str, unsigned long flags);


int ASN1_STRING_to_UTF8(unsigned char **out, const ASN1_STRING *in);

void *ASN1_d2i_bio(void *(*xnew)(void), d2i_of_void *d2i, BIO *in, void **x);







void *ASN1_item_d2i_bio_ex(const ASN1_ITEM *it, BIO *in, void *pval,
    OSSL_LIB_CTX *libctx, const char *propq);
void *ASN1_item_d2i_bio(const ASN1_ITEM *it, BIO *in, void *pval);
int ASN1_i2d_bio(i2d_of_void *i2d, BIO *out, const void *x);






int ASN1_item_i2d_bio(const ASN1_ITEM *it, BIO *out, const void *x);
BIO *ASN1_item_i2d_mem_bio(const ASN1_ITEM *it, const ASN1_VALUE *val);
int ASN1_UTCTIME_print(BIO *fp, const ASN1_UTCTIME *a);
int ASN1_GENERALIZEDTIME_print(BIO *fp, const ASN1_GENERALIZEDTIME *a);
int ASN1_TIME_print(BIO *bp, const ASN1_TIME *tm);
int ASN1_TIME_print_ex(BIO *bp, const ASN1_TIME *tm, unsigned long flags);
int ASN1_STRING_print(BIO *bp, const ASN1_STRING *v);
int ASN1_STRING_print_ex(BIO *out, const ASN1_STRING *str, unsigned long flags);
int ASN1_buf_print(BIO *bp, const unsigned char *buf, size_t buflen, int off);
int ASN1_bn_print(BIO *bp, const char *number, const BIGNUM *num,
    unsigned char *buf, int off);
int ASN1_parse(BIO *bp, const unsigned char *pp, long len, int indent);
int ASN1_parse_dump(BIO *bp, const unsigned char *pp, long len, int indent,
    int dump);
const char *ASN1_tag2str(int tag);



int ASN1_UNIVERSALSTRING_to_string(ASN1_UNIVERSALSTRING *s);

int ASN1_TYPE_set_octetstring(ASN1_TYPE *a, unsigned char *data, int len);
int ASN1_TYPE_get_octetstring(const ASN1_TYPE *a, unsigned char *data, int max_len);
int ASN1_TYPE_set_int_octetstring(ASN1_TYPE *a, long num,
    unsigned char *data, int len);
int ASN1_TYPE_get_int_octetstring(const ASN1_TYPE *a, long *num,
    unsigned char *data, int max_len);

void *ASN1_item_unpack(const ASN1_STRING *oct, const ASN1_ITEM *it);
void *ASN1_item_unpack_ex(const ASN1_STRING *oct, const ASN1_ITEM *it,
    OSSL_LIB_CTX *libctx, const char *propq);

ASN1_STRING *ASN1_item_pack(void *obj, const ASN1_ITEM *it,
    ASN1_OCTET_STRING **oct);

void ASN1_STRING_set_default_mask(unsigned long mask);
int ASN1_STRING_set_default_mask_asc(const char *p);
unsigned long ASN1_STRING_get_default_mask(void);
int ASN1_mbstring_copy(ASN1_STRING **out, const unsigned char *in, int len,
    int inform, unsigned long mask);
int ASN1_mbstring_ncopy(ASN1_STRING **out, const unsigned char *in, int len,
    int inform, unsigned long mask,
    long minsize, long maxsize);

ASN1_STRING *ASN1_STRING_set_by_NID(ASN1_STRING **out,
    const unsigned char *in, int inlen,
    int inform, int nid);
ASN1_STRING_TABLE *ASN1_STRING_TABLE_get(int nid);
int ASN1_STRING_TABLE_add(int, long, long, unsigned long, unsigned long);
void ASN1_STRING_TABLE_cleanup(void);




ASN1_VALUE *ASN1_item_new(const ASN1_ITEM *it);
ASN1_VALUE *ASN1_item_new_ex(const ASN1_ITEM *it, OSSL_LIB_CTX *libctx,
    const char *propq);
void ASN1_item_free(ASN1_VALUE *val, const ASN1_ITEM *it);
ASN1_VALUE *ASN1_item_d2i_ex(ASN1_VALUE **val, const unsigned char **in,
    long len, const ASN1_ITEM *it,
    OSSL_LIB_CTX *libctx, const char *propq);
ASN1_VALUE *ASN1_item_d2i(ASN1_VALUE **val, const unsigned char **in,
    long len, const ASN1_ITEM *it);
int ASN1_item_i2d(const ASN1_VALUE *val, unsigned char **out, const ASN1_ITEM *it);
int ASN1_item_ndef_i2d(const ASN1_VALUE *val, unsigned char **out,
    const ASN1_ITEM *it);

void ASN1_add_oid_module(void);
void ASN1_add_stable_module(void);

ASN1_TYPE *ASN1_generate_nconf(const char *str, CONF *nconf);
ASN1_TYPE *ASN1_generate_v3(const char *str, X509V3_CTX *cnf);
int ASN1_str2mask(const char *str, unsigned long *pmask);
int ASN1_item_print(BIO *out, const ASN1_VALUE *ifld, int indent,
    const ASN1_ITEM *it, const ASN1_PCTX *pctx);
ASN1_PCTX *ASN1_PCTX_new(void);
void ASN1_PCTX_free(ASN1_PCTX *p);
unsigned long ASN1_PCTX_get_flags(const ASN1_PCTX *p);
void ASN1_PCTX_set_flags(ASN1_PCTX *p, unsigned long flags);
unsigned long ASN1_PCTX_get_nm_flags(const ASN1_PCTX *p);
void ASN1_PCTX_set_nm_flags(ASN1_PCTX *p, unsigned long flags);
unsigned long ASN1_PCTX_get_cert_flags(const ASN1_PCTX *p);
void ASN1_PCTX_set_cert_flags(ASN1_PCTX *p, unsigned long flags);
unsigned long ASN1_PCTX_get_oid_flags(const ASN1_PCTX *p);
void ASN1_PCTX_set_oid_flags(ASN1_PCTX *p, unsigned long flags);
unsigned long ASN1_PCTX_get_str_flags(const ASN1_PCTX *p);
void ASN1_PCTX_set_str_flags(ASN1_PCTX *p, unsigned long flags);

ASN1_SCTX *ASN1_SCTX_new(int (*scan_cb)(ASN1_SCTX *ctx));
void ASN1_SCTX_free(ASN1_SCTX *p);
const ASN1_ITEM *ASN1_SCTX_get_item(ASN1_SCTX *p);
const ASN1_TEMPLATE *ASN1_SCTX_get_template(ASN1_SCTX *p);
unsigned long ASN1_SCTX_get_flags(ASN1_SCTX *p);
void ASN1_SCTX_set_app_data(ASN1_SCTX *p, void *data);
void *ASN1_SCTX_get_app_data(ASN1_SCTX *p);

const BIO_METHOD *BIO_f_asn1(void);


BIO *BIO_new_NDEF(BIO *out, ASN1_VALUE *val, const ASN1_ITEM *it);

int i2d_ASN1_bio_stream(BIO *out, ASN1_VALUE *val, BIO *in, int flags,
    const ASN1_ITEM *it);
int PEM_write_bio_ASN1_stream(BIO *out, ASN1_VALUE *val, BIO *in, int flags,
    const char *hdr, const ASN1_ITEM *it);

int SMIME_write_ASN1(BIO *bio, ASN1_VALUE *val, BIO *data, int flags,
    int ctype_nid, int econt_nid,
    struct stack_st_X509_ALGOR *mdalgs, const ASN1_ITEM *it);
int SMIME_write_ASN1_ex(BIO *bio, ASN1_VALUE *val, BIO *data, int flags,
    int ctype_nid, int econt_nid,
    struct stack_st_X509_ALGOR *mdalgs, const ASN1_ITEM *it,
    OSSL_LIB_CTX *libctx, const char *propq);
ASN1_VALUE *SMIME_read_ASN1(BIO *bio, BIO **bcont, const ASN1_ITEM *it);
ASN1_VALUE *SMIME_read_ASN1_ex(BIO *bio, int flags, BIO **bcont,
    const ASN1_ITEM *it, ASN1_VALUE **x,
    OSSL_LIB_CTX *libctx, const char *propq);
int SMIME_crlf_copy(BIO *in, BIO *out, int flags);
int SMIME_text(BIO *in, BIO *out);

const ASN1_ITEM *ASN1_ITEM_lookup(const char *name);
const ASN1_ITEM *ASN1_ITEM_get(size_t i);
       
typedef struct obj_name_st {
    int type;
    int alias;
    const char *name;
    const char *data;
} OBJ_NAME;



int OBJ_NAME_init(void);
int OBJ_NAME_new_index(unsigned long (*hash_func)(const char *),
    int (*cmp_func)(const char *, const char *),
    void (*free_func)(const char *, int, const char *));
const char *OBJ_NAME_get(const char *name, int type);
int OBJ_NAME_add(const char *name, int type, const char *data);
int OBJ_NAME_remove(const char *name, int type);
void OBJ_NAME_cleanup(int type);
void OBJ_NAME_do_all(int type, void (*fn)(const OBJ_NAME *, void *arg),
    void *arg);
void OBJ_NAME_do_all_sorted(int type,
    void (*fn)(const OBJ_NAME *, void *arg),
    void *arg);

extern ASN1_OBJECT *OBJ_dup(const ASN1_OBJECT *a);
ASN1_OBJECT *OBJ_nid2obj(int n);
const char *OBJ_nid2ln(int n);
const char *OBJ_nid2sn(int n);
int OBJ_obj2nid(const ASN1_OBJECT *o);
ASN1_OBJECT *OBJ_txt2obj(const char *s, int no_name);
int OBJ_obj2txt(char *buf, int buf_len, const ASN1_OBJECT *a, int no_name);
int OBJ_txt2nid(const char *s);
int OBJ_ln2nid(const char *s);
int OBJ_sn2nid(const char *s);
int OBJ_cmp(const ASN1_OBJECT *a, const ASN1_OBJECT *b);
const void *OBJ_bsearch_(const void *key, const void *base, int num, int size,
    int (*cmp)(const void *, const void *));
const void *OBJ_bsearch_ex_(const void *key, const void *base, int num,
    int size,
    int (*cmp)(const void *, const void *),
    int flags);
int OBJ_new_nid(int num);
int OBJ_add_object(const ASN1_OBJECT *obj);
int OBJ_create(const char *oid, const char *sn, const char *ln);





int OBJ_create_objects(BIO *in);

size_t OBJ_length(const ASN1_OBJECT *obj);
const unsigned char *OBJ_get0_data(const ASN1_OBJECT *obj);

int OBJ_find_sigid_algs(int signid, int *pdig_nid, int *ppkey_nid);
int OBJ_find_sigid_by_algs(int *psignid, int dig_nid, int pkey_nid);
int OBJ_add_sigid(int signid, int dig_id, int pkey_id);
void OBJ_sigid_free(void);
int EVP_set_default_properties(OSSL_LIB_CTX *libctx, const char *propq);
char *EVP_get1_default_properties(OSSL_LIB_CTX *libctx);
int EVP_default_properties_is_fips_enabled(OSSL_LIB_CTX *libctx);
int EVP_default_properties_enable_fips(OSSL_LIB_CTX *libctx, int enable);
__attribute__((deprecated("Since OpenSSL " "3.0"))) EVP_MD *EVP_MD_meth_new(int md_type, int pkey_type);
__attribute__((deprecated("Since OpenSSL " "3.0"))) EVP_MD *EVP_MD_meth_dup(const EVP_MD *md);
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_MD_meth_free(EVP_MD *md);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_MD_meth_set_input_blocksize(EVP_MD *md, int blocksize);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_MD_meth_set_result_size(EVP_MD *md, int resultsize);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_MD_meth_set_app_datasize(EVP_MD *md, int datasize);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_MD_meth_set_flags(EVP_MD *md, unsigned long flags);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_MD_meth_set_init(EVP_MD *md, int (*init)(EVP_MD_CTX *ctx));
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_MD_meth_set_update(EVP_MD *md, int (*update)(EVP_MD_CTX *ctx, const void *data, size_t count));
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_MD_meth_set_final(EVP_MD *md, int (*final)(EVP_MD_CTX *ctx, unsigned char *md));
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_MD_meth_set_copy(EVP_MD *md, int (*copy)(EVP_MD_CTX *to, const EVP_MD_CTX *from));
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_MD_meth_set_cleanup(EVP_MD *md, int (*cleanup)(EVP_MD_CTX *ctx));
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_MD_meth_set_ctrl(EVP_MD *md, int (*ctrl)(EVP_MD_CTX *ctx, int cmd, int p1, void *p2));
__attribute__((deprecated("Since OpenSSL " "3.0"))) int EVP_MD_meth_get_input_blocksize(const EVP_MD *md);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int EVP_MD_meth_get_result_size(const EVP_MD *md);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int EVP_MD_meth_get_app_datasize(const EVP_MD *md);
__attribute__((deprecated("Since OpenSSL " "3.0"))) unsigned long EVP_MD_meth_get_flags(const EVP_MD *md);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int (*EVP_MD_meth_get_init(const EVP_MD *md))(EVP_MD_CTX *ctx);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int (*EVP_MD_meth_get_update(const EVP_MD *md))(EVP_MD_CTX *ctx,
    const void *data, size_t count);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int (*EVP_MD_meth_get_final(const EVP_MD *md))(EVP_MD_CTX *ctx,
    unsigned char *md);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int (*EVP_MD_meth_get_copy(const EVP_MD *md))(EVP_MD_CTX *to,
    const EVP_MD_CTX *from);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int (*EVP_MD_meth_get_cleanup(const EVP_MD *md))(EVP_MD_CTX *ctx);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int (*EVP_MD_meth_get_ctrl(const EVP_MD *md))(EVP_MD_CTX *ctx, int cmd,
    int p1, void *p2);
__attribute__((deprecated("Since OpenSSL " "3.0")))
EVP_CIPHER *EVP_CIPHER_meth_new(int cipher_type, int block_size, int key_len);
__attribute__((deprecated("Since OpenSSL " "3.0")))
EVP_CIPHER *EVP_CIPHER_meth_dup(const EVP_CIPHER *cipher);
__attribute__((deprecated("Since OpenSSL " "3.0")))
void EVP_CIPHER_meth_free(EVP_CIPHER *cipher);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_CIPHER_meth_set_iv_length(EVP_CIPHER *cipher, int iv_len);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_CIPHER_meth_set_flags(EVP_CIPHER *cipher, unsigned long flags);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_CIPHER_meth_set_impl_ctx_size(EVP_CIPHER *cipher, int ctx_size);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_CIPHER_meth_set_init(EVP_CIPHER *cipher,
    int (*init)(EVP_CIPHER_CTX *ctx,
        const unsigned char *key,
        const unsigned char *iv,
        int enc));
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_CIPHER_meth_set_do_cipher(EVP_CIPHER *cipher,
    int (*do_cipher)(EVP_CIPHER_CTX *ctx,
        unsigned char *out,
        const unsigned char *in,
        size_t inl));
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_CIPHER_meth_set_cleanup(EVP_CIPHER *cipher,
    int (*cleanup)(EVP_CIPHER_CTX *));
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_CIPHER_meth_set_set_asn1_params(EVP_CIPHER *cipher,
    int (*set_asn1_parameters)(EVP_CIPHER_CTX *,
        ASN1_TYPE *));
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_CIPHER_meth_set_get_asn1_params(EVP_CIPHER *cipher,
    int (*get_asn1_parameters)(EVP_CIPHER_CTX *,
        ASN1_TYPE *));
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_CIPHER_meth_set_ctrl(EVP_CIPHER *cipher,
    int (*ctrl)(EVP_CIPHER_CTX *, int type,
        int arg, void *ptr));
__attribute__((deprecated("Since OpenSSL " "3.0"))) int (*EVP_CIPHER_meth_get_init(const EVP_CIPHER *cipher))(EVP_CIPHER_CTX *ctx,
    const unsigned char *key,
    const unsigned char *iv,
    int enc);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int (*EVP_CIPHER_meth_get_do_cipher(const EVP_CIPHER *cipher))(EVP_CIPHER_CTX *ctx,
    unsigned char *out,
    const unsigned char *in,
    size_t inl);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int (*EVP_CIPHER_meth_get_cleanup(const EVP_CIPHER *cipher))(EVP_CIPHER_CTX *);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int (*EVP_CIPHER_meth_get_set_asn1_params(const EVP_CIPHER *cipher))(EVP_CIPHER_CTX *,
    ASN1_TYPE *);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int (*EVP_CIPHER_meth_get_get_asn1_params(const EVP_CIPHER *cipher))(EVP_CIPHER_CTX *,
    ASN1_TYPE *);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int (*EVP_CIPHER_meth_get_ctrl(const EVP_CIPHER *cipher))(EVP_CIPHER_CTX *, int type,
    int arg, void *ptr);
typedef struct {
    unsigned char *out;
    const unsigned char *inp;
    size_t len;
    unsigned int interleave;
} EVP_CTRL_TLS1_1_MULTIBLOCK_PARAM;
typedef struct evp_cipher_info_st {
    const EVP_CIPHER *cipher;
    unsigned char iv[16];
} EVP_CIPHER_INFO;


typedef int(EVP_PBE_KEYGEN)(EVP_CIPHER_CTX *ctx, const char *pass,
    int passlen, ASN1_TYPE *param,
    const EVP_CIPHER *cipher, const EVP_MD *md,
    int en_de);

typedef int(EVP_PBE_KEYGEN_EX)(EVP_CIPHER_CTX *ctx, const char *pass,
    int passlen, ASN1_TYPE *param,
    const EVP_CIPHER *cipher, const EVP_MD *md,
    int en_de, OSSL_LIB_CTX *libctx, const char *propq);
int EVP_MD_get_type(const EVP_MD *md);


const char *EVP_MD_get0_name(const EVP_MD *md);

const char *EVP_MD_get0_description(const EVP_MD *md);
int EVP_MD_is_a(const EVP_MD *md, const char *name);
int EVP_MD_names_do_all(const EVP_MD *md,
    void (*fn)(const char *name, void *data),
    void *data);
const OSSL_PROVIDER *EVP_MD_get0_provider(const EVP_MD *md);
int EVP_MD_get_pkey_type(const EVP_MD *md);

int EVP_MD_get_size(const EVP_MD *md);

int EVP_MD_get_block_size(const EVP_MD *md);

unsigned long EVP_MD_get_flags(const EVP_MD *md);

int EVP_MD_xof(const EVP_MD *md);

const EVP_MD *EVP_MD_CTX_get0_md(const EVP_MD_CTX *ctx);
EVP_MD *EVP_MD_CTX_get1_md(EVP_MD_CTX *ctx);

__attribute__((deprecated("Since OpenSSL " "3.0")))
const EVP_MD *EVP_MD_CTX_md(const EVP_MD_CTX *ctx);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int (*EVP_MD_CTX_update_fn(EVP_MD_CTX *ctx))(EVP_MD_CTX *ctx,
    const void *data, size_t count);
__attribute__((deprecated("Since OpenSSL " "3.0")))
void EVP_MD_CTX_set_update_fn(EVP_MD_CTX *ctx,
    int (*update)(EVP_MD_CTX *ctx,
        const void *data, size_t count));

int EVP_MD_CTX_get_size_ex(const EVP_MD_CTX *ctx);
EVP_PKEY_CTX *EVP_MD_CTX_get_pkey_ctx(const EVP_MD_CTX *ctx);

void EVP_MD_CTX_set_pkey_ctx(EVP_MD_CTX *ctx, EVP_PKEY_CTX *pctx);
void *EVP_MD_CTX_get0_md_data(const EVP_MD_CTX *ctx);


int EVP_CIPHER_get_nid(const EVP_CIPHER *cipher);

const char *EVP_CIPHER_get0_name(const EVP_CIPHER *cipher);

const char *EVP_CIPHER_get0_description(const EVP_CIPHER *cipher);
int EVP_CIPHER_is_a(const EVP_CIPHER *cipher, const char *name);
int EVP_CIPHER_names_do_all(const EVP_CIPHER *cipher,
    void (*fn)(const char *name, void *data),
    void *data);
const OSSL_PROVIDER *EVP_CIPHER_get0_provider(const EVP_CIPHER *cipher);
int EVP_CIPHER_get_block_size(const EVP_CIPHER *cipher);


__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_CIPHER_impl_ctx_size(const EVP_CIPHER *cipher);

int EVP_CIPHER_get_key_length(const EVP_CIPHER *cipher);

int EVP_CIPHER_get_iv_length(const EVP_CIPHER *cipher);

unsigned long EVP_CIPHER_get_flags(const EVP_CIPHER *cipher);

int EVP_CIPHER_get_mode(const EVP_CIPHER *cipher);

int EVP_CIPHER_get_type(const EVP_CIPHER *cipher);

EVP_CIPHER *EVP_CIPHER_fetch(OSSL_LIB_CTX *ctx, const char *algorithm,
    const char *properties);
int EVP_CIPHER_can_pipeline(const EVP_CIPHER *cipher, int enc);
int EVP_CIPHER_up_ref(EVP_CIPHER *cipher);
void EVP_CIPHER_free(EVP_CIPHER *cipher);

const EVP_CIPHER *EVP_CIPHER_CTX_get0_cipher(const EVP_CIPHER_CTX *ctx);
EVP_CIPHER *EVP_CIPHER_CTX_get1_cipher(EVP_CIPHER_CTX *ctx);
int EVP_CIPHER_CTX_is_encrypting(const EVP_CIPHER_CTX *ctx);

int EVP_CIPHER_CTX_get_nid(const EVP_CIPHER_CTX *ctx);

int EVP_CIPHER_CTX_get_block_size(const EVP_CIPHER_CTX *ctx);

int EVP_CIPHER_CTX_get_key_length(const EVP_CIPHER_CTX *ctx);

int EVP_CIPHER_CTX_get_iv_length(const EVP_CIPHER_CTX *ctx);

int EVP_CIPHER_CTX_get_tag_length(const EVP_CIPHER_CTX *ctx);


const EVP_CIPHER *EVP_CIPHER_CTX_cipher(const EVP_CIPHER_CTX *ctx);
__attribute__((deprecated("Since OpenSSL " "3.0"))) const unsigned char *EVP_CIPHER_CTX_iv(const EVP_CIPHER_CTX *ctx);
__attribute__((deprecated("Since OpenSSL " "3.0"))) const unsigned char *EVP_CIPHER_CTX_original_iv(const EVP_CIPHER_CTX *ctx);
__attribute__((deprecated("Since OpenSSL " "3.0"))) unsigned char *EVP_CIPHER_CTX_iv_noconst(EVP_CIPHER_CTX *ctx);

int EVP_CIPHER_CTX_get_updated_iv(EVP_CIPHER_CTX *ctx, void *buf, size_t len);
int EVP_CIPHER_CTX_get_original_iv(EVP_CIPHER_CTX *ctx, void *buf, size_t len);

__attribute__((deprecated("Since OpenSSL " "3.0")))
unsigned char *EVP_CIPHER_CTX_buf_noconst(EVP_CIPHER_CTX *ctx);

int EVP_CIPHER_CTX_get_num(const EVP_CIPHER_CTX *ctx);

int EVP_CIPHER_CTX_set_num(EVP_CIPHER_CTX *ctx, int num);
EVP_CIPHER_CTX *EVP_CIPHER_CTX_dup(const EVP_CIPHER_CTX *in);
int EVP_CIPHER_CTX_copy(EVP_CIPHER_CTX *out, const EVP_CIPHER_CTX *in);
void *EVP_CIPHER_CTX_get_app_data(const EVP_CIPHER_CTX *ctx);
void EVP_CIPHER_CTX_set_app_data(EVP_CIPHER_CTX *ctx, void *data);
void *EVP_CIPHER_CTX_get_cipher_data(const EVP_CIPHER_CTX *ctx);
void *EVP_CIPHER_CTX_set_cipher_data(EVP_CIPHER_CTX *ctx, void *cipher_data);
 int EVP_Cipher(EVP_CIPHER_CTX *c,
    unsigned char *out,
    const unsigned char *in, unsigned int inl);
int EVP_MD_get_params(const EVP_MD *digest, OSSL_PARAM params[]);
int EVP_MD_CTX_set_params(EVP_MD_CTX *ctx, const OSSL_PARAM params[]);
int EVP_MD_CTX_get_params(EVP_MD_CTX *ctx, OSSL_PARAM params[]);
const OSSL_PARAM *EVP_MD_gettable_params(const EVP_MD *digest);
const OSSL_PARAM *EVP_MD_settable_ctx_params(const EVP_MD *md);
const OSSL_PARAM *EVP_MD_gettable_ctx_params(const EVP_MD *md);
const OSSL_PARAM *EVP_MD_CTX_settable_params(EVP_MD_CTX *ctx);
const OSSL_PARAM *EVP_MD_CTX_gettable_params(EVP_MD_CTX *ctx);
int EVP_MD_CTX_ctrl(EVP_MD_CTX *ctx, int cmd, int p1, void *p2);
EVP_MD_CTX *EVP_MD_CTX_new(void);
int EVP_MD_CTX_reset(EVP_MD_CTX *ctx);
void EVP_MD_CTX_free(EVP_MD_CTX *ctx);



 EVP_MD_CTX *EVP_MD_CTX_dup(const EVP_MD_CTX *in);
 int EVP_MD_CTX_copy_ex(EVP_MD_CTX *out, const EVP_MD_CTX *in);
void EVP_MD_CTX_set_flags(EVP_MD_CTX *ctx, int flags);
void EVP_MD_CTX_clear_flags(EVP_MD_CTX *ctx, int flags);
int EVP_MD_CTX_test_flags(const EVP_MD_CTX *ctx, int flags);
 int EVP_DigestInit_ex2(EVP_MD_CTX *ctx, const EVP_MD *type,
    const OSSL_PARAM params[]);
 int EVP_DigestInit_ex(EVP_MD_CTX *ctx, const EVP_MD *type,
    ENGINE *impl);
 int EVP_DigestUpdate(EVP_MD_CTX *ctx, const void *d,
    size_t cnt);
 int EVP_DigestFinal_ex(EVP_MD_CTX *ctx, unsigned char *md,
    unsigned int *s);
 int EVP_Digest(const void *data, size_t count,
    unsigned char *md, unsigned int *size,
    const EVP_MD *type, ENGINE *impl);
 int EVP_Q_digest(OSSL_LIB_CTX *libctx, const char *name,
    const char *propq, const void *data, size_t datalen,
    unsigned char *md, size_t *mdlen);

 int EVP_MD_CTX_copy(EVP_MD_CTX *out, const EVP_MD_CTX *in);
 int EVP_DigestInit(EVP_MD_CTX *ctx, const EVP_MD *type);
 int EVP_DigestFinal(EVP_MD_CTX *ctx, unsigned char *md,
    unsigned int *s);
 int EVP_DigestFinalXOF(EVP_MD_CTX *ctx, unsigned char *out,
    size_t outlen);
 int EVP_DigestSqueeze(EVP_MD_CTX *ctx, unsigned char *out,
    size_t outlen);

 EVP_MD *EVP_MD_fetch(OSSL_LIB_CTX *ctx, const char *algorithm,
    const char *properties);

int EVP_MD_up_ref(EVP_MD *md);
void EVP_MD_free(EVP_MD *md);

int EVP_read_pw_string(char *buf, int length, const char *prompt, int verify);
int EVP_read_pw_string_min(char *buf, int minlen, int maxlen,
    const char *prompt, int verify);
void EVP_set_pw_prompt(const char *prompt);
char *EVP_get_pw_prompt(void);

 int EVP_BytesToKey(const EVP_CIPHER *type, const EVP_MD *md,
    const unsigned char *salt,
    const unsigned char *data, int datal, int count,
    unsigned char *key, unsigned char *iv);

void EVP_CIPHER_CTX_set_flags(EVP_CIPHER_CTX *ctx, int flags);
void EVP_CIPHER_CTX_clear_flags(EVP_CIPHER_CTX *ctx, int flags);
int EVP_CIPHER_CTX_test_flags(const EVP_CIPHER_CTX *ctx, int flags);

 int EVP_EncryptInit(EVP_CIPHER_CTX *ctx, const EVP_CIPHER *cipher,
    const unsigned char *key, const unsigned char *iv);
 int EVP_EncryptInit_ex(EVP_CIPHER_CTX *ctx,
    const EVP_CIPHER *cipher, ENGINE *impl,
    const unsigned char *key,
    const unsigned char *iv);
 int EVP_EncryptInit_ex2(EVP_CIPHER_CTX *ctx, const EVP_CIPHER *cipher,
    const unsigned char *key,
    const unsigned char *iv,
    const OSSL_PARAM params[]);
 int EVP_EncryptUpdate(EVP_CIPHER_CTX *ctx, unsigned char *out,
    int *outl, const unsigned char *in, int inl);
 int EVP_EncryptFinal_ex(EVP_CIPHER_CTX *ctx, unsigned char *out,
    int *outl);
 int EVP_EncryptFinal(EVP_CIPHER_CTX *ctx, unsigned char *out,
    int *outl);

 int EVP_DecryptInit(EVP_CIPHER_CTX *ctx, const EVP_CIPHER *cipher,
    const unsigned char *key, const unsigned char *iv);
 int EVP_DecryptInit_ex(EVP_CIPHER_CTX *ctx,
    const EVP_CIPHER *cipher, ENGINE *impl,
    const unsigned char *key,
    const unsigned char *iv);
 int EVP_DecryptInit_ex2(EVP_CIPHER_CTX *ctx, const EVP_CIPHER *cipher,
    const unsigned char *key,
    const unsigned char *iv,
    const OSSL_PARAM params[]);
 int EVP_DecryptUpdate(EVP_CIPHER_CTX *ctx, unsigned char *out,
    int *outl, const unsigned char *in, int inl);
 int EVP_DecryptFinal(EVP_CIPHER_CTX *ctx, unsigned char *outm,
    int *outl);
 int EVP_DecryptFinal_ex(EVP_CIPHER_CTX *ctx, unsigned char *outm,
    int *outl);

 int EVP_CipherInit(EVP_CIPHER_CTX *ctx, const EVP_CIPHER *cipher,
    const unsigned char *key, const unsigned char *iv,
    int enc);
 int EVP_CipherInit_ex(EVP_CIPHER_CTX *ctx,
    const EVP_CIPHER *cipher, ENGINE *impl,
    const unsigned char *key,
    const unsigned char *iv, int enc);
 int EVP_CipherInit_SKEY(EVP_CIPHER_CTX *ctx, const EVP_CIPHER *cipher,
    EVP_SKEY *skey, const unsigned char *iv, size_t iv_len,
    int enc, const OSSL_PARAM params[]);
 int EVP_CipherInit_ex2(EVP_CIPHER_CTX *ctx, const EVP_CIPHER *cipher,
    const unsigned char *key, const unsigned char *iv,
    int enc, const OSSL_PARAM params[]);
 int EVP_CipherUpdate(EVP_CIPHER_CTX *ctx, unsigned char *out,
    int *outl, const unsigned char *in, int inl);
 int EVP_CipherFinal(EVP_CIPHER_CTX *ctx, unsigned char *outm,
    int *outl);
 int EVP_CipherPipelineEncryptInit(EVP_CIPHER_CTX *ctx,
    const EVP_CIPHER *cipher,
    const unsigned char *key, size_t keylen,
    size_t numpipes,
    const unsigned char **iv, size_t ivlen);
 int EVP_CipherPipelineDecryptInit(EVP_CIPHER_CTX *ctx,
    const EVP_CIPHER *cipher,
    const unsigned char *key, size_t keylen,
    size_t numpipes,
    const unsigned char **iv, size_t ivlen);
 int EVP_CipherPipelineUpdate(EVP_CIPHER_CTX *ctx,
    unsigned char **out, size_t *outl,
    const size_t *outsize,
    const unsigned char **in, const size_t *inl);
 int EVP_CipherPipelineFinal(EVP_CIPHER_CTX *ctx,
    unsigned char **outm, size_t *outl,
    const size_t *outsize);
 int EVP_CipherFinal_ex(EVP_CIPHER_CTX *ctx, unsigned char *outm,
    int *outl);

 int EVP_SignFinal(EVP_MD_CTX *ctx, unsigned char *md, unsigned int *s,
    EVP_PKEY *pkey);
 int EVP_SignFinal_ex(EVP_MD_CTX *ctx, unsigned char *md, unsigned int *s,
    EVP_PKEY *pkey, OSSL_LIB_CTX *libctx,
    const char *propq);

 int EVP_DigestSign(EVP_MD_CTX *ctx, unsigned char *sigret,
    size_t *siglen, const unsigned char *tbs,
    size_t tbslen);

 int EVP_VerifyFinal(EVP_MD_CTX *ctx, const unsigned char *sigbuf,
    unsigned int siglen, EVP_PKEY *pkey);
 int EVP_VerifyFinal_ex(EVP_MD_CTX *ctx, const unsigned char *sigbuf,
    unsigned int siglen, EVP_PKEY *pkey,
    OSSL_LIB_CTX *libctx, const char *propq);

 int EVP_DigestVerify(EVP_MD_CTX *ctx, const unsigned char *sigret,
    size_t siglen, const unsigned char *tbs,
    size_t tbslen);

 int EVP_DigestSignInit_ex(EVP_MD_CTX *ctx, EVP_PKEY_CTX **pctx,
    const char *mdname, OSSL_LIB_CTX *libctx,
    const char *props, EVP_PKEY *pkey,
    const OSSL_PARAM params[]);
 int EVP_DigestSignInit(EVP_MD_CTX *ctx, EVP_PKEY_CTX **pctx,
    const EVP_MD *type, ENGINE *e,
    EVP_PKEY *pkey);
 int EVP_DigestSignUpdate(EVP_MD_CTX *ctx, const void *data, size_t dsize);
 int EVP_DigestSignFinal(EVP_MD_CTX *ctx, unsigned char *sigret,
    size_t *siglen);

 int EVP_DigestVerifyInit_ex(EVP_MD_CTX *ctx, EVP_PKEY_CTX **pctx,
    const char *mdname, OSSL_LIB_CTX *libctx,
    const char *props, EVP_PKEY *pkey,
    const OSSL_PARAM params[]);
 int EVP_DigestVerifyInit(EVP_MD_CTX *ctx, EVP_PKEY_CTX **pctx,
    const EVP_MD *type, ENGINE *e,
    EVP_PKEY *pkey);
int EVP_DigestVerifyUpdate(EVP_MD_CTX *ctx, const void *data, size_t dsize);
 int EVP_DigestVerifyFinal(EVP_MD_CTX *ctx, const unsigned char *sig,
    size_t siglen);

 int EVP_OpenInit(EVP_CIPHER_CTX *ctx, const EVP_CIPHER *type,
    const unsigned char *ek, int ekl,
    const unsigned char *iv, EVP_PKEY *priv);
 int EVP_OpenFinal(EVP_CIPHER_CTX *ctx, unsigned char *out, int *outl);

 int EVP_SealInit(EVP_CIPHER_CTX *ctx, const EVP_CIPHER *type,
    unsigned char **ek, int *ekl, unsigned char *iv,
    EVP_PKEY **pubk, int npubk);
 int EVP_SealFinal(EVP_CIPHER_CTX *ctx, unsigned char *out, int *outl);

EVP_ENCODE_CTX *EVP_ENCODE_CTX_new(void);
void EVP_ENCODE_CTX_free(EVP_ENCODE_CTX *ctx);
int EVP_ENCODE_CTX_copy(EVP_ENCODE_CTX *dctx, const EVP_ENCODE_CTX *sctx);
int EVP_ENCODE_CTX_num(EVP_ENCODE_CTX *ctx);
void EVP_EncodeInit(EVP_ENCODE_CTX *ctx);
int EVP_EncodeUpdate(EVP_ENCODE_CTX *ctx, unsigned char *out, int *outl,
    const unsigned char *in, int inl);
void EVP_EncodeFinal(EVP_ENCODE_CTX *ctx, unsigned char *out, int *outl);
int EVP_EncodeBlock(unsigned char *t, const unsigned char *f, int n);

void EVP_DecodeInit(EVP_ENCODE_CTX *ctx);
int EVP_DecodeUpdate(EVP_ENCODE_CTX *ctx, unsigned char *out, int *outl,
    const unsigned char *in, int inl);
int EVP_DecodeFinal(EVP_ENCODE_CTX *ctx, unsigned char *out, int *outl);
int EVP_DecodeBlock(unsigned char *t, const unsigned char *f, int n);





EVP_CIPHER_CTX *EVP_CIPHER_CTX_new(void);
int EVP_CIPHER_CTX_reset(EVP_CIPHER_CTX *c);
void EVP_CIPHER_CTX_free(EVP_CIPHER_CTX *c);
int EVP_CIPHER_CTX_set_key_length(EVP_CIPHER_CTX *x, int keylen);
int EVP_CIPHER_CTX_set_padding(EVP_CIPHER_CTX *c, int pad);
int EVP_CIPHER_CTX_ctrl(EVP_CIPHER_CTX *ctx, int type, int arg, void *ptr);
int EVP_CIPHER_CTX_rand_key(EVP_CIPHER_CTX *ctx, unsigned char *key);
int EVP_CIPHER_get_params(EVP_CIPHER *cipher, OSSL_PARAM params[]);
int EVP_CIPHER_CTX_set_params(EVP_CIPHER_CTX *ctx, const OSSL_PARAM params[]);
int EVP_CIPHER_CTX_get_params(EVP_CIPHER_CTX *ctx, OSSL_PARAM params[]);
const OSSL_PARAM *EVP_CIPHER_gettable_params(const EVP_CIPHER *cipher);
const OSSL_PARAM *EVP_CIPHER_settable_ctx_params(const EVP_CIPHER *cipher);
const OSSL_PARAM *EVP_CIPHER_gettable_ctx_params(const EVP_CIPHER *cipher);
const OSSL_PARAM *EVP_CIPHER_CTX_settable_params(EVP_CIPHER_CTX *ctx);
const OSSL_PARAM *EVP_CIPHER_CTX_gettable_params(EVP_CIPHER_CTX *ctx);

int EVP_CIPHER_CTX_set_algor_params(EVP_CIPHER_CTX *ctx, const X509_ALGOR *alg);
int EVP_CIPHER_CTX_get_algor_params(EVP_CIPHER_CTX *ctx, X509_ALGOR *alg);
int EVP_CIPHER_CTX_get_algor(EVP_CIPHER_CTX *ctx, X509_ALGOR **alg);

const BIO_METHOD *BIO_f_md(void);
const BIO_METHOD *BIO_f_base64(void);
const BIO_METHOD *BIO_f_cipher(void);
const BIO_METHOD *BIO_f_reliable(void);
 int BIO_set_cipher(BIO *b, const EVP_CIPHER *c, const unsigned char *k,
    const unsigned char *i, int enc);

const EVP_MD *EVP_md_null(void);




const EVP_MD *EVP_md4(void);


const EVP_MD *EVP_md5(void);
const EVP_MD *EVP_md5_sha1(void);


const EVP_MD *EVP_blake2b512(void);
const EVP_MD *EVP_blake2s256(void);

const EVP_MD *EVP_sha1(void);
const EVP_MD *EVP_sha224(void);
const EVP_MD *EVP_sha256(void);
const EVP_MD *EVP_sha384(void);
const EVP_MD *EVP_sha512(void);
const EVP_MD *EVP_sha512_224(void);
const EVP_MD *EVP_sha512_256(void);
const EVP_MD *EVP_sha3_224(void);
const EVP_MD *EVP_sha3_256(void);
const EVP_MD *EVP_sha3_384(void);
const EVP_MD *EVP_sha3_512(void);
const EVP_MD *EVP_shake128(void);
const EVP_MD *EVP_shake256(void);


const EVP_MD *EVP_mdc2(void);


const EVP_MD *EVP_ripemd160(void);


const EVP_MD *EVP_whirlpool(void);


const EVP_MD *EVP_sm3(void);

const EVP_CIPHER *EVP_enc_null(void);

const EVP_CIPHER *EVP_des_ecb(void);
const EVP_CIPHER *EVP_des_ede(void);
const EVP_CIPHER *EVP_des_ede3(void);
const EVP_CIPHER *EVP_des_ede_ecb(void);
const EVP_CIPHER *EVP_des_ede3_ecb(void);
const EVP_CIPHER *EVP_des_cfb64(void);

const EVP_CIPHER *EVP_des_cfb1(void);
const EVP_CIPHER *EVP_des_cfb8(void);
const EVP_CIPHER *EVP_des_ede_cfb64(void);

const EVP_CIPHER *EVP_des_ede3_cfb64(void);

const EVP_CIPHER *EVP_des_ede3_cfb1(void);
const EVP_CIPHER *EVP_des_ede3_cfb8(void);
const EVP_CIPHER *EVP_des_ofb(void);
const EVP_CIPHER *EVP_des_ede_ofb(void);
const EVP_CIPHER *EVP_des_ede3_ofb(void);
const EVP_CIPHER *EVP_des_cbc(void);
const EVP_CIPHER *EVP_des_ede_cbc(void);
const EVP_CIPHER *EVP_des_ede3_cbc(void);
const EVP_CIPHER *EVP_desx_cbc(void);
const EVP_CIPHER *EVP_des_ede3_wrap(void);







const EVP_CIPHER *EVP_rc4(void);
const EVP_CIPHER *EVP_rc4_40(void);

const EVP_CIPHER *EVP_rc4_hmac_md5(void);



const EVP_CIPHER *EVP_idea_ecb(void);
const EVP_CIPHER *EVP_idea_cfb64(void);

const EVP_CIPHER *EVP_idea_ofb(void);
const EVP_CIPHER *EVP_idea_cbc(void);


const EVP_CIPHER *EVP_rc2_ecb(void);
const EVP_CIPHER *EVP_rc2_cbc(void);
const EVP_CIPHER *EVP_rc2_40_cbc(void);
const EVP_CIPHER *EVP_rc2_64_cbc(void);
const EVP_CIPHER *EVP_rc2_cfb64(void);

const EVP_CIPHER *EVP_rc2_ofb(void);


const EVP_CIPHER *EVP_bf_ecb(void);
const EVP_CIPHER *EVP_bf_cbc(void);
const EVP_CIPHER *EVP_bf_cfb64(void);

const EVP_CIPHER *EVP_bf_ofb(void);


const EVP_CIPHER *EVP_cast5_ecb(void);
const EVP_CIPHER *EVP_cast5_cbc(void);
const EVP_CIPHER *EVP_cast5_cfb64(void);

const EVP_CIPHER *EVP_cast5_ofb(void);
const EVP_CIPHER *EVP_aes_128_ecb(void);
const EVP_CIPHER *EVP_aes_128_cbc(void);
const EVP_CIPHER *EVP_aes_128_cfb1(void);
const EVP_CIPHER *EVP_aes_128_cfb8(void);
const EVP_CIPHER *EVP_aes_128_cfb128(void);

const EVP_CIPHER *EVP_aes_128_ofb(void);
const EVP_CIPHER *EVP_aes_128_ctr(void);
const EVP_CIPHER *EVP_aes_128_ccm(void);
const EVP_CIPHER *EVP_aes_128_gcm(void);
const EVP_CIPHER *EVP_aes_128_xts(void);
const EVP_CIPHER *EVP_aes_128_wrap(void);
const EVP_CIPHER *EVP_aes_128_wrap_pad(void);

const EVP_CIPHER *EVP_aes_128_ocb(void);

const EVP_CIPHER *EVP_aes_192_ecb(void);
const EVP_CIPHER *EVP_aes_192_cbc(void);
const EVP_CIPHER *EVP_aes_192_cfb1(void);
const EVP_CIPHER *EVP_aes_192_cfb8(void);
const EVP_CIPHER *EVP_aes_192_cfb128(void);

const EVP_CIPHER *EVP_aes_192_ofb(void);
const EVP_CIPHER *EVP_aes_192_ctr(void);
const EVP_CIPHER *EVP_aes_192_ccm(void);
const EVP_CIPHER *EVP_aes_192_gcm(void);
const EVP_CIPHER *EVP_aes_192_wrap(void);
const EVP_CIPHER *EVP_aes_192_wrap_pad(void);

const EVP_CIPHER *EVP_aes_192_ocb(void);

const EVP_CIPHER *EVP_aes_256_ecb(void);
const EVP_CIPHER *EVP_aes_256_cbc(void);
const EVP_CIPHER *EVP_aes_256_cfb1(void);
const EVP_CIPHER *EVP_aes_256_cfb8(void);
const EVP_CIPHER *EVP_aes_256_cfb128(void);

const EVP_CIPHER *EVP_aes_256_ofb(void);
const EVP_CIPHER *EVP_aes_256_ctr(void);
const EVP_CIPHER *EVP_aes_256_ccm(void);
const EVP_CIPHER *EVP_aes_256_gcm(void);
const EVP_CIPHER *EVP_aes_256_xts(void);
const EVP_CIPHER *EVP_aes_256_wrap(void);
const EVP_CIPHER *EVP_aes_256_wrap_pad(void);

const EVP_CIPHER *EVP_aes_256_ocb(void);

const EVP_CIPHER *EVP_aes_128_cbc_hmac_sha1(void);
const EVP_CIPHER *EVP_aes_256_cbc_hmac_sha1(void);
const EVP_CIPHER *EVP_aes_128_cbc_hmac_sha256(void);
const EVP_CIPHER *EVP_aes_256_cbc_hmac_sha256(void);

const EVP_CIPHER *EVP_aria_128_ecb(void);
const EVP_CIPHER *EVP_aria_128_cbc(void);
const EVP_CIPHER *EVP_aria_128_cfb1(void);
const EVP_CIPHER *EVP_aria_128_cfb8(void);
const EVP_CIPHER *EVP_aria_128_cfb128(void);

const EVP_CIPHER *EVP_aria_128_ctr(void);
const EVP_CIPHER *EVP_aria_128_ofb(void);
const EVP_CIPHER *EVP_aria_128_gcm(void);
const EVP_CIPHER *EVP_aria_128_ccm(void);
const EVP_CIPHER *EVP_aria_192_ecb(void);
const EVP_CIPHER *EVP_aria_192_cbc(void);
const EVP_CIPHER *EVP_aria_192_cfb1(void);
const EVP_CIPHER *EVP_aria_192_cfb8(void);
const EVP_CIPHER *EVP_aria_192_cfb128(void);

const EVP_CIPHER *EVP_aria_192_ctr(void);
const EVP_CIPHER *EVP_aria_192_ofb(void);
const EVP_CIPHER *EVP_aria_192_gcm(void);
const EVP_CIPHER *EVP_aria_192_ccm(void);
const EVP_CIPHER *EVP_aria_256_ecb(void);
const EVP_CIPHER *EVP_aria_256_cbc(void);
const EVP_CIPHER *EVP_aria_256_cfb1(void);
const EVP_CIPHER *EVP_aria_256_cfb8(void);
const EVP_CIPHER *EVP_aria_256_cfb128(void);

const EVP_CIPHER *EVP_aria_256_ctr(void);
const EVP_CIPHER *EVP_aria_256_ofb(void);
const EVP_CIPHER *EVP_aria_256_gcm(void);
const EVP_CIPHER *EVP_aria_256_ccm(void);


const EVP_CIPHER *EVP_camellia_128_ecb(void);
const EVP_CIPHER *EVP_camellia_128_cbc(void);
const EVP_CIPHER *EVP_camellia_128_cfb1(void);
const EVP_CIPHER *EVP_camellia_128_cfb8(void);
const EVP_CIPHER *EVP_camellia_128_cfb128(void);

const EVP_CIPHER *EVP_camellia_128_ofb(void);
const EVP_CIPHER *EVP_camellia_128_ctr(void);
const EVP_CIPHER *EVP_camellia_192_ecb(void);
const EVP_CIPHER *EVP_camellia_192_cbc(void);
const EVP_CIPHER *EVP_camellia_192_cfb1(void);
const EVP_CIPHER *EVP_camellia_192_cfb8(void);
const EVP_CIPHER *EVP_camellia_192_cfb128(void);

const EVP_CIPHER *EVP_camellia_192_ofb(void);
const EVP_CIPHER *EVP_camellia_192_ctr(void);
const EVP_CIPHER *EVP_camellia_256_ecb(void);
const EVP_CIPHER *EVP_camellia_256_cbc(void);
const EVP_CIPHER *EVP_camellia_256_cfb1(void);
const EVP_CIPHER *EVP_camellia_256_cfb8(void);
const EVP_CIPHER *EVP_camellia_256_cfb128(void);

const EVP_CIPHER *EVP_camellia_256_ofb(void);
const EVP_CIPHER *EVP_camellia_256_ctr(void);


const EVP_CIPHER *EVP_chacha20(void);

const EVP_CIPHER *EVP_chacha20_poly1305(void);




const EVP_CIPHER *EVP_seed_ecb(void);
const EVP_CIPHER *EVP_seed_cbc(void);
const EVP_CIPHER *EVP_seed_cfb128(void);

const EVP_CIPHER *EVP_seed_ofb(void);



const EVP_CIPHER *EVP_sm4_ecb(void);
const EVP_CIPHER *EVP_sm4_cbc(void);
const EVP_CIPHER *EVP_sm4_cfb128(void);

const EVP_CIPHER *EVP_sm4_ofb(void);
const EVP_CIPHER *EVP_sm4_ctr(void);
int EVP_add_cipher(const EVP_CIPHER *cipher);
int EVP_add_digest(const EVP_MD *digest);

const EVP_CIPHER *EVP_get_cipherbyname(const char *name);
const EVP_MD *EVP_get_digestbyname(const char *name);

void EVP_CIPHER_do_all(void (*fn)(const EVP_CIPHER *ciph,
                           const char *from, const char *to, void *x),
    void *arg);
void EVP_CIPHER_do_all_sorted(void (*fn)(const EVP_CIPHER *ciph, const char *from,
                                  const char *to, void *x),
    void *arg);
void EVP_CIPHER_do_all_provided(OSSL_LIB_CTX *libctx,
    void (*fn)(EVP_CIPHER *cipher, void *arg),
    void *arg);

void EVP_MD_do_all(void (*fn)(const EVP_MD *ciph,
                       const char *from, const char *to, void *x),
    void *arg);
void EVP_MD_do_all_sorted(void (*fn)(const EVP_MD *ciph, const char *from,
                              const char *to, void *x),
    void *arg);
void EVP_MD_do_all_provided(OSSL_LIB_CTX *libctx,
    void (*fn)(EVP_MD *md, void *arg),
    void *arg);



EVP_MAC *EVP_MAC_fetch(OSSL_LIB_CTX *libctx, const char *algorithm,
    const char *properties);
int EVP_MAC_up_ref(EVP_MAC *mac);
void EVP_MAC_free(EVP_MAC *mac);
const char *EVP_MAC_get0_name(const EVP_MAC *mac);
const char *EVP_MAC_get0_description(const EVP_MAC *mac);
int EVP_MAC_is_a(const EVP_MAC *mac, const char *name);
const OSSL_PROVIDER *EVP_MAC_get0_provider(const EVP_MAC *mac);
int EVP_MAC_get_params(EVP_MAC *mac, OSSL_PARAM params[]);

EVP_MAC_CTX *EVP_MAC_CTX_new(EVP_MAC *mac);
void EVP_MAC_CTX_free(EVP_MAC_CTX *ctx);
EVP_MAC_CTX *EVP_MAC_CTX_dup(const EVP_MAC_CTX *src);
EVP_MAC *EVP_MAC_CTX_get0_mac(EVP_MAC_CTX *ctx);
int EVP_MAC_CTX_get_params(EVP_MAC_CTX *ctx, OSSL_PARAM params[]);
int EVP_MAC_CTX_set_params(EVP_MAC_CTX *ctx, const OSSL_PARAM params[]);

size_t EVP_MAC_CTX_get_mac_size(EVP_MAC_CTX *ctx);
size_t EVP_MAC_CTX_get_block_size(EVP_MAC_CTX *ctx);
unsigned char *EVP_Q_mac(OSSL_LIB_CTX *libctx, const char *name, const char *propq,
    const char *subalg, const OSSL_PARAM *params,
    const void *key, size_t keylen,
    const unsigned char *data, size_t datalen,
    unsigned char *out, size_t outsize, size_t *outlen);
int EVP_MAC_init(EVP_MAC_CTX *ctx, const unsigned char *key, size_t keylen,
    const OSSL_PARAM params[]);
int EVP_MAC_init_SKEY(EVP_MAC_CTX *ctx, EVP_SKEY *skey, const OSSL_PARAM params[]);
int EVP_MAC_update(EVP_MAC_CTX *ctx, const unsigned char *data, size_t datalen);
int EVP_MAC_final(EVP_MAC_CTX *ctx,
    unsigned char *out, size_t *outl, size_t outsize);
int EVP_MAC_finalXOF(EVP_MAC_CTX *ctx, unsigned char *out, size_t outsize);
const OSSL_PARAM *EVP_MAC_gettable_params(const EVP_MAC *mac);
const OSSL_PARAM *EVP_MAC_gettable_ctx_params(const EVP_MAC *mac);
const OSSL_PARAM *EVP_MAC_settable_ctx_params(const EVP_MAC *mac);
const OSSL_PARAM *EVP_MAC_CTX_gettable_params(EVP_MAC_CTX *ctx);
const OSSL_PARAM *EVP_MAC_CTX_settable_params(EVP_MAC_CTX *ctx);

void EVP_MAC_do_all_provided(OSSL_LIB_CTX *libctx,
    void (*fn)(EVP_MAC *mac, void *arg),
    void *arg);
int EVP_MAC_names_do_all(const EVP_MAC *mac,
    void (*fn)(const char *name, void *data),
    void *data);


EVP_RAND *EVP_RAND_fetch(OSSL_LIB_CTX *libctx, const char *algorithm,
    const char *properties);
int EVP_RAND_up_ref(EVP_RAND *rand);
void EVP_RAND_free(EVP_RAND *rand);
const char *EVP_RAND_get0_name(const EVP_RAND *rand);
const char *EVP_RAND_get0_description(const EVP_RAND *md);
int EVP_RAND_is_a(const EVP_RAND *rand, const char *name);
const OSSL_PROVIDER *EVP_RAND_get0_provider(const EVP_RAND *rand);
int EVP_RAND_get_params(EVP_RAND *rand, OSSL_PARAM params[]);

EVP_RAND_CTX *EVP_RAND_CTX_new(EVP_RAND *rand, EVP_RAND_CTX *parent);
int EVP_RAND_CTX_up_ref(EVP_RAND_CTX *ctx);
void EVP_RAND_CTX_free(EVP_RAND_CTX *ctx);
EVP_RAND *EVP_RAND_CTX_get0_rand(EVP_RAND_CTX *ctx);
int EVP_RAND_CTX_get_params(EVP_RAND_CTX *ctx, OSSL_PARAM params[]);
int EVP_RAND_CTX_set_params(EVP_RAND_CTX *ctx, const OSSL_PARAM params[]);
const OSSL_PARAM *EVP_RAND_gettable_params(const EVP_RAND *rand);
const OSSL_PARAM *EVP_RAND_gettable_ctx_params(const EVP_RAND *rand);
const OSSL_PARAM *EVP_RAND_settable_ctx_params(const EVP_RAND *rand);
const OSSL_PARAM *EVP_RAND_CTX_gettable_params(EVP_RAND_CTX *ctx);
const OSSL_PARAM *EVP_RAND_CTX_settable_params(EVP_RAND_CTX *ctx);

void EVP_RAND_do_all_provided(OSSL_LIB_CTX *libctx,
    void (*fn)(EVP_RAND *rand, void *arg),
    void *arg);
int EVP_RAND_names_do_all(const EVP_RAND *rand,
    void (*fn)(const char *name, void *data),
    void *data);

 int EVP_RAND_instantiate(EVP_RAND_CTX *ctx, unsigned int strength,
    int prediction_resistance,
    const unsigned char *pstr, size_t pstr_len,
    const OSSL_PARAM params[]);
int EVP_RAND_uninstantiate(EVP_RAND_CTX *ctx);
 int EVP_RAND_generate(EVP_RAND_CTX *ctx, unsigned char *out,
    size_t outlen, unsigned int strength,
    int prediction_resistance,
    const unsigned char *addin, size_t addin_len);
int EVP_RAND_reseed(EVP_RAND_CTX *ctx, int prediction_resistance,
    const unsigned char *ent, size_t ent_len,
    const unsigned char *addin, size_t addin_len);
 int EVP_RAND_nonce(EVP_RAND_CTX *ctx, unsigned char *out, size_t outlen);
 int EVP_RAND_enable_locking(EVP_RAND_CTX *ctx);

int EVP_RAND_verify_zeroization(EVP_RAND_CTX *ctx);
unsigned int EVP_RAND_get_strength(EVP_RAND_CTX *ctx);
int EVP_RAND_get_state(EVP_RAND_CTX *ctx);







__attribute__((deprecated("Since OpenSSL " "3.0"))) int EVP_PKEY_decrypt_old(unsigned char *dec_key,
    const unsigned char *enc_key,
    int enc_key_len,
    EVP_PKEY *private_key);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int EVP_PKEY_encrypt_old(unsigned char *enc_key,
    const unsigned char *key,
    int key_len, EVP_PKEY *pub_key);

int EVP_PKEY_is_a(const EVP_PKEY *pkey, const char *name);
int EVP_PKEY_type_names_do_all(const EVP_PKEY *pkey,
    void (*fn)(const char *name, void *data),
    void *data);
int EVP_PKEY_type(int type);
int EVP_PKEY_get_id(const EVP_PKEY *pkey);

int EVP_PKEY_get_base_id(const EVP_PKEY *pkey);

int EVP_PKEY_get_bits(const EVP_PKEY *pkey);

int EVP_PKEY_get_security_bits(const EVP_PKEY *pkey);

int EVP_PKEY_get_security_category(const EVP_PKEY *pkey);
int EVP_PKEY_get_size(const EVP_PKEY *pkey);

int EVP_PKEY_can_sign(const EVP_PKEY *pkey);
int EVP_PKEY_set_type(EVP_PKEY *pkey, int type);
int EVP_PKEY_set_type_str(EVP_PKEY *pkey, const char *str, int len);
int EVP_PKEY_set_type_by_keymgmt(EVP_PKEY *pkey, EVP_KEYMGMT *keymgmt);


__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_PKEY_set1_engine(EVP_PKEY *pkey, ENGINE *e);
__attribute__((deprecated("Since OpenSSL " "3.0")))
ENGINE *EVP_PKEY_get0_engine(const EVP_PKEY *pkey);

__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_PKEY_assign(EVP_PKEY *pkey, int type, void *key);
__attribute__((deprecated("Since OpenSSL " "3.0")))
void *EVP_PKEY_get0(const EVP_PKEY *pkey);
__attribute__((deprecated("Since OpenSSL " "3.0")))
const unsigned char *EVP_PKEY_get0_hmac(const EVP_PKEY *pkey, size_t *len);

__attribute__((deprecated("Since OpenSSL " "3.0")))
const unsigned char *EVP_PKEY_get0_poly1305(const EVP_PKEY *pkey, size_t *len);


__attribute__((deprecated("Since OpenSSL " "3.0")))
const unsigned char *EVP_PKEY_get0_siphash(const EVP_PKEY *pkey, size_t *len);


struct rsa_st;
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_PKEY_set1_RSA(EVP_PKEY *pkey, struct rsa_st *key);
__attribute__((deprecated("Since OpenSSL " "3.0")))
const struct rsa_st *EVP_PKEY_get0_RSA(const EVP_PKEY *pkey);
__attribute__((deprecated("Since OpenSSL " "3.0")))
struct rsa_st *EVP_PKEY_get1_RSA(EVP_PKEY *pkey);


struct dsa_st;
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_PKEY_set1_DSA(EVP_PKEY *pkey, struct dsa_st *key);
__attribute__((deprecated("Since OpenSSL " "3.0")))
const struct dsa_st *EVP_PKEY_get0_DSA(const EVP_PKEY *pkey);
__attribute__((deprecated("Since OpenSSL " "3.0")))
struct dsa_st *EVP_PKEY_get1_DSA(EVP_PKEY *pkey);



struct dh_st;
__attribute__((deprecated("Since OpenSSL " "3.0"))) int EVP_PKEY_set1_DH(EVP_PKEY *pkey, struct dh_st *key);
__attribute__((deprecated("Since OpenSSL " "3.0"))) const struct dh_st *EVP_PKEY_get0_DH(const EVP_PKEY *pkey);
__attribute__((deprecated("Since OpenSSL " "3.0"))) struct dh_st *EVP_PKEY_get1_DH(EVP_PKEY *pkey);



struct ec_key_st;
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_PKEY_set1_EC_KEY(EVP_PKEY *pkey, struct ec_key_st *key);
__attribute__((deprecated("Since OpenSSL " "3.0")))
const struct ec_key_st *EVP_PKEY_get0_EC_KEY(const EVP_PKEY *pkey);
__attribute__((deprecated("Since OpenSSL " "3.0")))
struct ec_key_st *EVP_PKEY_get1_EC_KEY(EVP_PKEY *pkey);



EVP_PKEY *EVP_PKEY_new(void);
int EVP_PKEY_up_ref(EVP_PKEY *pkey);
EVP_PKEY *EVP_PKEY_dup(EVP_PKEY *pkey);
void EVP_PKEY_free(EVP_PKEY *pkey);
const char *EVP_PKEY_get0_description(const EVP_PKEY *pkey);
const OSSL_PROVIDER *EVP_PKEY_get0_provider(const EVP_PKEY *key);

EVP_PKEY *d2i_PublicKey(int type, EVP_PKEY **a, const unsigned char **pp,
    long length);
int i2d_PublicKey(const EVP_PKEY *a, unsigned char **pp);

EVP_PKEY *d2i_PrivateKey_ex(int type, EVP_PKEY **a, const unsigned char **pp,
    long length, OSSL_LIB_CTX *libctx,
    const char *propq);
EVP_PKEY *d2i_PrivateKey(int type, EVP_PKEY **a, const unsigned char **pp,
    long length);
EVP_PKEY *d2i_AutoPrivateKey_ex(EVP_PKEY **a, const unsigned char **pp,
    long length, OSSL_LIB_CTX *libctx,
    const char *propq);
EVP_PKEY *d2i_AutoPrivateKey(EVP_PKEY **a, const unsigned char **pp,
    long length);
int i2d_PrivateKey(const EVP_PKEY *a, unsigned char **pp);
int i2d_PKCS8PrivateKey(const EVP_PKEY *a, unsigned char **pp);

int i2d_KeyParams(const EVP_PKEY *a, unsigned char **pp);
EVP_PKEY *d2i_KeyParams(int type, EVP_PKEY **a, const unsigned char **pp,
    long length);
int i2d_KeyParams_bio(BIO *bp, const EVP_PKEY *pkey);
EVP_PKEY *d2i_KeyParams_bio(int type, EVP_PKEY **a, BIO *in);

int EVP_PKEY_copy_parameters(EVP_PKEY *to, const EVP_PKEY *from);
int EVP_PKEY_missing_parameters(const EVP_PKEY *pkey);
int EVP_PKEY_save_parameters(EVP_PKEY *pkey, int mode);
int EVP_PKEY_parameters_eq(const EVP_PKEY *a, const EVP_PKEY *b);
int EVP_PKEY_eq(const EVP_PKEY *a, const EVP_PKEY *b);


__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_PKEY_cmp_parameters(const EVP_PKEY *a, const EVP_PKEY *b);
__attribute__((deprecated("Since OpenSSL " "3.0")))
int EVP_PKEY_cmp(const EVP_PKEY *a, const EVP_PKEY *b);


int EVP_PKEY_print_public(BIO *out, const EVP_PKEY *pkey,
    int indent, ASN1_PCTX *pctx);
int EVP_PKEY_print_private(BIO *out, const EVP_PKEY *pkey,
    int indent, ASN1_PCTX *pctx);
int EVP_PKEY_print_params(BIO *out, const EVP_PKEY *pkey,
    int indent, ASN1_PCTX *pctx);

int EVP_PKEY_print_public_fp(FILE *fp, const EVP_PKEY *pkey,
    int indent, ASN1_PCTX *pctx);
int EVP_PKEY_print_private_fp(FILE *fp, const EVP_PKEY *pkey,
    int indent, ASN1_PCTX *pctx);
int EVP_PKEY_print_params_fp(FILE *fp, const EVP_PKEY *pkey,
    int indent, ASN1_PCTX *pctx);


int EVP_PKEY_get_default_digest_nid(EVP_PKEY *pkey, int *pnid);
int EVP_PKEY_get_default_digest_name(EVP_PKEY *pkey,
    char *mdname, size_t mdname_sz);
int EVP_PKEY_digestsign_supports_digest(EVP_PKEY *pkey, OSSL_LIB_CTX *libctx,
    const char *name, const char *propq);
int EVP_PKEY_set1_encoded_public_key(EVP_PKEY *pkey,
    const unsigned char *pub, size_t publen);
size_t EVP_PKEY_get1_encoded_public_key(EVP_PKEY *pkey, unsigned char **ppub);


int EVP_CIPHER_param_to_asn1(EVP_CIPHER_CTX *c, ASN1_TYPE *type);
int EVP_CIPHER_asn1_to_param(EVP_CIPHER_CTX *c, ASN1_TYPE *type);


int EVP_CIPHER_set_asn1_iv(EVP_CIPHER_CTX *c, ASN1_TYPE *type);
int EVP_CIPHER_get_asn1_iv(EVP_CIPHER_CTX *c, ASN1_TYPE *type);


int PKCS5_PBE_keyivgen(EVP_CIPHER_CTX *ctx, const char *pass, int passlen,
    ASN1_TYPE *param, const EVP_CIPHER *cipher,
    const EVP_MD *md, int en_de);
int PKCS5_PBE_keyivgen_ex(EVP_CIPHER_CTX *cctx, const char *pass, int passlen,
    ASN1_TYPE *param, const EVP_CIPHER *cipher,
    const EVP_MD *md, int en_de, OSSL_LIB_CTX *libctx,
    const char *propq);
int PKCS5_PBKDF2_HMAC_SHA1(const char *pass, int passlen,
    const unsigned char *salt, int saltlen, int iter,
    int keylen, unsigned char *out);
int PKCS5_PBKDF2_HMAC(const char *pass, int passlen,
    const unsigned char *salt, int saltlen, int iter,
    const EVP_MD *digest, int keylen, unsigned char *out);
int PKCS5_v2_PBE_keyivgen(EVP_CIPHER_CTX *ctx, const char *pass, int passlen,
    ASN1_TYPE *param, const EVP_CIPHER *cipher,
    const EVP_MD *md, int en_de);
int PKCS5_v2_PBE_keyivgen_ex(EVP_CIPHER_CTX *ctx, const char *pass, int passlen,
    ASN1_TYPE *param, const EVP_CIPHER *cipher,
    const EVP_MD *md, int en_de,
    OSSL_LIB_CTX *libctx, const char *propq);


int EVP_PBE_scrypt(const char *pass, size_t passlen,
    const unsigned char *salt, size_t saltlen,
    uint64_t N, uint64_t r, uint64_t p, uint64_t maxmem,
    unsigned char *key, size_t keylen);
int EVP_PBE_scrypt_ex(const char *pass, size_t passlen,
    const unsigned char *salt, size_t saltlen,
    uint64_t N, uint64_t r, uint64_t p, uint64_t maxmem,
    unsigned char *key, size_t keylen,
    OSSL_LIB_CTX *ctx, const char *propq);

int PKCS5_v2_scrypt_keyivgen(EVP_CIPHER_CTX *ctx, const char *pass,
    int passlen, ASN1_TYPE *param,
    const EVP_CIPHER *c, const EVP_MD *md, int en_de);
int PKCS5_v2_scrypt_keyivgen_ex(EVP_CIPHER_CTX *ctx, const char *pass,
    int passlen, ASN1_TYPE *param,
    const EVP_CIPHER *c, const EVP_MD *md, int en_de,
    OSSL_LIB_CTX *libctx, const char *propq);


void PKCS5_PBE_add(void);

int EVP_PBE_CipherInit(ASN1_OBJECT *pbe_obj, const char *pass, int passlen,
    ASN1_TYPE *param, EVP_CIPHER_CTX *ctx, int en_de);

int EVP_PBE_CipherInit_ex(ASN1_OBJECT *pbe_obj, const char *pass, int passlen,
    ASN1_TYPE *param, EVP_CIPHER_CTX *ctx, int en_de,
    OSSL_LIB_CTX *libctx, const char *propq);
int EVP_PBE_alg_add_type(int pbe_type, int pbe_nid, int cipher_nid,
    int md_nid, EVP_PBE_KEYGEN *keygen);
int EVP_PBE_alg_add(int nid, const EVP_CIPHER *cipher, const EVP_MD *md,
    EVP_PBE_KEYGEN *keygen);
int EVP_PBE_find(int type, int pbe_nid, int *pcnid, int *pmnid,
    EVP_PBE_KEYGEN **pkeygen);
int EVP_PBE_find_ex(int type, int pbe_nid, int *pcnid, int *pmnid,
    EVP_PBE_KEYGEN **pkeygen, EVP_PBE_KEYGEN_EX **pkeygen_ex);
void EVP_PBE_cleanup(void);
int EVP_PBE_get(int *ptype, int *ppbe_nid, size_t num);
__attribute__((deprecated("Since OpenSSL " "3.6"))) int EVP_PKEY_asn1_get_count(void);
__attribute__((deprecated("Since OpenSSL " "3.6"))) const EVP_PKEY_ASN1_METHOD *EVP_PKEY_asn1_get0(int idx);
__attribute__((deprecated("Since OpenSSL " "3.6")))
const EVP_PKEY_ASN1_METHOD *EVP_PKEY_asn1_find(ENGINE **pe, int type);
__attribute__((deprecated("Since OpenSSL " "3.6")))
const EVP_PKEY_ASN1_METHOD *EVP_PKEY_asn1_find_str(ENGINE **pe,
    const char *str, int len);
__attribute__((deprecated("Since OpenSSL " "3.6"))) int EVP_PKEY_asn1_add0(const EVP_PKEY_ASN1_METHOD *ameth);
__attribute__((deprecated("Since OpenSSL " "3.6"))) int EVP_PKEY_asn1_add_alias(int to, int from);
__attribute__((deprecated("Since OpenSSL " "3.6")))
int EVP_PKEY_asn1_get0_info(int *ppkey_id, int *pkey_base_id,
    int *ppkey_flags, const char **pinfo,
    const char **ppem_str,
    const EVP_PKEY_ASN1_METHOD *ameth);

__attribute__((deprecated("Since OpenSSL " "3.6"))) const EVP_PKEY_ASN1_METHOD *EVP_PKEY_get0_asn1(const EVP_PKEY *pkey);
__attribute__((deprecated("Since OpenSSL " "3.6"))) EVP_PKEY_ASN1_METHOD *EVP_PKEY_asn1_new(int id, int flags,
    const char *pem_str,
    const char *info);
__attribute__((deprecated("Since OpenSSL " "3.6"))) void EVP_PKEY_asn1_copy(EVP_PKEY_ASN1_METHOD *dst,
    const EVP_PKEY_ASN1_METHOD *src);
__attribute__((deprecated("Since OpenSSL " "3.6"))) void EVP_PKEY_asn1_free(EVP_PKEY_ASN1_METHOD *ameth);
__attribute__((deprecated("Since OpenSSL " "3.6")))
void EVP_PKEY_asn1_set_public(EVP_PKEY_ASN1_METHOD *ameth,
    int (*pub_decode)(EVP_PKEY *pk,
        const X509_PUBKEY *pub),
    int (*pub_encode)(X509_PUBKEY *pub,
        const EVP_PKEY *pk),
    int (*pub_cmp)(const EVP_PKEY *a,
        const EVP_PKEY *b),
    int (*pub_print)(BIO *out,
        const EVP_PKEY *pkey,
        int indent, ASN1_PCTX *pctx),
    int (*pkey_size)(const EVP_PKEY *pk),
    int (*pkey_bits)(const EVP_PKEY *pk));
__attribute__((deprecated("Since OpenSSL " "3.6")))
void EVP_PKEY_asn1_set_private(EVP_PKEY_ASN1_METHOD *ameth,
    int (*priv_decode)(EVP_PKEY *pk,
        const PKCS8_PRIV_KEY_INFO
            *p8inf),
    int (*priv_encode)(PKCS8_PRIV_KEY_INFO *p8,
        const EVP_PKEY *pk),
    int (*priv_print)(BIO *out,
        const EVP_PKEY *pkey,
        int indent,
        ASN1_PCTX *pctx));
__attribute__((deprecated("Since OpenSSL " "3.6")))
void EVP_PKEY_asn1_set_param(EVP_PKEY_ASN1_METHOD *ameth,
    int (*param_decode)(EVP_PKEY *pkey,
        const unsigned char **pder,
        int derlen),
    int (*param_encode)(const EVP_PKEY *pkey,
        unsigned char **pder),
    int (*param_missing)(const EVP_PKEY *pk),
    int (*param_copy)(EVP_PKEY *to,
        const EVP_PKEY *from),
    int (*param_cmp)(const EVP_PKEY *a,
        const EVP_PKEY *b),
    int (*param_print)(BIO *out,
        const EVP_PKEY *pkey,
        int indent,
        ASN1_PCTX *pctx));

__attribute__((deprecated("Since OpenSSL " "3.6")))
void EVP_PKEY_asn1_set_free(EVP_PKEY_ASN1_METHOD *ameth,
    void (*pkey_free)(EVP_PKEY *pkey));
__attribute__((deprecated("Since OpenSSL " "3.6")))
void EVP_PKEY_asn1_set_ctrl(EVP_PKEY_ASN1_METHOD *ameth,
    int (*pkey_ctrl)(EVP_PKEY *pkey, int op,
        long arg1, void *arg2));
__attribute__((deprecated("Since OpenSSL " "3.6")))
void EVP_PKEY_asn1_set_item(EVP_PKEY_ASN1_METHOD *ameth,
    int (*item_verify)(EVP_MD_CTX *ctx,
        const ASN1_ITEM *it,
        const void *data,
        const X509_ALGOR *a,
        const ASN1_BIT_STRING *sig,
        EVP_PKEY *pkey),
    int (*item_sign)(EVP_MD_CTX *ctx,
        const ASN1_ITEM *it,
        const void *data,
        X509_ALGOR *alg1,
        X509_ALGOR *alg2,
        ASN1_BIT_STRING *sig));

__attribute__((deprecated("Since OpenSSL " "3.6")))
void EVP_PKEY_asn1_set_siginf(EVP_PKEY_ASN1_METHOD *ameth,
    int (*siginf_set)(X509_SIG_INFO *siginf,
        const X509_ALGOR *alg,
        const ASN1_STRING *sig));

__attribute__((deprecated("Since OpenSSL " "3.6")))
void EVP_PKEY_asn1_set_check(EVP_PKEY_ASN1_METHOD *ameth,
    int (*pkey_check)(const EVP_PKEY *pk));

__attribute__((deprecated("Since OpenSSL " "3.6")))
void EVP_PKEY_asn1_set_public_check(EVP_PKEY_ASN1_METHOD *ameth,
    int (*pkey_pub_check)(const EVP_PKEY *pk));

__attribute__((deprecated("Since OpenSSL " "3.6")))
void EVP_PKEY_asn1_set_param_check(EVP_PKEY_ASN1_METHOD *ameth,
    int (*pkey_param_check)(const EVP_PKEY *pk));

__attribute__((deprecated("Since OpenSSL " "3.6")))
void EVP_PKEY_asn1_set_set_priv_key(EVP_PKEY_ASN1_METHOD *ameth,
    int (*set_priv_key)(EVP_PKEY *pk,
        const unsigned char
            *priv,
        size_t len));
__attribute__((deprecated("Since OpenSSL " "3.6")))
void EVP_PKEY_asn1_set_set_pub_key(EVP_PKEY_ASN1_METHOD *ameth,
    int (*set_pub_key)(EVP_PKEY *pk,
        const unsigned char *pub,
        size_t len));
__attribute__((deprecated("Since OpenSSL " "3.6")))
void EVP_PKEY_asn1_set_get_priv_key(EVP_PKEY_ASN1_METHOD *ameth,
    int (*get_priv_key)(const EVP_PKEY *pk,
        unsigned char *priv,
        size_t *len));
__attribute__((deprecated("Since OpenSSL " "3.6")))
void EVP_PKEY_asn1_set_get_pub_key(EVP_PKEY_ASN1_METHOD *ameth,
    int (*get_pub_key)(const EVP_PKEY *pk,
        unsigned char *pub,
        size_t *len));

__attribute__((deprecated("Since OpenSSL " "3.6")))
void EVP_PKEY_asn1_set_security_bits(EVP_PKEY_ASN1_METHOD *ameth,
    int (*pkey_security_bits)(const EVP_PKEY
            *pk));


int EVP_PKEY_CTX_get_signature_md(EVP_PKEY_CTX *ctx, const EVP_MD **md);
int EVP_PKEY_CTX_set_signature_md(EVP_PKEY_CTX *ctx, const EVP_MD *md);

int EVP_PKEY_CTX_set1_id(EVP_PKEY_CTX *ctx, const void *id, int len);
int EVP_PKEY_CTX_get1_id(EVP_PKEY_CTX *ctx, void *id);
int EVP_PKEY_CTX_get1_id_len(EVP_PKEY_CTX *ctx, size_t *id_len);

int EVP_PKEY_CTX_set_kem_op(EVP_PKEY_CTX *ctx, const char *op);

const char *EVP_PKEY_get0_type_name(const EVP_PKEY *key);
int EVP_PKEY_CTX_set_mac_key(EVP_PKEY_CTX *ctx, const unsigned char *key,
    int keylen);
__attribute__((deprecated("Since OpenSSL " "3.0"))) const EVP_PKEY_METHOD *EVP_PKEY_meth_find(int type);
__attribute__((deprecated("Since OpenSSL " "3.0"))) EVP_PKEY_METHOD *EVP_PKEY_meth_new(int id, int flags);
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get0_info(int *ppkey_id, int *pflags,
    const EVP_PKEY_METHOD *meth);
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_copy(EVP_PKEY_METHOD *dst,
    const EVP_PKEY_METHOD *src);
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_free(EVP_PKEY_METHOD *pmeth);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int EVP_PKEY_meth_add0(const EVP_PKEY_METHOD *pmeth);
__attribute__((deprecated("Since OpenSSL " "3.0"))) int EVP_PKEY_meth_remove(const EVP_PKEY_METHOD *pmeth);
__attribute__((deprecated("Since OpenSSL " "3.0"))) size_t EVP_PKEY_meth_get_count(void);
__attribute__((deprecated("Since OpenSSL " "3.0"))) const EVP_PKEY_METHOD *EVP_PKEY_meth_get0(size_t idx);


EVP_KEYMGMT *EVP_KEYMGMT_fetch(OSSL_LIB_CTX *ctx, const char *algorithm,
    const char *properties);
int EVP_KEYMGMT_up_ref(EVP_KEYMGMT *keymgmt);
void EVP_KEYMGMT_free(EVP_KEYMGMT *keymgmt);
const OSSL_PROVIDER *EVP_KEYMGMT_get0_provider(const EVP_KEYMGMT *keymgmt);
const char *EVP_KEYMGMT_get0_name(const EVP_KEYMGMT *keymgmt);
const char *EVP_KEYMGMT_get0_description(const EVP_KEYMGMT *keymgmt);
int EVP_KEYMGMT_is_a(const EVP_KEYMGMT *keymgmt, const char *name);
void EVP_KEYMGMT_do_all_provided(OSSL_LIB_CTX *libctx,
    void (*fn)(EVP_KEYMGMT *keymgmt, void *arg),
    void *arg);
int EVP_KEYMGMT_names_do_all(const EVP_KEYMGMT *keymgmt,
    void (*fn)(const char *name, void *data),
    void *data);
const OSSL_PARAM *EVP_KEYMGMT_gettable_params(const EVP_KEYMGMT *keymgmt);
const OSSL_PARAM *EVP_KEYMGMT_settable_params(const EVP_KEYMGMT *keymgmt);
const OSSL_PARAM *EVP_KEYMGMT_gen_settable_params(const EVP_KEYMGMT *keymgmt);
const OSSL_PARAM *EVP_KEYMGMT_gen_gettable_params(const EVP_KEYMGMT *keymgmt);

EVP_SKEYMGMT *EVP_SKEYMGMT_fetch(OSSL_LIB_CTX *ctx, const char *algorithm,
    const char *properties);
int EVP_SKEYMGMT_up_ref(EVP_SKEYMGMT *keymgmt);
void EVP_SKEYMGMT_free(EVP_SKEYMGMT *keymgmt);
const OSSL_PROVIDER *EVP_SKEYMGMT_get0_provider(const EVP_SKEYMGMT *keymgmt);
const char *EVP_SKEYMGMT_get0_name(const EVP_SKEYMGMT *keymgmt);
const char *EVP_SKEYMGMT_get0_description(const EVP_SKEYMGMT *keymgmt);
int EVP_SKEYMGMT_is_a(const EVP_SKEYMGMT *keymgmt, const char *name);
void EVP_SKEYMGMT_do_all_provided(OSSL_LIB_CTX *libctx,
    void (*fn)(EVP_SKEYMGMT *keymgmt, void *arg),
    void *arg);
int EVP_SKEYMGMT_names_do_all(const EVP_SKEYMGMT *keymgmt,
    void (*fn)(const char *name, void *data),
    void *data);
const OSSL_PARAM *EVP_SKEYMGMT_get0_gen_settable_params(const EVP_SKEYMGMT *skeymgmt);
const OSSL_PARAM *EVP_SKEYMGMT_get0_imp_settable_params(const EVP_SKEYMGMT *skeymgmt);

EVP_PKEY_CTX *EVP_PKEY_CTX_new(EVP_PKEY *pkey, ENGINE *e);
EVP_PKEY_CTX *EVP_PKEY_CTX_new_id(int id, ENGINE *e);
EVP_PKEY_CTX *EVP_PKEY_CTX_new_from_name(OSSL_LIB_CTX *libctx,
    const char *name,
    const char *propquery);
EVP_PKEY_CTX *EVP_PKEY_CTX_new_from_pkey(OSSL_LIB_CTX *libctx,
    EVP_PKEY *pkey, const char *propquery);
EVP_PKEY_CTX *EVP_PKEY_CTX_dup(const EVP_PKEY_CTX *ctx);
void EVP_PKEY_CTX_free(EVP_PKEY_CTX *ctx);
int EVP_PKEY_CTX_is_a(EVP_PKEY_CTX *ctx, const char *keytype);

int EVP_PKEY_CTX_get_params(EVP_PKEY_CTX *ctx, OSSL_PARAM *params);
const OSSL_PARAM *EVP_PKEY_CTX_gettable_params(const EVP_PKEY_CTX *ctx);
int EVP_PKEY_CTX_set_params(EVP_PKEY_CTX *ctx, const OSSL_PARAM *params);
const OSSL_PARAM *EVP_PKEY_CTX_settable_params(const EVP_PKEY_CTX *ctx);

int EVP_PKEY_CTX_set_algor_params(EVP_PKEY_CTX *ctx, const X509_ALGOR *alg);
int EVP_PKEY_CTX_get_algor_params(EVP_PKEY_CTX *ctx, X509_ALGOR *alg);
int EVP_PKEY_CTX_get_algor(EVP_PKEY_CTX *ctx, X509_ALGOR **alg);

int EVP_PKEY_CTX_ctrl(EVP_PKEY_CTX *ctx, int keytype, int optype,
    int cmd, int p1, void *p2);
int EVP_PKEY_CTX_ctrl_str(EVP_PKEY_CTX *ctx, const char *type,
    const char *value);
int EVP_PKEY_CTX_ctrl_uint64(EVP_PKEY_CTX *ctx, int keytype, int optype,
    int cmd, uint64_t value);

int EVP_PKEY_CTX_str2ctrl(EVP_PKEY_CTX *ctx, int cmd, const char *str);
int EVP_PKEY_CTX_hex2ctrl(EVP_PKEY_CTX *ctx, int cmd, const char *hex);

int EVP_PKEY_CTX_md(EVP_PKEY_CTX *ctx, int optype, int cmd, const char *md);

int EVP_PKEY_CTX_get_operation(EVP_PKEY_CTX *ctx);
void EVP_PKEY_CTX_set0_keygen_info(EVP_PKEY_CTX *ctx, int *dat, int datlen);

EVP_PKEY *EVP_PKEY_new_mac_key(int type, ENGINE *e,
    const unsigned char *key, int keylen);
EVP_PKEY *EVP_PKEY_new_raw_private_key_ex(OSSL_LIB_CTX *libctx,
    const char *keytype,
    const char *propq,
    const unsigned char *priv, size_t len);
EVP_PKEY *EVP_PKEY_new_raw_private_key(int type, ENGINE *e,
    const unsigned char *priv,
    size_t len);
EVP_PKEY *EVP_PKEY_new_raw_public_key_ex(OSSL_LIB_CTX *libctx,
    const char *keytype, const char *propq,
    const unsigned char *pub, size_t len);
EVP_PKEY *EVP_PKEY_new_raw_public_key(int type, ENGINE *e,
    const unsigned char *pub,
    size_t len);
int EVP_PKEY_get_raw_private_key(const EVP_PKEY *pkey, unsigned char *priv,
    size_t *len);
int EVP_PKEY_get_raw_public_key(const EVP_PKEY *pkey, unsigned char *pub,
    size_t *len);


__attribute__((deprecated("Since OpenSSL " "3.0")))
EVP_PKEY *EVP_PKEY_new_CMAC_key(ENGINE *e, const unsigned char *priv,
    size_t len, const EVP_CIPHER *cipher);


void EVP_PKEY_CTX_set_data(EVP_PKEY_CTX *ctx, void *data);
void *EVP_PKEY_CTX_get_data(const EVP_PKEY_CTX *ctx);
EVP_PKEY *EVP_PKEY_CTX_get0_pkey(EVP_PKEY_CTX *ctx);

EVP_PKEY *EVP_PKEY_CTX_get0_peerkey(EVP_PKEY_CTX *ctx);

void EVP_PKEY_CTX_set_app_data(EVP_PKEY_CTX *ctx, void *data);
void *EVP_PKEY_CTX_get_app_data(EVP_PKEY_CTX *ctx);

int EVP_PKEY_CTX_set_signature(EVP_PKEY_CTX *pctx,
    const unsigned char *sig, size_t siglen);

void EVP_SIGNATURE_free(EVP_SIGNATURE *signature);
int EVP_SIGNATURE_up_ref(EVP_SIGNATURE *signature);
OSSL_PROVIDER *EVP_SIGNATURE_get0_provider(const EVP_SIGNATURE *signature);
EVP_SIGNATURE *EVP_SIGNATURE_fetch(OSSL_LIB_CTX *ctx, const char *algorithm,
    const char *properties);
int EVP_SIGNATURE_is_a(const EVP_SIGNATURE *signature, const char *name);
const char *EVP_SIGNATURE_get0_name(const EVP_SIGNATURE *signature);
const char *EVP_SIGNATURE_get0_description(const EVP_SIGNATURE *signature);
void EVP_SIGNATURE_do_all_provided(OSSL_LIB_CTX *libctx,
    void (*fn)(EVP_SIGNATURE *signature,
        void *data),
    void *data);
int EVP_SIGNATURE_names_do_all(const EVP_SIGNATURE *signature,
    void (*fn)(const char *name, void *data),
    void *data);
const OSSL_PARAM *EVP_SIGNATURE_gettable_ctx_params(const EVP_SIGNATURE *sig);
const OSSL_PARAM *EVP_SIGNATURE_settable_ctx_params(const EVP_SIGNATURE *sig);

void EVP_ASYM_CIPHER_free(EVP_ASYM_CIPHER *cipher);
int EVP_ASYM_CIPHER_up_ref(EVP_ASYM_CIPHER *cipher);
OSSL_PROVIDER *EVP_ASYM_CIPHER_get0_provider(const EVP_ASYM_CIPHER *cipher);
EVP_ASYM_CIPHER *EVP_ASYM_CIPHER_fetch(OSSL_LIB_CTX *ctx, const char *algorithm,
    const char *properties);
int EVP_ASYM_CIPHER_is_a(const EVP_ASYM_CIPHER *cipher, const char *name);
const char *EVP_ASYM_CIPHER_get0_name(const EVP_ASYM_CIPHER *cipher);
const char *EVP_ASYM_CIPHER_get0_description(const EVP_ASYM_CIPHER *cipher);
void EVP_ASYM_CIPHER_do_all_provided(OSSL_LIB_CTX *libctx,
    void (*fn)(EVP_ASYM_CIPHER *cipher,
        void *arg),
    void *arg);
int EVP_ASYM_CIPHER_names_do_all(const EVP_ASYM_CIPHER *cipher,
    void (*fn)(const char *name, void *data),
    void *data);
const OSSL_PARAM *EVP_ASYM_CIPHER_gettable_ctx_params(const EVP_ASYM_CIPHER *ciph);
const OSSL_PARAM *EVP_ASYM_CIPHER_settable_ctx_params(const EVP_ASYM_CIPHER *ciph);

void EVP_KEM_free(EVP_KEM *wrap);
int EVP_KEM_up_ref(EVP_KEM *wrap);
OSSL_PROVIDER *EVP_KEM_get0_provider(const EVP_KEM *wrap);
EVP_KEM *EVP_KEM_fetch(OSSL_LIB_CTX *ctx, const char *algorithm,
    const char *properties);
int EVP_KEM_is_a(const EVP_KEM *wrap, const char *name);
const char *EVP_KEM_get0_name(const EVP_KEM *wrap);
const char *EVP_KEM_get0_description(const EVP_KEM *wrap);
void EVP_KEM_do_all_provided(OSSL_LIB_CTX *libctx,
    void (*fn)(EVP_KEM *wrap, void *arg), void *arg);
int EVP_KEM_names_do_all(const EVP_KEM *wrap,
    void (*fn)(const char *name, void *data), void *data);
const OSSL_PARAM *EVP_KEM_gettable_ctx_params(const EVP_KEM *kem);
const OSSL_PARAM *EVP_KEM_settable_ctx_params(const EVP_KEM *kem);

int EVP_PKEY_sign_init(EVP_PKEY_CTX *ctx);
int EVP_PKEY_sign_init_ex(EVP_PKEY_CTX *ctx, const OSSL_PARAM params[]);
int EVP_PKEY_sign_init_ex2(EVP_PKEY_CTX *ctx,
    EVP_SIGNATURE *algo, const OSSL_PARAM params[]);
int EVP_PKEY_sign(EVP_PKEY_CTX *ctx,
    unsigned char *sig, size_t *siglen,
    const unsigned char *tbs, size_t tbslen);
int EVP_PKEY_sign_message_init(EVP_PKEY_CTX *ctx,
    EVP_SIGNATURE *algo, const OSSL_PARAM params[]);
int EVP_PKEY_sign_message_update(EVP_PKEY_CTX *ctx,
    const unsigned char *in, size_t inlen);
int EVP_PKEY_sign_message_final(EVP_PKEY_CTX *ctx,
    unsigned char *sig, size_t *siglen);
int EVP_PKEY_verify_init(EVP_PKEY_CTX *ctx);
int EVP_PKEY_verify_init_ex(EVP_PKEY_CTX *ctx, const OSSL_PARAM params[]);
int EVP_PKEY_verify_init_ex2(EVP_PKEY_CTX *ctx,
    EVP_SIGNATURE *algo, const OSSL_PARAM params[]);
int EVP_PKEY_verify(EVP_PKEY_CTX *ctx,
    const unsigned char *sig, size_t siglen,
    const unsigned char *tbs, size_t tbslen);
int EVP_PKEY_verify_message_init(EVP_PKEY_CTX *ctx,
    EVP_SIGNATURE *algo, const OSSL_PARAM params[]);
int EVP_PKEY_verify_message_update(EVP_PKEY_CTX *ctx,
    const unsigned char *in, size_t inlen);
int EVP_PKEY_verify_message_final(EVP_PKEY_CTX *ctx);
int EVP_PKEY_verify_recover_init(EVP_PKEY_CTX *ctx);
int EVP_PKEY_verify_recover_init_ex(EVP_PKEY_CTX *ctx,
    const OSSL_PARAM params[]);
int EVP_PKEY_verify_recover_init_ex2(EVP_PKEY_CTX *ctx,
    EVP_SIGNATURE *algo,
    const OSSL_PARAM params[]);
int EVP_PKEY_verify_recover(EVP_PKEY_CTX *ctx,
    unsigned char *rout, size_t *routlen,
    const unsigned char *sig, size_t siglen);
int EVP_PKEY_encrypt_init(EVP_PKEY_CTX *ctx);
int EVP_PKEY_encrypt_init_ex(EVP_PKEY_CTX *ctx, const OSSL_PARAM params[]);
int EVP_PKEY_encrypt(EVP_PKEY_CTX *ctx,
    unsigned char *out, size_t *outlen,
    const unsigned char *in, size_t inlen);
int EVP_PKEY_decrypt_init(EVP_PKEY_CTX *ctx);
int EVP_PKEY_decrypt_init_ex(EVP_PKEY_CTX *ctx, const OSSL_PARAM params[]);
int EVP_PKEY_decrypt(EVP_PKEY_CTX *ctx,
    unsigned char *out, size_t *outlen,
    const unsigned char *in, size_t inlen);

int EVP_PKEY_derive_init(EVP_PKEY_CTX *ctx);
int EVP_PKEY_derive_init_ex(EVP_PKEY_CTX *ctx, const OSSL_PARAM params[]);
int EVP_PKEY_derive_set_peer_ex(EVP_PKEY_CTX *ctx, EVP_PKEY *peer,
    int validate_peer);
int EVP_PKEY_derive_set_peer(EVP_PKEY_CTX *ctx, EVP_PKEY *peer);
int EVP_PKEY_derive(EVP_PKEY_CTX *ctx, unsigned char *key, size_t *keylen);
EVP_SKEY *EVP_PKEY_derive_SKEY(EVP_PKEY_CTX *ctx, EVP_SKEYMGMT *mgmt,
    const char *key_type, const char *propquery,
    size_t keylen, const OSSL_PARAM params[]);

int EVP_PKEY_encapsulate_init(EVP_PKEY_CTX *ctx, const OSSL_PARAM params[]);
int EVP_PKEY_auth_encapsulate_init(EVP_PKEY_CTX *ctx, EVP_PKEY *authpriv,
    const OSSL_PARAM params[]);
int EVP_PKEY_encapsulate(EVP_PKEY_CTX *ctx,
    unsigned char *wrappedkey, size_t *wrappedkeylen,
    unsigned char *genkey, size_t *genkeylen);
int EVP_PKEY_decapsulate_init(EVP_PKEY_CTX *ctx, const OSSL_PARAM params[]);
int EVP_PKEY_auth_decapsulate_init(EVP_PKEY_CTX *ctx, EVP_PKEY *authpub,
    const OSSL_PARAM params[]);
int EVP_PKEY_decapsulate(EVP_PKEY_CTX *ctx,
    unsigned char *unwrapped, size_t *unwrappedlen,
    const unsigned char *wrapped, size_t wrappedlen);
typedef int EVP_PKEY_gen_cb(EVP_PKEY_CTX *ctx);

int EVP_PKEY_fromdata_init(EVP_PKEY_CTX *ctx);
int EVP_PKEY_fromdata(EVP_PKEY_CTX *ctx, EVP_PKEY **ppkey, int selection,
    OSSL_PARAM param[]);
const OSSL_PARAM *EVP_PKEY_fromdata_settable(EVP_PKEY_CTX *ctx, int selection);

int EVP_PKEY_todata(const EVP_PKEY *pkey, int selection, OSSL_PARAM **params);
int EVP_PKEY_export(const EVP_PKEY *pkey, int selection,
    OSSL_CALLBACK *export_cb, void *export_cbarg);

const OSSL_PARAM *EVP_PKEY_gettable_params(const EVP_PKEY *pkey);
int EVP_PKEY_get_params(const EVP_PKEY *pkey, OSSL_PARAM params[]);
int EVP_PKEY_get_int_param(const EVP_PKEY *pkey, const char *key_name,
    int *out);
int EVP_PKEY_get_size_t_param(const EVP_PKEY *pkey, const char *key_name,
    size_t *out);
int EVP_PKEY_get_bn_param(const EVP_PKEY *pkey, const char *key_name,
    BIGNUM **bn);
int EVP_PKEY_get_utf8_string_param(const EVP_PKEY *pkey, const char *key_name,
    char *str, size_t max_buf_sz, size_t *out_sz);
int EVP_PKEY_get_octet_string_param(const EVP_PKEY *pkey, const char *key_name,
    unsigned char *buf, size_t max_buf_sz,
    size_t *out_sz);

const OSSL_PARAM *EVP_PKEY_settable_params(const EVP_PKEY *pkey);
int EVP_PKEY_set_params(EVP_PKEY *pkey, OSSL_PARAM params[]);
int EVP_PKEY_set_int_param(EVP_PKEY *pkey, const char *key_name, int in);
int EVP_PKEY_set_size_t_param(EVP_PKEY *pkey, const char *key_name, size_t in);
int EVP_PKEY_set_bn_param(EVP_PKEY *pkey, const char *key_name,
    const BIGNUM *bn);
int EVP_PKEY_set_utf8_string_param(EVP_PKEY *pkey, const char *key_name,
    const char *str);
int EVP_PKEY_set_octet_string_param(EVP_PKEY *pkey, const char *key_name,
    const unsigned char *buf, size_t bsize);

int EVP_PKEY_get_ec_point_conv_form(const EVP_PKEY *pkey);
int EVP_PKEY_get_field_type(const EVP_PKEY *pkey);

EVP_PKEY *EVP_PKEY_Q_keygen(OSSL_LIB_CTX *libctx, const char *propq,
    const char *type, ...);
int EVP_PKEY_paramgen_init(EVP_PKEY_CTX *ctx);
int EVP_PKEY_paramgen(EVP_PKEY_CTX *ctx, EVP_PKEY **ppkey);
int EVP_PKEY_keygen_init(EVP_PKEY_CTX *ctx);
int EVP_PKEY_keygen(EVP_PKEY_CTX *ctx, EVP_PKEY **ppkey);
int EVP_PKEY_generate(EVP_PKEY_CTX *ctx, EVP_PKEY **ppkey);
int EVP_PKEY_check(EVP_PKEY_CTX *ctx);
int EVP_PKEY_public_check(EVP_PKEY_CTX *ctx);
int EVP_PKEY_public_check_quick(EVP_PKEY_CTX *ctx);
int EVP_PKEY_param_check(EVP_PKEY_CTX *ctx);
int EVP_PKEY_param_check_quick(EVP_PKEY_CTX *ctx);
int EVP_PKEY_private_check(EVP_PKEY_CTX *ctx);
int EVP_PKEY_pairwise_check(EVP_PKEY_CTX *ctx);



int EVP_PKEY_set_ex_data(EVP_PKEY *key, int idx, void *arg);
void *EVP_PKEY_get_ex_data(const EVP_PKEY *key, int idx);

void EVP_PKEY_CTX_set_cb(EVP_PKEY_CTX *ctx, EVP_PKEY_gen_cb *cb);
EVP_PKEY_gen_cb *EVP_PKEY_CTX_get_cb(EVP_PKEY_CTX *ctx);

int EVP_PKEY_CTX_get_keygen_info(EVP_PKEY_CTX *ctx, int idx);

__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_init(EVP_PKEY_METHOD *pmeth,
    int (*init)(EVP_PKEY_CTX *ctx));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_copy(EVP_PKEY_METHOD *pmeth, int (*copy)(EVP_PKEY_CTX *dst, const EVP_PKEY_CTX *src));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_cleanup(EVP_PKEY_METHOD *pmeth, void (*cleanup)(EVP_PKEY_CTX *ctx));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_paramgen(EVP_PKEY_METHOD *pmeth, int (*paramgen_init)(EVP_PKEY_CTX *ctx),
    int (*paramgen)(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_keygen(EVP_PKEY_METHOD *pmeth, int (*keygen_init)(EVP_PKEY_CTX *ctx),
    int (*keygen)(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_sign(EVP_PKEY_METHOD *pmeth, int (*sign_init)(EVP_PKEY_CTX *ctx),
    int (*sign)(EVP_PKEY_CTX *ctx, unsigned char *sig, size_t *siglen,
        const unsigned char *tbs, size_t tbslen));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_verify(EVP_PKEY_METHOD *pmeth, int (*verify_init)(EVP_PKEY_CTX *ctx),
    int (*verify)(EVP_PKEY_CTX *ctx, const unsigned char *sig, size_t siglen,
        const unsigned char *tbs, size_t tbslen));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_verify_recover(EVP_PKEY_METHOD *pmeth, int (*verify_recover_init)(EVP_PKEY_CTX *ctx),
    int (*verify_recover)(EVP_PKEY_CTX *ctx, unsigned char *sig,
        size_t *siglen, const unsigned char *tbs,
        size_t tbslen));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_signctx(EVP_PKEY_METHOD *pmeth, int (*signctx_init)(EVP_PKEY_CTX *ctx, EVP_MD_CTX *mctx),
    int (*signctx)(EVP_PKEY_CTX *ctx, unsigned char *sig, size_t *siglen,
        EVP_MD_CTX *mctx));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_verifyctx(EVP_PKEY_METHOD *pmeth, int (*verifyctx_init)(EVP_PKEY_CTX *ctx, EVP_MD_CTX *mctx),
    int (*verifyctx)(EVP_PKEY_CTX *ctx, const unsigned char *sig, int siglen,
        EVP_MD_CTX *mctx));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_encrypt(EVP_PKEY_METHOD *pmeth, int (*encrypt_init)(EVP_PKEY_CTX *ctx),
    int (*encryptfn)(EVP_PKEY_CTX *ctx, unsigned char *out, size_t *outlen,
        const unsigned char *in, size_t inlen));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_decrypt(EVP_PKEY_METHOD *pmeth, int (*decrypt_init)(EVP_PKEY_CTX *ctx),
    int (*decrypt)(EVP_PKEY_CTX *ctx, unsigned char *out, size_t *outlen,
        const unsigned char *in, size_t inlen));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_derive(EVP_PKEY_METHOD *pmeth, int (*derive_init)(EVP_PKEY_CTX *ctx),
    int (*derive)(EVP_PKEY_CTX *ctx, unsigned char *key, size_t *keylen));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_ctrl(EVP_PKEY_METHOD *pmeth, int (*ctrl)(EVP_PKEY_CTX *ctx, int type, int p1, void *p2),
    int (*ctrl_str)(EVP_PKEY_CTX *ctx, const char *type, const char *value));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_digestsign(EVP_PKEY_METHOD *pmeth,
    int (*digestsign)(EVP_MD_CTX *ctx, unsigned char *sig, size_t *siglen,
        const unsigned char *tbs, size_t tbslen));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_digestverify(EVP_PKEY_METHOD *pmeth,
    int (*digestverify)(EVP_MD_CTX *ctx, const unsigned char *sig,
        size_t siglen, const unsigned char *tbs,
        size_t tbslen));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_check(EVP_PKEY_METHOD *pmeth, int (*check)(EVP_PKEY *pkey));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_public_check(EVP_PKEY_METHOD *pmeth, int (*check)(EVP_PKEY *pkey));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_param_check(EVP_PKEY_METHOD *pmeth, int (*check)(EVP_PKEY *pkey));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_set_digest_custom(EVP_PKEY_METHOD *pmeth, int (*digest_custom)(EVP_PKEY_CTX *ctx, EVP_MD_CTX *mctx));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_init(const EVP_PKEY_METHOD *pmeth, int (**pinit)(EVP_PKEY_CTX *ctx));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_copy(const EVP_PKEY_METHOD *pmeth, int (**pcopy)(EVP_PKEY_CTX *dst, const EVP_PKEY_CTX *src));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_cleanup(const EVP_PKEY_METHOD *pmeth, void (**pcleanup)(EVP_PKEY_CTX *ctx));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_paramgen(const EVP_PKEY_METHOD *pmeth, int (**pparamgen_init)(EVP_PKEY_CTX *ctx),
    int (**pparamgen)(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_keygen(const EVP_PKEY_METHOD *pmeth, int (**pkeygen_init)(EVP_PKEY_CTX *ctx),
    int (**pkeygen)(EVP_PKEY_CTX *ctx, EVP_PKEY *pkey));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_sign(const EVP_PKEY_METHOD *pmeth, int (**psign_init)(EVP_PKEY_CTX *ctx),
    int (**psign)(EVP_PKEY_CTX *ctx, unsigned char *sig, size_t *siglen,
        const unsigned char *tbs, size_t tbslen));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_verify(const EVP_PKEY_METHOD *pmeth, int (**pverify_init)(EVP_PKEY_CTX *ctx),
    int (**pverify)(EVP_PKEY_CTX *ctx, const unsigned char *sig,
        size_t siglen, const unsigned char *tbs, size_t tbslen));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_verify_recover(const EVP_PKEY_METHOD *pmeth,
    int (**pverify_recover_init)(EVP_PKEY_CTX *ctx),
    int (**pverify_recover)(EVP_PKEY_CTX *ctx, unsigned char *sig,
        size_t *siglen, const unsigned char *tbs,
        size_t tbslen));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_signctx(const EVP_PKEY_METHOD *pmeth,
    int (**psignctx_init)(EVP_PKEY_CTX *ctx, EVP_MD_CTX *mctx),
    int (**psignctx)(EVP_PKEY_CTX *ctx, unsigned char *sig, size_t *siglen,
        EVP_MD_CTX *mctx));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_verifyctx(const EVP_PKEY_METHOD *pmeth,
    int (**pverifyctx_init)(EVP_PKEY_CTX *ctx, EVP_MD_CTX *mctx),
    int (**pverifyctx)(EVP_PKEY_CTX *ctx, const unsigned char *sig,
        int siglen, EVP_MD_CTX *mctx));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_encrypt(const EVP_PKEY_METHOD *pmeth, int (**pencrypt_init)(EVP_PKEY_CTX *ctx),
    int (**pencryptfn)(EVP_PKEY_CTX *ctx, unsigned char *out, size_t *outlen,
        const unsigned char *in, size_t inlen));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_decrypt(const EVP_PKEY_METHOD *pmeth, int (**pdecrypt_init)(EVP_PKEY_CTX *ctx),
    int (**pdecrypt)(EVP_PKEY_CTX *ctx, unsigned char *out, size_t *outlen,
        const unsigned char *in, size_t inlen));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_derive(const EVP_PKEY_METHOD *pmeth, int (**pderive_init)(EVP_PKEY_CTX *ctx),
    int (**pderive)(EVP_PKEY_CTX *ctx, unsigned char *key, size_t *keylen));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_ctrl(const EVP_PKEY_METHOD *pmeth,
    int (**pctrl)(EVP_PKEY_CTX *ctx, int type, int p1, void *p2),
    int (**pctrl_str)(EVP_PKEY_CTX *ctx, const char *type,
        const char *value));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_digestsign(const EVP_PKEY_METHOD *pmeth,
    int (**digestsign)(EVP_MD_CTX *ctx, unsigned char *sig, size_t *siglen,
        const unsigned char *tbs, size_t tbslen));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_digestverify(const EVP_PKEY_METHOD *pmeth,
    int (**digestverify)(EVP_MD_CTX *ctx, const unsigned char *sig,
        size_t siglen, const unsigned char *tbs,
        size_t tbslen));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_check(const EVP_PKEY_METHOD *pmeth, int (**pcheck)(EVP_PKEY *pkey));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_public_check(const EVP_PKEY_METHOD *pmeth, int (**pcheck)(EVP_PKEY *pkey));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_param_check(const EVP_PKEY_METHOD *pmeth, int (**pcheck)(EVP_PKEY *pkey));
__attribute__((deprecated("Since OpenSSL " "3.0"))) void EVP_PKEY_meth_get_digest_custom(const EVP_PKEY_METHOD *pmeth,
    int (**pdigest_custom)(EVP_PKEY_CTX *ctx, EVP_MD_CTX *mctx));


void EVP_KEYEXCH_free(EVP_KEYEXCH *exchange);
int EVP_KEYEXCH_up_ref(EVP_KEYEXCH *exchange);
EVP_KEYEXCH *EVP_KEYEXCH_fetch(OSSL_LIB_CTX *ctx, const char *algorithm,
    const char *properties);
OSSL_PROVIDER *EVP_KEYEXCH_get0_provider(const EVP_KEYEXCH *exchange);
int EVP_KEYEXCH_is_a(const EVP_KEYEXCH *keyexch, const char *name);
const char *EVP_KEYEXCH_get0_name(const EVP_KEYEXCH *keyexch);
const char *EVP_KEYEXCH_get0_description(const EVP_KEYEXCH *keyexch);
void EVP_KEYEXCH_do_all_provided(OSSL_LIB_CTX *libctx,
    void (*fn)(EVP_KEYEXCH *keyexch, void *data),
    void *data);
int EVP_KEYEXCH_names_do_all(const EVP_KEYEXCH *keyexch,
    void (*fn)(const char *name, void *data),
    void *data);
const OSSL_PARAM *EVP_KEYEXCH_gettable_ctx_params(const EVP_KEYEXCH *keyexch);
const OSSL_PARAM *EVP_KEYEXCH_settable_ctx_params(const EVP_KEYEXCH *keyexch);

void EVP_add_alg_module(void);

int EVP_PKEY_CTX_set_group_name(EVP_PKEY_CTX *ctx, const char *name);
int EVP_PKEY_CTX_get_group_name(EVP_PKEY_CTX *ctx, char *name, size_t namelen);
int EVP_PKEY_get_group_name(const EVP_PKEY *pkey, char *name, size_t name_sz,
    size_t *gname_len);

OSSL_LIB_CTX *EVP_PKEY_CTX_get0_libctx(EVP_PKEY_CTX *ctx);
const char *EVP_PKEY_CTX_get0_propq(const EVP_PKEY_CTX *ctx);
const OSSL_PROVIDER *EVP_PKEY_CTX_get0_provider(const EVP_PKEY_CTX *ctx);

int EVP_SKEY_is_a(const EVP_SKEY *skey, const char *name);
EVP_SKEY *EVP_SKEY_import(OSSL_LIB_CTX *libctx, const char *skeymgmtname, const char *propquery,
    int selection, const OSSL_PARAM *params);
EVP_SKEY *EVP_SKEY_generate(OSSL_LIB_CTX *libctx, const char *skeymgmtname,
    const char *propquery, const OSSL_PARAM *params);
EVP_SKEY *EVP_SKEY_import_raw_key(OSSL_LIB_CTX *libctx, const char *skeymgmtname,
    unsigned char *key, size_t keylen,
    const char *propquery);
EVP_SKEY *EVP_SKEY_import_SKEYMGMT(OSSL_LIB_CTX *libctx, EVP_SKEYMGMT *skeymgmt,
    int selection, const OSSL_PARAM *params);
int EVP_SKEY_get0_raw_key(const EVP_SKEY *skey, const unsigned char **key,
    size_t *len);
const char *EVP_SKEY_get0_key_id(const EVP_SKEY *skey);
int EVP_SKEY_export(const EVP_SKEY *skey, int selection,
    OSSL_CALLBACK *export_cb, void *export_cbarg);
int EVP_SKEY_up_ref(EVP_SKEY *skey);
void EVP_SKEY_free(EVP_SKEY *skey);
const char *EVP_SKEY_get0_skeymgmt_name(const EVP_SKEY *skey);
const char *EVP_SKEY_get0_provider_name(const EVP_SKEY *skey);
EVP_SKEY *EVP_SKEY_to_provider(EVP_SKEY *skey, OSSL_LIB_CTX *libctx,
    OSSL_PROVIDER *prov, const char *propquery);

typedef void WGetWriteCallback(void *opaque, int err, void *data, size_t size);
typedef size_t WGetReadCallback(void *opaque, void *data, size_t size);
typedef struct XHRState XHRState;

XHRState *fs_wget(const char *url, const char *user, const char *password,
                  void *opaque, WGetWriteCallback *cb, BOOL single_write);
void fs_wget_free(XHRState *s);

void fs_wget_init(void);
void fs_wget_end(void);


typedef BOOL FSNetEventLoopCompletionFunc(void *opaque);
void fs_net_set_fdset(int *pfd_max, fd_set *rfds, fd_set *wfds, fd_set *efds,
                      int *ptimeout);
void fs_net_event_loop(FSNetEventLoopCompletionFunc *cb, void *opaque);




extern const uint8_t encrypted_file_magic[4];

typedef int DecryptFileCB(void *opaque, const uint8_t *data, size_t len);
typedef struct DecryptFileState DecryptFileState;

DecryptFileState *decrypt_file_init(AES_KEY *aes_state,
                                    DecryptFileCB *write_cb,
                                    void *opaque);
int decrypt_file(DecryptFileState *s, const uint8_t *data,
                 size_t size);
int decrypt_file_flush(DecryptFileState *s);
void decrypt_file_end(DecryptFileState *s);

void pbkdf2_hmac_sha256(const uint8_t *pwd, int pwd_len,
                        const uint8_t *salt, int salt_len,
                        int iter, int key_len, uint8_t *out);



typedef void FSWGetFileCB(FSDevice *fs, FSFile *f, int64_t size, void *opaque);

void fs_wget_file2(FSDevice *fs, FSFile *f, const char *url,
                   const char *user, const char *password,
                   FSFile *posted_file, uint64_t post_data_len,
                   FSWGetFileCB *cb, void *opaque,
                   AES_KEY *aes_state);



typedef struct {



    uint8_t *data;

    size_t allocated_size;
} FileBuffer;

void file_buffer_init(FileBuffer *bs);
void file_buffer_reset(FileBuffer *bs);
int file_buffer_resize(FileBuffer *bs, size_t new_size);
void file_buffer_write(FileBuffer *bs, size_t offset, const uint8_t *buf,
                       size_t size);
void file_buffer_set(FileBuffer *bs, size_t offset, int val, size_t size);
void file_buffer_read(FileBuffer *bs, size_t offset, uint8_t *buf,
                      size_t size);
typedef enum {
    FT_FIFO = 1,
    FT_CHR = 2,
    FT_DIR = 4,
    FT_BLK = 6,
    FT_REG = 8,
    FT_LNK = 10,
    FT_SOCK = 12,
} FSINodeTypeEnum;

typedef enum {
    REG_STATE_LOCAL,
    REG_STATE_UNLOADED,
    REG_STATE_LOADING,
    REG_STATE_LOADED,
} FSINodeRegStateEnum;

typedef struct FSBaseURL {
    struct list_head link;
    int ref_count;
    char *base_url_id;
    char *url;
    char *user;
    char *password;
    BOOL encrypted;
    AES_KEY aes_state;
} FSBaseURL;

typedef struct FSINode {
    struct list_head link;
    uint64_t inode_num;
    int32_t refcount;
    int32_t open_count;
    FSINodeTypeEnum type;
    uint32_t mode;
    uint32_t uid;
    uint32_t gid;
    uint32_t mtime_sec;
    uint32_t ctime_sec;
    uint32_t mtime_nsec;
    uint32_t ctime_nsec;
    union {
        struct {
            FSINodeRegStateEnum state;
            size_t size;
            FileBuffer fbuf;
            FSBaseURL *base_url;
            FSFileID file_id;
            struct list_head link;
            struct FSOpenInfo *open_info;
            BOOL is_fscmd;

            char *filename;

        } reg;
        struct {
            struct list_head de_list;
            int size;
        } dir;
        struct {
            uint32_t major;
            uint32_t minor;
        } dev;
        struct {
            char *name;
        } symlink;
    } u;
} FSINode;

typedef struct {
    struct list_head link;
    FSINode *inode;
    uint8_t mark;
    char name[0];
} FSDirEntry;

typedef enum {
    FS_CMD_XHR,
    FS_CMD_PBKDF2,
} FSCMDRequestEnum;



typedef struct {
    FSCMDRequestEnum type;
    struct CmdXHRState *xhr_state;
    int reply_len;
    uint8_t reply_buf[64];
} FSCMDRequest;

struct FSFile {
    uint32_t uid;
    FSINode *inode;
    BOOL is_opened;
    uint32_t open_flags;
    FSCMDRequest *req;
};

typedef struct {
    struct list_head link;
    BOOL is_archive;
    const char *name;
} PreloadFile;

typedef struct {
    struct list_head link;
    FSFileID file_id;
    struct list_head file_list;
} PreloadEntry;

typedef struct {
    struct list_head link;
    FSFileID file_id;
    uint64_t size;
    const char *name;
} PreloadArchiveFile;

typedef struct {
    struct list_head link;
    const char *name;
    struct list_head file_list;
} PreloadArchive;

typedef struct FSDeviceMem {
    FSDevice common;

    struct list_head inode_list;
    int64_t inode_count;
    uint64_t inode_limit;
    int64_t fs_blocks;
    uint64_t fs_max_blocks;
    uint64_t inode_num_alloc;
    int block_size_log2;
    uint32_t block_size;
    FSINode *root_inode;
    struct list_head inode_cache_list;
    int64_t inode_cache_size;
    int64_t inode_cache_size_limit;
    struct list_head preload_list;
    struct list_head preload_archive_list;

    struct list_head base_url_list;
    char *import_dir;

    BOOL dump_cache_load;
    BOOL dump_started;
    char *dump_preload_dir;
    FILE *dump_preload_file;
    FILE *dump_preload_archive_file;

    char *dump_archive_name;
    uint64_t dump_archive_size;
    FILE *dump_archive_file;

    int dump_archive_num;
    struct list_head dump_preload_list;
    struct list_head dump_exclude_list;

} FSDeviceMem;

typedef enum {
    FS_OPEN_WGET_REG,
    FS_OPEN_WGET_ARCHIVE,
    FS_OPEN_WGET_ARCHIVE_FILE,
} FSOpenWgetEnum;

typedef struct FSOpenInfo {
    FSDevice *fs;
    FSOpenWgetEnum open_type;


    XHRState *xhr;
    FSINode *n;
    DecryptFileState *dec_state;
    size_t cur_pos;

    struct list_head archive_link;
    uint64_t archive_offset;
    struct list_head archive_file_list;


    FSFile *f;
    FSOpenCompletionFunc *cb;
    void *opaque;
} FSOpenInfo;

static void fs_close(FSDevice *fs, FSFile *f);
static void inode_decref(FSDevice *fs1, FSINode *n);
static int fs_cmd_write(FSDevice *fs, FSFile *f, uint64_t offset,
                        const uint8_t *buf, int buf_len);
static int fs_cmd_read(FSDevice *fs, FSFile *f, uint64_t offset,
                       uint8_t *buf, int buf_len);
static int fs_truncate(FSDevice *fs1, FSINode *n, uint64_t size);
static void fs_open_end(FSOpenInfo *oi);
static void fs_base_url_decref(FSDevice *fs, FSBaseURL *bu);
static FSBaseURL *fs_net_set_base_url(FSDevice *fs1,
                                      const char *base_url_id,
                                      const char *url,
                                      const char *user, const char *password,
                                      AES_KEY *aes_state);
static void fs_cmd_close(FSDevice *fs, FSFile *f);
static void fs_error_archive(FSOpenInfo *oi);

static void dump_loaded_file(FSDevice *fs1, FSINode *n);




void file_buffer_init(FileBuffer *bs)
{
    // Prototype for the Rust function
void file_buffer_init__part635(FileBuffer *bs);

// Call to the Rust function, replacing the original TARGET
file_buffer_init__part635(bs);
}

void file_buffer_reset(FileBuffer *bs)
{
    void file_buffer_reset__part637(FileBuffer *bs);
}

int file_buffer_resize(FileBuffer *bs, size_t new_size)
{
    uint8_t *file_buffer_resize__part639(FileBuffer *bs, size_t new_size);
uint8_t *new_data = file_buffer_resize__part639(bs, new_size);
    int file_buffer_resize__part640(uint8_t *new_data, size_t new_size)
{
    if (!new_data && new_size != 0)
        return -1;
    return 0;
}
    int file_buffer_resize__part641(FileBuffer *bs, uint8_t *new_data, size_t new_size);
return file_buffer_resize__part641(bs, new_data, new_size);
}

void file_buffer_write(FileBuffer *bs, size_t offset, const uint8_t *buf,
                       size_t size)
{
    void file_buffer_write__part643(FileBuffer *bs, size_t offset, const uint8_t *buf, size_t size);
file_buffer_write__part643(bs, offset, buf, size);
}

void file_buffer_set(FileBuffer *bs, size_t offset, int val, size_t size)
{
    void file_buffer_set__part645(void *dest, size_t offset, int val, size_t size);

void file_buffer_set(FileBuffer *bs, size_t offset, int val, size_t size)
{
    file_buffer_set__part645(bs->data, offset, val, size);
}
}

void file_buffer_read(FileBuffer *bs, size_t offset, uint8_t *buf,
                       size_t size)
{
    void file_buffer_read__part647(FileBuffer *bs, size_t offset, uint8_t *buf, size_t size);

memcpy(buf, bs->data + offset, size);
}


static int64_t to_blocks(FSDeviceMem *fs, uint64_t size)
{
    int64_t to_blocks__part649(uint64_t size, uint32_t block_size, int block_size_log2);
    return to_blocks__part649(size, fs->block_size, fs->block_size_log2);

}

static FSINode *inode_incref(FSDevice *fs, FSINode *n)
{
    extern FSINode *inode_incref__part651(FSDevice *fs, FSINode *n);

n->refcount++;
return n;
}

static FSINode *inode_inc_open(FSDevice *fs, FSINode *n)
{
    FSINode *inode_inc_open__part653(FSDevice *fs, FSINode *n);

n->open_count++;
return n;
}

static void inode_free(FSDevice *fs1, FSINode *n)
{
    void *inode_free__part655(void *fs1);

FSDeviceMem *fs = (FSDeviceMem *)inode_free__part655(fs1);
    void inode_free__part656(FSDevice *fs1, FSINode *n);
    inode_free__part656(fs1, n);

    void inode_free__part657(FSDeviceMem *fs, FSINode *n);
    inode_free__part657(fs, n);

}

static void inode_decref(FSDevice *fs1, FSINode *n)
{
    
   void inode_decref__part659(int refcount);
inode_decref__part659(n->refcount);
    int inode_decref__part660(FSINode *n);
    if (inode_decref__part660(n)) {
        inode_free(fs1, n);
    }
}

static void inode_dec_open(FSDevice *fs1, FSINode *n)
{
    
   extern void inode_dec_open__part662(
    int open_count,
    const char *assertion,
    const char *file,
    unsigned int line,
    const char *function
);

inode_dec_open__part662(
    n->open_count,
    "n->open_count >= 1",
    "fs_net.c",
    397,
    __extension__ __PRETTY_FUNCTION__
);
    void inode_dec_open__part663(FSDevice *fs1, FSINode *n);
inode_dec_open__part663(fs1, n);
}

static void inode_update_mtime(FSDevice *fs, FSINode *n)
{
    void inode_update_mtime__part665(FSINode *n);
inode_update_mtime__part665(n);
}

static FSINode *inode_new(FSDevice *fs1, FSINodeTypeEnum type,
                          uint32_t mode, uint32_t uid, uint32_t gid)
{
    FSINode *inode_new__part667(FSDevice *fs1, FSINodeTypeEnum type,
                            uint32_t mode, uint32_t uid, uint32_t gid);
    FSDeviceMem *fs = (FSDeviceMem *)fs1;
    FSINode *n;

    n = inode_new__part667(fs1, type, mode, uid, gid);


        void inode_new__part668(int type, FSINode *n);
    inode_new__part668(type, n);


        FSINode *inode_new__part669(FSDevice *fs1, FSINode *n);
    return inode_new__part669(fs1, n);
}



static FSDirEntry *inode_dir_add(FSDevice *fs1, FSINode *n, const char *name,
                                 FSINode *n1)
{
    FSDirEntry *inode_dir_add__part671(FSDevice *fs1, FSINode *n, const char *name, FSINode *n1);
    return inode_dir_add__part671(fs1, n, name, n1);

}

static FSDirEntry *inode_search(FSINode *n, const char *name)
{
    struct list_head *el;
    FSDirEntry *de;

    extern FSDirEntry *inode_search__part673(FSINode *n, const char *name);

if (n->type != FT_DIR)
    return inode_search__part673(n, name);

        FSDirEntry *inode_search__part674(struct list_head *head, const char *name);
    de = inode_search__part674(&n->u.dir.de_list, name);
    if (de)
        return de;

    FSDirEntry *inode_search__part675();
return inode_search__part675();
}

static FSINode *inode_search_path1(FSDevice *fs, FSINode *n, const char *path)
{
    char name[1024];
    const char *p, *p1;
    int len;
    FSDirEntry *de;

    void inode_search_path1__part677(char *name, const char **p_out, const char *path);
    inode_search_path1__part677(name, &p, path);

    extern void inode_search_path1__part678(const char **p);
inode_search_path1__part678(&p);
    int inode_search_path1__part679(const char *p);
    if (inode_search_path1__part679(p))
        return n;
        {
        FSINode *inode_search_path1__part680(FSINode *n, const char *p);
        (void)name;
        (void)p1;
        (void)len;
        (void)de;
        n = inode_search_path1__part680(n, p);
    }

    void *inode_search_path1__part681(void *n);
    return (FSINode *)inode_search_path1__part681((void *)n);
}

static FSINode *inode_search_path(FSDevice *fs1, const char *path)
{
    FSDeviceMem *fs = (FSDeviceMem *)fs1;
    void *inode_search_path__part683(void);
if (!fs1)
        return inode_search_path__part683();
    FSINode *inode_search_path__part684(FSDevice *fs, FSINode *n, const char *path);
    return inode_search_path__part684(fs1, fs->root_inode, path);

}

static BOOL is_empty_dir(FSDevice *fs, FSINode *n)
{
    struct list_head *el;
    FSDirEntry *de;

    {
    int is_empty_dir__part686(struct list_head *head);
    if (is_empty_dir__part686(&n->u.dir.de_list) == 0)
        return FALSE;
}

    BOOL is_empty_dir__part687(void);
return is_empty_dir__part687();
}

static void inode_dirent_delete_no_decref(FSDevice *fs1, FSINode *n, FSDirEntry *de)
{
        void inode_dirent_delete_no_decref__part689(FSDevice *fs1, FSINode *n, FSDirEntry *de);
    inode_dirent_delete_no_decref__part689(fs1, n, de);

}

static void inode_dirent_delete(FSDevice *fs, FSINode *n, FSDirEntry *de)
{
        void inode_dirent_delete__part691(FSDevice *fs, FSINode *n, FSDirEntry *de);
    inode_dirent_delete__part691(fs, n, de);

}

static void flush_dir(FSDevice *fs, FSINode *n)
{
    struct list_head *el, *el1;
    FSDirEntry *de;
    {
    extern void flush_dir__part693(FSDevice *fs, FSINode *n);
    flush_dir__part693(fs, n);
}

    
   void flush_dir__part694_int_check(int n_dir_size);
}

static void fs_delete(FSDevice *fs, FSFile *f)
{
    void fs_delete__part696(FSDevice *fs, FSFile *f);
    fs_delete__part696(fs, f);

}

static FSFile *fid_create(FSDevice *fs1, FSINode *n, uint32_t uid)
{
        FSFile *fid_create__part698(FSDevice *fs1, FSINode *n, uint32_t uid);

    return fid_create__part698(fs1, n, uid);

}

static void inode_to_qid(FSQID *qid, FSINode *n)
{
    void inode_to_qid__part700(FSQID *qid, int type);
    inode_to_qid__part700(qid, (int)n->type);

    void inode_to_qid__part701(FSQID *qid, FSINode *n);
    inode_to_qid__part701(qid, n);

}

static void fs_statfs(FSDevice *fs1, FSStatFS *st)
{
    void fs_statfs__part703(FSDevice *fs1, FSStatFS *st);
    fs_statfs__part703(fs1, st);

}

static int fs_attach(FSDevice *fs1, FSFile **pf, FSQID *qid, uint32_t uid,
                     const char *uname, const char *aname)
{
    int fs_attach__part705(FSDevice *fs1, FSFile **pf, FSQID *qid, uint32_t uid);
    return fs_attach__part705(fs1, pf, qid, uid);
}

static int fs_walk(FSDevice *fs, FSFile **pf, FSQID *qids,
                   FSFile *f, int count, char **names)
{
    FSINode *fs_walk__part707(FSFile *f);

    int i;
    FSINode *n;
    FSDirEntry *de;

    n = fs_walk__part707(f);
    int fs_walk__part708(FSINode **n, FSQID *qids, int count, char **names);
    i = fs_walk__part708(&n, qids, count, names);

        int fs_walk__part709(FSDevice *fs, FSFile **pf, FSFile *f, FSINode *n, int i);
    return fs_walk__part709(fs, pf, f, n, i);

}

static int fs_mkdir(FSDevice *fs, FSQID *qid, FSFile *f,
                    const char *name, uint32_t mode, uint32_t gid)
{
    FSINode *fs_mkdir__part711(FSFile *f);
    FSINode *n = fs_mkdir__part711(f), *n1;

        {
        int fs_mkdir__part712(FSINode *n);
        if (fs_mkdir__part712(n))
            return -20;
    }

        int fs_mkdir__part713(FSINode *n, const char *name);
    if (fs_mkdir__part713(n, name))
        return -17;

    int fs_mkdir__part714(FSDevice *fs, FSQID *qid, FSFile *f, FSINode *n,
                      const char *name, uint32_t mode, uint32_t gid);
    return fs_mkdir__part714(fs, qid, f, n, name, mode, gid);

}



static void fs_trim_cache(FSDevice *fs1, int64_t added_size)
{
    FSDeviceMem *fs = (FSDeviceMem *)fs1;
    struct list_head *el, *el1;
    FSINode *n;

    int fs_trim_cache__part716(int64_t inode_cache_size, int64_t added_size, int64_t inode_cache_size_limit);
    if (fs_trim_cache__part716(fs->inode_cache_size, added_size, fs->inode_cache_size_limit))
        return;

        void fs_trim_cache__part717(void *fs, int64_t added_size);
    fs_trim_cache__part717((void *)fs, added_size);

}

static void fs_open_end(FSOpenInfo *oi)
{
    void fs_open_end__part719(FSOpenInfo *oi);
    fs_open_end__part719(oi);

    void fs_open_end__part720(DecryptFileState *dec_state);
    fs_open_end__part720(oi->dec_state);
    void fs_open_end__part721(void *oi);
fs_open_end__part721(oi);

}

static int fs_open_write_cb(void *opaque, const uint8_t *data, size_t size)
{
        FSOpenInfo *oi = opaque;
    size_t len;
    FSINode *n;

    void fs_open_write_cb__part723(void *opaque, size_t *out_len, FSINode **out_n);
    fs_open_write_cb__part723(opaque, &len, &n);

    size_t fs_open_write_cb__part724(size_t len, size_t size);
    len = fs_open_write_cb__part724(len, size);

    int fs_open_write_cb__part725(size_t *cur_pos, FileBuffer *fbuf,
                              const uint8_t *data, size_t len);
    return fs_open_write_cb__part725(&oi->cur_pos, &n->u.reg.fbuf, data, len);

}

static void fs_wget_set_loaded(FSINode *n)
{
    void fs_wget_set_loaded__part727(FSINode *n, FSOpenInfo **poi, FSDeviceMem **pfs);
    FSOpenInfo *oi;
    FSDeviceMem *fs;
    FSFile *f;
    FSQID qid;

    fs_wget_set_loaded__part727(n, &oi, &fs);
    (void)f;
    (void)qid;

        if (oi->cb) {
        void fs_wget_set_loaded__part728(FSOpenInfo *oi, FSINode *n, FSQID *qid, FSFile **pf);
        fs_wget_set_loaded__part728(oi, n, &qid, &f);
    }

    void fs_wget_set_loaded__part729(FSOpenInfo *oi);
fs_wget_set_loaded__part729(oi);

}

static void fs_wget_set_error(FSINode *n)
{
    FSOpenInfo *oi;
    FSOpenInfo *fs_wget_set_error__part731(FSINode *n);
    oi = fs_wget_set_error__part731(n);

    void fs_wget_set_error__part732(FSOpenInfo *oi);

if (oi->cb) {
        fs_wget_set_error__part732(oi);
    }

    void fs_wget_set_error__part733(FSOpenInfo *oi);
    fs_wget_set_error__part733(oi);
}

static void fs_read_archive(FSOpenInfo *oi)
{
    FSINode *n = oi->n;
    uint64_t pos, pos1, l;
    uint8_t buf[1024];
    FSINode *n1;
    FSOpenInfo *oi1;
    struct list_head *el, *el1;

        {
        void fs_read_archive__part735(FSOpenInfo *oi, FSINode *n,
                                      void (*set_loaded)(FSINode *n1));

        fs_read_archive__part735(oi, n, fs_wget_set_loaded);
    }

}

static void fs_error_archive(FSOpenInfo *oi)
{
    FSOpenInfo *oi1;
    struct list_head *el, *el1;

        void fs_error_archive__part737(FSOpenInfo *oi);
    fs_error_archive__part737(oi);

}

static void fs_open_cb(void *opaque, int err, void *data, size_t size)
{
    FSOpenInfo *oi = opaque;
    FSINode *n = oi->n;


    void fs_open_cb__part739(FSOpenInfo *oi, FSINode *n, int err, void *data, size_t size);
    fs_open_cb__part739(oi, n, err, data, size);

}


static int fs_open_wget(FSDevice *fs1, FSINode *n, FSOpenWgetEnum open_type)
{
    char *url;
    FSOpenInfo *oi;
    char fname[32];
    FSBaseURL *bu;

    void fs_open_wget__part741(FSDevice *fs1, FSINode *n);
    fs_open_wget__part741(fs1, n);

    {
        extern int fs_open_wget__part742(FSINode *n);
        if (fs_open_wget__part742(n) < 0)
            return -5;
    }

    FSOpenInfo *fs_open_wget__part743(FSDevice *fs1, FSINode *n, FSOpenWgetEnum open_type);
    oi = fs_open_wget__part743(fs1, n, open_type);

    {
        extern void fs_open_wget__part744(FSOpenInfo *oi, int open_type,
                                          FSBaseURL *bu, FSFileID file_id,
                                          void *dec_write_cb, void *wget_cb);
        fs_open_wget__part744(oi, (int)open_type, n->u.reg.base_url,
                              n->u.reg.file_id,
                              (void *)fs_open_write_cb, (void *)fs_open_cb);
    }

        int fs_open_wget__part745(FSINode *n, FSOpenInfo *oi);
    return fs_open_wget__part745(n, oi);

}


static void fs_preload_file(FSDevice *fs1, const char *filename)
{
    FSINode *fs_preload_file__part747(FSDevice *fs1, const char *filename);

    FSINode *n;

    n = fs_preload_file__part747(fs1, filename);

        void fs_preload_file__part748(FSDevice *fs1, FSINode *n);
    fs_preload_file__part748(fs1, n);

}

static PreloadArchive *find_preload_archive(FSDeviceMem *fs,
                                            const char *filename)
{
    PreloadArchive *pa;
    struct list_head *el;
        {
        PreloadArchive *found_pa;
        PreloadArchive *find_preload_archive__part750(struct list_head *head, const char *filename);
        found_pa = find_preload_archive__part750(&fs->preload_archive_list, filename);
        if (found_pa)
            return found_pa;
    }

    void *find_preload_archive__part751(void);
    return (PreloadArchive *)find_preload_archive__part751();

}

static void fs_preload_archive(FSDevice *fs1, const char *filename)
{
    PreloadArchive *fs_preload_archive__part753(const FSDevice *fs1, const char *filename);
    FSDeviceMem *fs = (FSDeviceMem *)fs1;
    PreloadArchive *paf_unused_marker;
    PreloadArchiveFile *paf;
    struct list_head *el;
    FSINode *n, *n1;
    uint64_t offset;
    BOOL has_unloaded;
    PreloadArchive *pa = fs_preload_archive__part753(fs1, filename);
    (void)paf_unused_marker;

    int fs_preload_archive__part754(PreloadArchive *pa);
    if (fs_preload_archive__part754(pa))
        return;



    struct FSINode *fs_preload_archive__part755(void *fs1, const char *filename, struct FSINode *root_inode);
    n = fs_preload_archive__part755((void *)fs1, filename, ((FSDeviceMem *)fs1)->root_inode);
    int fs_preload_archive__part756(FSDevice *fs1, FSINode *n, PreloadArchive *pa,
                                void *write_cb, void *open_cb);
{
    if (fs_preload_archive__part756(fs1, n, pa, (void *)fs_open_write_cb,
                                    (void *)fs_open_cb))
        return;
}

}

static void fs_preload_files(FSDevice *fs1, FSFileID file_id)
{
    FSDeviceMem *fs = (FSDeviceMem *)fs1;
    struct list_head *el;
    PreloadEntry *pe;
    PreloadFile *pf;

        PreloadEntry *fs_preload_files__part758(struct list_head *head, FSFileID file_id);
    pe = fs_preload_files__part758(&fs->preload_list, file_id);
    if (pe != 0)
        goto found;

    void fs_preload_files__part759(void);
    fs_preload_files__part759();
    return;

 found:
    {
        void fs_preload_files__part760(FSDevice *fs1, PreloadEntry *pe,
                                       void (*preload_file)(FSDevice *, const char *),
                                       void (*preload_archive)(FSDevice *, const char *));
        fs_preload_files__part760(fs1, pe, fs_preload_file, fs_preload_archive);
    }

}




static int fs_open(FSDevice *fs1, FSQID *qid, FSFile *f, uint32_t flags,
                   FSOpenCompletionFunc *cb, void *opaque)
{
        void fs_open__part762(FSFile *f, FSDevice *fs1,
                          FSINode **out_n, FSDeviceMem **out_fs);
    FSINode *n;
    FSDeviceMem *fs;
    int ret;

    fs_open__part762(f, fs1, &n, &fs);


        {
        int fs_open__part763(uint32_t flags, int n_type);
        int fs_open__part763_ret = fs_open__part763(flags, (int)n->type);
        if (fs_open__part763_ret != 0)
            return fs_open__part763_ret;
    }

    void fs_open__part764(FSFile *f, uint32_t flags);
fs_open__part764(f, flags);
        int fs_open__part765(FSDevice *fs1, FSQID *qid, FSFile *f, uint32_t flags,
                         void *cb, void *opaque,
                         struct list_head *inode_cache_list,
                         void (*preload_files)(FSDevice *, uint64_t),
                         int (*open_wget)(FSDevice *, FSINode *, int),
                         int (*truncate_fn)(FSDevice *, FSINode *, uint64_t));
    ret = fs_open__part765(fs1, qid, f, flags, (void *)cb, opaque,
                           &fs->inode_cache_list, fs_preload_files,
                           (int (*)(FSDevice *, FSINode *, int))fs_open_wget,
                           fs_truncate);
    return ret;

}

static int fs_create(FSDevice *fs, FSQID *qid, FSFile *f, const char *name,
                     uint32_t flags, uint32_t mode, uint32_t gid)
{
    FSINode *n1, *n = f->inode;

        {
        int fs_create__part767(FSINode *n);
        if (fs_create__part767(n))
            return -20;
    }

        {
        int fs_create__part768(FSDevice *fs, FSQID *qid, FSFile *f,
                               const char *name, uint32_t flags, uint32_t mode,
                               uint32_t gid);
        int ret;

        ret = fs_create__part768(fs, qid, f, name, flags, mode, gid);
        return ret;
    }

}

static int fs_readdir(FSDevice *fs, FSFile *f, uint64_t offset1,
                      uint8_t *buf, int count)
{
    FSINode *n1, *n = f->inode;
    int len, pos, name_len, type;
    struct list_head *el;
    FSDirEntry *de;
    uint64_t offset;

    {
    extern int fs_readdir__part770(const FSFile *f, const FSINode *n);
    int rc770 = fs_readdir__part770(f, n);
    if (rc770 != 0)
        return rc770;
}


    struct list_head *fs_readdir__part771(FSINode *n, uint64_t *offset);
    el = fs_readdir__part771(n, &offset);
    int fs_readdir__part772(FSINode *n, uint64_t offset1, uint64_t *offset, struct list_head **el);
    if (fs_readdir__part772(n, offset1, &offset, &el))
        return 0;

    int fs_readdir__part773(void);
pos = fs_readdir__part773();
    {
    void fs_readdir__part774(uint64_t *offset, struct list_head **el, int *pos,
                             int count, uint8_t *buf, FSINode *n);
    fs_readdir__part774(&offset, &el, &pos, count, buf, n);
}

    int fs_readdir__part775(int pos);
    return fs_readdir__part775(pos);

}

static int fs_read(FSDevice *fs, FSFile *f, uint64_t offset,
                   uint8_t *buf, int count)
{
    FSINode *n = f->inode;
    uint64_t count1;

        int fs_read__part777(FSFile *f);
    if (fs_read__part777(f))
        return -71;

    extern int fs_read__part778(int n_type);
    if (fs_read__part778(n->type))
        return -5;

    int fs_read__part779(FSFile *f);
    if (fs_read__part779(f))
        return -5;
        {
        int fs_read_part780_ret = 0;
        int fs_read_part780_hit;
        int fs_read__part780(FSDevice *fs, FSFile *f, uint64_t offset,
                             uint8_t *buf, int count, int is_fscmd, int *ret);
        fs_read_part780_hit = fs_read__part780(fs, f, offset, buf, count,
                                               (int)n->u.reg.is_fscmd,
                                               &fs_read_part780_ret);
        if (fs_read_part780_hit)
            return fs_read_part780_ret;
    }

    int fs_read__part781(uint64_t offset, size_t size);
    if (fs_read__part781(offset, n->u.reg.size))
        return 0;
    uint64_t fs_read__part782(uint64_t size, uint64_t offset);
count1 = fs_read__part782((uint64_t)n->u.reg.size, offset);

    {
    int fs_read__part783(uint64_t count1, int count);
    count = fs_read__part783(count1, count);
}

    int fs_read__part784(FileBuffer *bs, size_t offset, uint8_t *buf, int count);
    return fs_read__part784(&n->u.reg.fbuf, offset, buf, count);

}

static int fs_truncate(FSDevice *fs1, FSINode *n, uint64_t size)
{
    FSDeviceMem *fs = (FSDeviceMem *)fs1;
    intptr_t diff, diff_blocks;
    size_t new_allocated_size;

        int fs_truncate__part786(const FSINode *n);

    if (fs_truncate__part786(n))
        return -22;

    int fs_truncate__part787(uint64_t size);
    if (fs_truncate__part787(size) != 0)
        return -28;

        intptr_t fs_truncate__part788(uint64_t size, size_t reg_size);
    diff = fs_truncate__part788(size, n->u.reg.size);
    int fs_truncate__part789(intptr_t diff);
    if (fs_truncate__part789(diff))
        return 0;

    intptr_t fs_truncate__part790(uint32_t block_size, int block_size_log2, uint64_t size, uint64_t old_size);
    diff_blocks = fs_truncate__part790(fs->block_size, fs->block_size_log2, size, n->u.reg.size);

    {
    int fs_truncate__part791(FSDeviceMem *fs, FSINode *n, uint64_t size,
                             intptr_t diff, intptr_t diff_blocks);
    int fs_truncate__part791_ret =
        fs_truncate__part791(fs, n, size, diff, diff_blocks);
    (void)new_allocated_size;
    if (fs_truncate__part791_ret < 0)
        return fs_truncate__part791_ret;
}

        extern int fs_truncate__part792(FSDeviceMem *fs, intptr_t diff_blocks,
                                    FSINode *n, uint64_t size);
    return fs_truncate__part792(fs, diff_blocks, n, size);

}

static int fs_write(FSDevice *fs1, FSFile *f, uint64_t offset,
                    const uint8_t *buf, int count)
{
    FSDeviceMem *fs = (FSDeviceMem *)fs1;
    FSINode *n = f->inode;
    uint64_t end;
    int err;

    {
        int fs_write__part794(FSFile *f);
        if (!fs_write__part794(f))
            return -71;
    }

    {
        int fs_write__part795(const FSINode *n);
        int _fs_write_part795_rc = fs_write__part795(n);
        if (_fs_write_part795_rc)
            return _fs_write_part795_rc;
    }

    int fs_write__part796(uint32_t open_flags);
    if (fs_write__part796(f->open_flags))
        return -5;
    int fs_write__part797(int count);
    if (fs_write__part797(count))
        return 0;

        {
        int is_fscmd;
        int fs_write__part798(FSFile *f);
        is_fscmd = fs_write__part798(f);
        if (is_fscmd) {
            return fs_cmd_write(fs1, f, offset, buf, count);
        }
    }

    uint64_t fs_write__part799(uint64_t offset, int count);
    end = fs_write__part799(offset, count);

        if (end > n->u.reg.size) {
        int fs_write__part800(FSDevice *fs1, FSINode *n, uint64_t end);
        err = fs_write__part800(fs1, n, end);
        if (err)
            return err;
    }

    void fs_write__part801(FSDevice *fs, FSINode *n);
    fs_write__part801(fs1, n);

    void fs_write__part802(FSDeviceMem *fs, FSINode *n);
    fs_write__part802(fs, n);

        {
        int fs_write__part803(FileBuffer *fbuf, uint64_t offset,
                              const uint8_t *buf, int count);
        return fs_write__part803(&n->u.reg.fbuf, offset, buf, count);
    }

}

static void fs_close(FSDevice *fs, FSFile *f)
{
    void fs_close__part805(struct FSFile *f);
    fs_close__part805(f);

    void fs_close__part806(FSDevice *fs, FSFile *f);
    if (f->req)
        fs_close__part806(fs, f);

}

static int fs_stat(FSDevice *fs1, FSFile *f, FSStat *st)
{
    void fs_stat__part808(void *fs1, FSFile *f, FSStat *st);
    fs_stat__part808(fs1, f, st);
    FSDeviceMem *fs = (FSDeviceMem *)fs1;
    FSINode *n = f->inode;

    uint64_t fs_stat__part809(const FSINode *n);
    st->st_rdev = fs_stat__part809(n);

    void fs_stat__part810(uint64_t *st_blksize, uint32_t block_size);
fs_stat__part810(&st->st_blksize, fs->block_size);

        uint64_t fs_stat__part811(const FSINode *n);
    st->st_size = fs_stat__part811(n);

        {
        int fs_stat__part812(uint64_t *st_blocks, const uint64_t *st_size,
                             uint64_t *st_atime_sec, uint32_t *st_atime_nsec,
                             uint64_t *st_mtime_sec, uint32_t *st_mtime_nsec,
                             uint64_t *st_ctime_sec, uint32_t *st_ctime_nsec,
                             uint32_t block_size, int block_size_log2,
                             uint32_t mtime_sec, uint32_t mtime_nsec,
                             uint32_t ctime_sec, uint32_t ctime_nsec);
        return fs_stat__part812(&st->st_blocks, &st->st_size,
                                &st->st_atime_sec, &st->st_atime_nsec,
                                &st->st_mtime_sec, &st->st_mtime_nsec,
                                &st->st_ctime_sec, &st->st_ctime_nsec,
                                fs->block_size, fs->block_size_log2,
                                n->mtime_sec, n->mtime_nsec,
                                n->ctime_sec, n->ctime_nsec);
    }

}

static int fs_setattr(FSDevice *fs1, FSFile *f, uint32_t mask,
                      uint32_t mode, uint32_t uid, uint32_t gid,
                      uint64_t size, uint64_t atime_sec, uint64_t atime_nsec,
                      uint64_t mtime_sec, uint64_t mtime_nsec)
{
    FSINode *n = f->inode;
    int ret;

        void fs_setattr__part814(FSINode *n, uint32_t mask, uint32_t mode);
    fs_setattr__part814(n, mask, mode);

    void fs_setattr__part815(FSINode *n, uint32_t uid);
    if (mask & 0x00000002) {
        fs_setattr__part815(n, uid);
    }
    void fs_setattr__part816(uint32_t mask, uint32_t gid, uint32_t *gid_ptr);
    fs_setattr__part816(mask, gid, &n->gid);

    int fs_setattr__part817(FSDevice *fs1, FSINode *n, uint32_t mask, uint64_t size);
    ret = fs_setattr__part817(fs1, n, mask, size);
    if (ret)
        return ret;

        if (mask & 0x00000020) {
        void fs_setattr__part818(void *fs1, FSINode *n, uint32_t mask,
                                 uint64_t mtime_sec, uint64_t mtime_nsec);
        fs_setattr__part818(fs1, n, mask, mtime_sec, mtime_nsec);
    }
    if (mask & 0x00000040) {
        void fs_setattr__part819(FSINode *n);
        fs_setattr__part819(n);
    }
        int fs_setattr__part820(void);
    return fs_setattr__part820();

}

static int fs_link(FSDevice *fs, FSFile *df, FSFile *f, const char *name)
{
    FSINode *n = df->inode;

    int fs_link__part822(struct FSFile *f);
    if (fs_link__part822(f))
        return -1;

    extern int fs_link__part823(FSINode *n, const char *name);

    if (fs_link__part823(n, name))
        return -17;

    void fs_link__part824(FSDevice *fs, FSINode *n, const char *name, FSINode *n1);
    fs_link__part824(fs, n, name, f->inode);
    return 0;
}

static int fs_symlink(FSDevice *fs, FSQID *qid,
                      FSFile *f, const char *name, const char *symgt, uint32_t gid)
{
    FSINode *n1, *n = f->inode;

        int fs_symlink__part826(FSINode *n, const char *name);
    if (fs_symlink__part826(n, name))
        return -17;

        int fs_symlink__part827(FSDevice *fs, FSQID *qid, FSINode **pn1,
                            FSFile *f, const char *name, const char *symgt,
                            uint32_t gid);
    return fs_symlink__part827(fs, qid, &n1, f, name, symgt, gid);

}

static int fs_mknod(FSDevice *fs, FSQID *qid,
             FSFile *f, const char *name, uint32_t mode, uint32_t major,
             uint32_t minor, uint32_t gid)
{
        int fs_mknod__part829(uint32_t mode, FSFile *f, FSINode **out_n1, FSINode **out_n);
    int type;
    FSINode *n1, *n;

    type = fs_mknod__part829(mode, f, &n1, &n);


        {
        int fs_mknod__part830(int type);
        if (fs_mknod__part830(type))
            return -22;
    }

    int fs_mknod__part831(FSINode *n, const char *name);
    if (fs_mknod__part831(n, name))
        return -17;

        extern FSINode *fs_mknod__part832(FSDevice *fs, FSFile *f, int type, uint32_t mode, uint32_t gid);
    n1 = fs_mknod__part832(fs, f, type, mode, gid);
    void fs_mknod__part833(FSINode *n1, int type, uint32_t major, uint32_t minor);
    fs_mknod__part833(n1, type, major, minor);

        {
        int fs_mknod__part834(FSDevice *fs, FSQID *qid, FSINode *n,
                              const char *name, FSINode *n1);
        return fs_mknod__part834(fs, qid, n, name, n1);
    }

}

static int fs_readlink(FSDevice *fs, char *buf, int buf_size, FSFile *f)
{
    FSINode *n = f->inode;
    int len;
        extern int fs_readlink__part836(FSINode *n);
    if (fs_readlink__part836(n))
        return -5;

        {
        int fs_readlink__part837(char *buf, int buf_size, FSINode *n);
        return fs_readlink__part837(buf, buf_size, n);
    }
}

static int fs_renameat(FSDevice *fs, FSFile *f, const char *name,
                       FSFile *new_f, const char *new_name)
{
        FSDirEntry *de, *de1;
    FSINode *n1;

    FSDirEntry *fs_renameat__part839(FSINode *n, const char *name);
    de = fs_renameat__part839(f->inode, name);

    int fs_renameat__part840(void *de);
    if (fs_renameat__part840(de))
        return -2;

    FSDirEntry *fs_renameat__part841(FSFile *new_f, const char *new_name, FSINode **n1);
    de1 = fs_renameat__part841(new_f, new_name, &n1);

        int fs_renameat__part842(FSDevice *fs, FSFile *new_f, FSDirEntry *de1, FSINode **pn1);
    if (de1) {
        int renameat_err = fs_renameat__part842(fs, new_f, de1, &n1);
        if (renameat_err)
            return renameat_err;
    }
    void fs_renameat__part843(FSDevice *fs, FSFile *f, FSFile *new_f, const char *new_name, FSDirEntry *de);
    fs_renameat__part843(fs, f, new_f, new_name, de);
    void fs_renameat__part844(FSDevice *fs, FSINode *n1);

    if (n1)
        fs_renameat__part844(fs, n1);

    int fs_renameat__part845(void);
    return fs_renameat__part845();

}

static int fs_unlinkat(FSDevice *fs, FSFile *f, const char *name)
{
    FSDirEntry *de;
    FSINode *n;

    int fs_unlinkat__part847(const char *name);
    if (fs_unlinkat__part847(name))
        return -2;

    FSDirEntry *fs_unlinkat__part848(FSINode *n, const char *name);
    de = fs_unlinkat__part848(f->inode, name);
        {
        int fs_unlinkat__part849(FSDirEntry *de);
        int __fs_unlinkat_r849 = fs_unlinkat__part849(de);
        if (__fs_unlinkat_r849)
            return __fs_unlinkat_r849;
    }

        {
        FSINode *(*fs_unlinkat__part850)(FSDirEntry *de);
        n = fs_unlinkat__part850(de);
    }

    int fs_unlinkat__part851(FSDevice *fs, FSINode *n);
int ret851 = fs_unlinkat__part851(fs, n);
if (ret851 < 0)
    return ret851;

    int fs_unlinkat__part852(FSDevice *fs, FSINode *n, FSDirEntry *de);
    return fs_unlinkat__part852(fs, f->inode, de);
}

static int fs_lock(FSDevice *fs, FSFile *f, const FSLock *lock)
{
    FSINode *n = f->inode;
    extern int fs_lock__part854(const FSFile *f);
{
    int fs_lock__part854_rc = fs_lock__part854(f);
    if (fs_lock__part854_rc)
        return fs_lock__part854_rc;
}
    int fs_lock__part855(FSINode *n);
    {
        int fs_lock__part855_ret = fs_lock__part855(n);
        if (fs_lock__part855_ret != 0)
            return fs_lock__part855_ret;
    }


    int fs_lock__part856(void);
    return fs_lock__part856();

}

static int fs_getlock(FSDevice *fs, FSFile *f, FSLock *lock)
{
    FSINode *n = f->inode;
    int fs_getlock__part858(FSFile *f);
    if (fs_getlock__part858(f))
        return -71;

    {
        int fs_getlock__part859(int node_type);
        int fs_getlock__part859_ret = fs_getlock__part859((int)n->type);
        if (fs_getlock__part859_ret != 0)
            return fs_getlock__part859_ret;
    }


        { extern int fs_getlock__part860(void); return fs_getlock__part860(); }
}


static void fs_mem_end(FSDevice *fs1)
{
    FSDeviceMem *fs = (FSDeviceMem *)fs1;
    struct list_head *el, *el1, *el2, *el3;
    FSINode *n;
    FSDirEntry *de;

        void fs_mem_end__part862(FSDevice *fs1);
    fs_mem_end__part862(fs1);

    
       void fs_mem_end__part863(struct list_head *inode_cache_list, char *import_dir);
    fs_mem_end__part863(&fs->inode_cache_list, fs->import_dir);

}

FSDevice *fs_mem_init(void)
{
        FSDevice *fs_mem_init__part865(void);
    void fs_mem_init__part865_rest(FSDevice *fs1);
    FSDeviceMem *fs;

    fs = (FSDeviceMem *)fs_mem_init__part865();

    fs->common.fs_end = fs_mem_end;
    fs->common.fs_delete = fs_delete;
    fs->common.fs_statfs = fs_statfs;
    fs->common.fs_attach = fs_attach;
    fs->common.fs_walk = fs_walk;
    fs->common.fs_mkdir = fs_mkdir;
    fs->common.fs_open = fs_open;
    fs->common.fs_create = fs_create;
    fs->common.fs_stat = fs_stat;
    fs->common.fs_setattr = fs_setattr;
    fs->common.fs_close = fs_close;
    fs->common.fs_readdir = fs_readdir;
    fs->common.fs_read = fs_read;
    fs->common.fs_write = fs_write;
    fs->common.fs_link = fs_link;
    fs->common.fs_symlink = fs_symlink;
    fs->common.fs_mknod = fs_mknod;
    fs->common.fs_readlink = fs_readlink;
    fs->common.fs_renameat = fs_renameat;
    fs->common.fs_unlinkat = fs_unlinkat;
    fs->common.fs_lock = fs_lock;
    fs->common.fs_getlock = fs_getlock;

    fs_mem_init__part865_rest(&fs->common);

    return (FSDevice *)fs;

}

static BOOL fs_is_net(FSDevice *fs)
{
    int fs_is_net__part867(const void *slot, const void *mem_end);
return fs_is_net__part867((const void *)fs->fs_end, (const void *)fs_mem_end);

}

static FSBaseURL *fs_find_base_url(FSDevice *fs1,
                                   const char *base_url_id)
{
    FSDeviceMem *fs = (FSDeviceMem *)fs1;
    struct list_head *el;
    FSBaseURL *bu;

    {
        FSBaseURL *fs_find_base_url__part869(FSDevice *fs1, const char *base_url_id);
        for(el = (&fs->base_url_list)->next; el != (&fs->base_url_list); el = el->next) {
            bu = fs_find_base_url__part869(fs1, base_url_id);
            if (bu)
                return bu;
        }
    }
    FSBaseURL *fs_find_base_url__part870(void);
    return fs_find_base_url__part870();

}

static void fs_base_url_decref(FSDevice *fs, FSBaseURL *bu)
{
    
   void fs_base_url_decref__part872(FSBaseURL *bu);
    fs_base_url_decref__part872(bu);

    void fs_base_url_decref__part873(FSBaseURL *bu);
    fs_base_url_decref__part873(bu);
}

static FSBaseURL *fs_net_set_base_url(FSDevice *fs1,
                                      const char *base_url_id,
                                      const char *url,
                                      const char *user, const char *password,
                                      AES_KEY *aes_state)
{
        FSDeviceMem *fs = (FSDeviceMem *)fs1;
    FSBaseURL *bu;
    FSBaseURL *fs_net_set_base_url__part875(FSDevice *fs1, const char *base_url_id,
                                            struct list_head *base_url_list,
                                            void (*fs_mem_end)(FSDevice *));
    bu = fs_net_set_base_url__part875(fs1, base_url_id, &fs->base_url_list, fs_mem_end);

        {
        extern FSBaseURL *fs_net_set_base_url__part876(FSDeviceMem *fs, FSBaseURL *bu,
                                                      const char *base_url_id);
        bu = fs_net_set_base_url__part876(fs, bu, base_url_id);
    }


    char *fs_net_set_base_url__part877(const char *url);
    bu->url = fs_net_set_base_url__part877(url);
        char *fs_net_set_base_url__part878(const char *user);
    bu->user = fs_net_set_base_url__part878(user);

    void fs_net_set_base_url__part879(FSBaseURL *bu, const char *password);
    fs_net_set_base_url__part879(bu, password);
    void fs_net_set_base_url__part880(FSBaseURL *bu, AES_KEY *aes_state);
    fs_net_set_base_url__part880(bu, aes_state);

        FSBaseURL *fs_net_set_base_url__part881(FSBaseURL *bu);
    return fs_net_set_base_url__part881(bu);

}

static int fs_net_reset_base_url(FSDevice *fs1,
                                 const char *base_url_id)
{
    FSBaseURL *bu;
extern FSBaseURL *fs_net_reset_base_url__part883(FSDevice *fs1, const char *base_url_id);

    
   ((void) sizeof (__assert_single_arg (
   fs_is_net(fs1)
   )), __extension__ ({ if (
   fs_is_net(fs1)
   ) ; else __assert_fail (
   "fs_is_net(fs1)"
   , "fs_net.c", 1658, __extension__ __PRETTY_FUNCTION__); }))
                         ;
    bu = fs_net_reset_base_url__part883(fs1, base_url_id);

    int fs_net_reset_base_url__part884(void *bu);
    if (fs_net_reset_base_url__part884(bu))
        return -2;

    int fs_net_reset_base_url__part885(FSDevice *fs1, FSBaseURL *bu);
    return fs_net_reset_base_url__part885(fs1, bu);
}

static void fs_net_set_fs_max_size(FSDevice *fs1, uint64_t fs_max_size)
{
        void fs_net_set_fs_max_size__part887(FSDevice *fs1, uint64_t fs_max_size);

   ((void) sizeof (__assert_single_arg (
   fs_is_net(fs1)
   )), __extension__ ({ if (
   fs_is_net(fs1)
   ) ; else __assert_fail (
   "fs_is_net(fs1)"
   , "fs_net.c", 1670, __extension__ __PRETTY_FUNCTION__); }))
                         ;
    fs_net_set_fs_max_size__part887(fs1, fs_max_size);

}

static int fs_net_set_url(FSDevice *fs1, FSINode *n,
                          const char *base_url_id, FSFileID file_id, uint64_t size)
{
    FSDeviceMem *fs = (FSDeviceMem *)fs1;
    FSBaseURL *bu;

    FSBaseURL *fs_net_set_url__part889(FSDevice *fs1, const char *base_url_id,
                                       void (*fs_mem_end)(FSDevice *));

    bu = fs_net_set_url__part889(fs1, base_url_id, fs_mem_end);

        {
        int fs_net_set_url__part890(const void *bu);
        if (fs_net_set_url__part890(bu))
            return -2;
    }


    {
    int fs_net_set_url__part891_ret;
    extern int fs_net_set_url__part891(FSINode *n);
    fs_net_set_url__part891_ret = fs_net_set_url__part891(n);
    if (fs_net_set_url__part891_ret)
        return fs_net_set_url__part891_ret;
}


    void fs_net_set_url__part892(FSDeviceMem *fs, FSINode *n, FSBaseURL *bu, uint64_t size, FSFileID file_id);
    fs_net_set_url__part892(fs, n, bu, size, file_id);
        int fs_net_set_url__part893(void);
    return fs_net_set_url__part893();
}



typedef enum {
    JSON_STR,
    JSON_INT,
    JSON_OBJ,
    JSON_ARRAY,
    JSON_BOOL,
    JSON_NULL,
    JSON_UNDEFINED,
    JSON_EXCEPTION,
} JSONTypeEnum;

typedef struct {
    int len;
    char data[0];
} JSONString;

typedef struct JSONValue {
    JSONTypeEnum type;
    union {
        JSONString *str;
        int int32;
        BOOL b;
        struct JSONObject *obj;
        struct JSONArray *array;
    } u;
} JSONValue;

typedef struct JSONProperty {
    JSONValue name;
    JSONValue value;
} JSONProperty;

typedef struct JSONObject {
    int len;
    int size;
    JSONProperty *props;
} JSONObject;

typedef struct JSONArray {
    int len;
    int size;
    JSONValue *tab;
} JSONArray;

JSONValue json_string_new2(const char *str, int len);
JSONValue json_string_new(const char *str);
JSONValue __attribute__((format(printf, 1, 2))) json_error_new(const char *fmt, ...);
void json_free(JSONValue val);

JSONValue json_object_new(void);
JSONValue json_object_get(JSONValue val, const char *name);
int json_object_set(JSONValue val, const char *name, JSONValue prop_val);

JSONValue json_array_new(void);
JSONValue json_array_get(JSONValue val, unsigned int idx);
int json_array_set(JSONValue val, unsigned int idx, JSONValue prop_val);

static inline BOOL json_is_error(JSONValue val)
{
    return val.type == JSON_EXCEPTION;
}

static inline BOOL json_is_undefined(JSONValue val)
{
    return val.type == JSON_UNDEFINED;
}

static inline JSONValue json_undefined_new(void)
{
    JSONValue val;
    val.type = JSON_UNDEFINED;
    val.u.int32 = 0;
    return val;
}

static inline JSONValue json_null_new(void)
{
    JSONValue val;
    val.type = JSON_NULL;
    val.u.int32 = 0;
    return val;
}

static inline JSONValue json_int32_new(int v)
{
    JSONValue val;
    val.type = JSON_INT;
    val.u.int32 = v;
    return val;
}

static inline JSONValue json_bool_new(BOOL v)
{
    JSONValue val;
    val.type = JSON_BOOL;
    val.u.b = v;
    return val;
}

const char *json_get_str(JSONValue val);
const char *json_get_error(JSONValue val);

JSONValue json_parse_value(const char *p);
JSONValue json_parse_value_len(const char *p, int len);



static void fs_dump_add_file(struct list_head *head, const char *name)
{
    void fs_dump_add_file__part895(struct list_head *head, const char *name);
    fs_dump_add_file__part895(head, name);
}

static PreloadFile *fs_dump_find_file(struct list_head *head, const char *name)
{
    PreloadFile *pf;
    struct list_head *el;
        PreloadFile *fs_dump_find_file__part897(struct list_head *head, const char *name);
    {
        PreloadFile *found = fs_dump_find_file__part897(head, name);
        if (found != ((void *)0))
            return found;
    }

        {
        PreloadFile *fs_dump_find_file__part898(void);
        return fs_dump_find_file__part898();
    }

}

static void dump_close_archive(FSDevice *fs1)
{
    FSDeviceMem *fs = (FSDeviceMem *)fs1;
    void dump_close_archive__part900(FILE *f);
    dump_close_archive__part900(fs->dump_archive_file);
        void dump_close_archive__part901(FSDeviceMem *fs);
    dump_close_archive__part901(fs);

}

static void dump_loaded_file(FSDevice *fs1, FSINode *n)
{
    FSDeviceMem *fs = (FSDeviceMem *)fs1;
    char filename[1024];
    const char *fname, *p;

    extern int dump_loaded_file__part903(int dump_cache_load, const char *filename);
    if (dump_loaded_file__part903(fs->dump_cache_load, n->u.reg.filename))
        return;

        const char *dump_loaded_file__part904(const FSINode *n);
    fname = dump_loaded_file__part904(n);


    void dump_loaded_file__part905(FSDevice *fs1, const char *fname);
    dump_loaded_file__part905(fs1, fname);
    int dump_loaded_file__part906(int dump_started);
    if (dump_loaded_file__part906(fs->dump_started))
        return;


    void dump_loaded_file__part907(char *filename, size_t filename_size,
                               const char *dump_preload_dir,
                               const char *dump_archive_name,
                               int *dump_archive_num,
                               void **dump_archive_file,
                               void *dump_preload_archive_file,
                               void *dump_preload_file);
    dump_loaded_file__part907(filename, sizeof(filename),
                              fs->dump_preload_dir, fs->dump_archive_name,
                              &fs->dump_archive_num,
                              (void **)&fs->dump_archive_file,
                              fs->dump_preload_archive_file,
                              fs->dump_preload_file);


    void dump_loaded_file__part908(const char *fname, size_t size,
                               const uint8_t *fbuf_data, uint64_t file_id,
                               FILE *dump_preload_file,
                               FILE *dump_preload_archive_file,
                               FILE **dump_archive_file,
                               uint64_t *dump_archive_size);
    dump_loaded_file__part908(fname, n->u.reg.size, n->u.reg.fbuf.data,
                              n->u.reg.file_id, fs->dump_preload_file,
                              fs->dump_preload_archive_file,
                              &fs->dump_archive_file, &fs->dump_archive_size);

}

static JSONValue json_load(const char *filename)
{
    FILE *json_load__part910(const char *filename);
    JSONValue val;
    size_t size;
    char *buf;

    FILE *f = json_load__part910(filename);

        void json_load__part911(void *f, const char *filename);
    json_load__part911(f, filename);

        JSONValue json_load__part912(FILE *f);
    return json_load__part912(f);

}

void fs_dump_cache_load(FSDevice *fs1, const char *cfg_filename)
{
    FSDeviceMem *fs = (FSDeviceMem *)fs1;
    JSONValue cfg, val, array;
    char *fname;
    const char *preload_dir, *name;
    int i;

        int fs_dump_cache_load__part914(FSDevice *fs1, void (*fs_mem_end_fn)(FSDevice *));
    if (!fs_dump_cache_load__part914(fs1, fs_mem_end))
        return;
    JSONValue fs_dump_cache_load__part915(const char *filename);
    cfg = fs_dump_cache_load__part915(cfg_filename);

    {
        int fs_dump_cache_load__part916(JSONValue cfg);
        if (fs_dump_cache_load__part916(cfg))
            exit(1);
    }

    JSONValue fs_dump_cache_load__part917(JSONValue cfg);
    val = fs_dump_cache_load__part917(cfg);
    int fs_dump_cache_load__part918(JSONValue cfg);
    if (fs_dump_cache_load__part918(cfg)) {
    config_error:
        exit(1);
    }

        const char *fs_dump_cache_load__part919(JSONValue val);
    preload_dir = fs_dump_cache_load__part919(val);
    int fs_dump_cache_load__part920(const char *preload_dir);
    if (fs_dump_cache_load__part920(preload_dir)) {
        goto config_error;
    }
    void fs_dump_cache_load__part921(char **dir_out, struct list_head *preload_list,
    struct list_head *exclude_list, const char *preload_dir, JSONValue cfg,
    JSONValue *array_out);
    fs_dump_cache_load__part921(&fs->dump_preload_dir, &fs->dump_preload_list,
                                &fs->dump_exclude_list, preload_dir, cfg, &array);
    int fs_dump_cache_load__part922(int array_type);
    if (fs_dump_cache_load__part922(array.type)) {
        goto config_error;
    }

        {
        extern int fs_dump_cache_load__part923(const JSONValue *array,
                                               unsigned int idx,
                                               struct list_head *head);
        for(i = 0; i < array.u.array->len; i++) {
            if (fs_dump_cache_load__part923(&array, (unsigned int)i,
                                            &fs->dump_preload_list))
                goto config_error;
        }
    }

        char *fs_dump_cache_load__part924(JSONValue cfg, const char *dump_preload_dir, FILE **dump_preload_file);
    fname = fs_dump_cache_load__part924(cfg, fs->dump_preload_dir, &fs->dump_preload_file);
        {
        void fs_dump_cache_load__part925(FILE *fp, const char *fname);
        fs_dump_cache_load__part925(fs->dump_preload_file, fname);
    }

    void fs_dump_cache_load__part926(char **fname, const char *dir, FILE **file);
    fs_dump_cache_load__part926(&fname, fs->dump_preload_dir, &fs->dump_preload_archive_file);

    {
        void fs_dump_cache_load__part927(FILE *_fp, const char *_fname);
        fs_dump_cache_load__part927(fs->dump_preload_archive_file, fname);
    }

    void fs_dump_cache_load__part928(char *fname, int *dump_cache_load);
    fs_dump_cache_load__part928(fname, &fs->dump_cache_load);

}
static int filelist_load_rec(FSDevice *fs1, const char **pp, FSINode *dir,
                             const char *path)
{

    char fname[1024], lname[1024];
    int ret;
    const char *p;
    FSINodeTypeEnum type;
    uint32_t mode, uid, gid;
    uint64_t size;
    FSINode *n;

    const char *filelist_load_rec__part930(const char **pp);
    p = filelist_load_rec__part930(pp);

        {
        extern int filelist_load_rec__part931(FSDevice *fs1, const char **pp,
                                              FSINode *dir, const char *path,
                                              FSINode *(*c_inode_new)(FSDevice *, FSINodeTypeEnum, uint32_t, uint32_t, uint32_t),
                                              FSINode *(*c_inode_incref)(FSDevice *, FSINode *),
                                              FSDirEntry *(*c_inode_dir_add)(FSDevice *, FSINode *, const char *, FSINode *),
                                              int (*c_fs_net_set_url)(FSDevice *, FSINode *, const char *, FSFileID, uint64_t),
                                              int (*c_filelist_load_rec)(FSDevice *, const char **, FSINode *, const char *),
                                              const int *dump_cache_load);

        ret = filelist_load_rec__part931(fs1, &p, dir, path,
                                         inode_new, inode_incref, inode_dir_add,
                                         fs_net_set_url, filelist_load_rec,
                                         &((FSDeviceMem *)fs1)->dump_cache_load);
        if (ret)
            return ret;
    }

    int filelist_load_rec__part932(const char **pp, const char *p);
    return filelist_load_rec__part932(pp, p);
}

static int filelist_load(FSDevice *fs1, const char *str)
{
    FSDeviceMem *fs = (FSDeviceMem *)fs1;
    int ret;
    const char *p;

        {
        int filelist_load__part934(const char *str);
        int part934_ret = filelist_load__part934(str);
        if (part934_ret != 0)
            return part934_ret;
    }

        extern const char *filelist_load__part935(const char *str);
    p = filelist_load__part935(str);
    int filelist_load__part936(const char *p);
    if (filelist_load__part936(p))
        return -1;
    int filelist_load__part937(FSDevice *fs1, const char **pp);
    ret = filelist_load__part937(fs1, &p);
    return ret;
}




static void __attribute__((format(printf, 1, 2))) fatal_error(const char *fmt, ...)
{
    va_list ap;
    
   __builtin_c23_va_start(
   ap, fmt
   )
                    ;
    fprintf(
           stderr
                 , "Error: ");
    vfprintf(
            stderr
                  , fmt, ap);
    fprintf(
           stderr
                 , "\n");
    
   __builtin_va_end(
   ap
   )
             ;
    exit(1);
}

static void fs_create_cmd(FSDevice *fs)
{
    void fs_create_cmd__part940(FSDevice *fs);
    fs_create_cmd__part940(fs);

}

typedef struct {
    FSDevice *fs;
    char *url;
    void (*start_cb)(void *opaque);
    void *start_opaque;

    FSFile *root_fd;
    FSFile *fd;
    int file_index;

} FSNetInitState;

static void fs_initial_sync(FSDevice *fs,
                            const char *url, void (*start_cb)(void *opaque),
                            void *start_opaque);
static void head_loaded(FSDevice *fs, FSFile *f, int64_t size, void *opaque);
static void filelist_loaded(FSDevice *fs, FSFile *f, int64_t size, void *opaque);
static void kernel_load_cb(FSDevice *fs, FSQID *qid, int err,
                           void *opaque);
static int preload_parse(FSDevice *fs, const char *fname, BOOL is_new);







FSDevice *fs_net_init(const char *url, void (*start_cb)(void *opaque),
                      void *start_opaque)
{
    FSDevice *fs_net_init__part944(void);
    FSDevice *fs = fs_net_init__part944();


        {
        void fs_net_init__part945(FSDevice *fs, const char *url,
                                  void (*start_cb)(void *opaque),
                                  void *start_opaque,
                                  void (*initial_sync)(FSDevice *fs,
                                                       const char *url,
                                                       void (*start_cb)(void *opaque),
                                                       void *start_opaque));
        fs_net_init__part945(fs, url, start_cb, start_opaque, fs_initial_sync);
    }

    FSDevice *fs_net_init__part946(FSDevice *fs);
return fs_net_init__part946(fs);

}

static void fs_initial_sync(FSDevice *fs,
                            const char *url, void (*start_cb)(void *opaque),
                            void *start_opaque)
{
        FSNetInitState *s;
    FSFile *head_fd;
    char *head_url;
    void fs_initial_sync__part948(FSDevice *fs, const char *url,
                                  void (*start_cb)(void *opaque),
                                  void *start_opaque,
                                  FSNetInitState **ps,
                                  FSFile **phead_fd,
                                  char **phead_url,
                                  FSWGetFileCB *head_loaded_cb);

    fs_initial_sync__part948(fs, url, start_cb, start_opaque,
                             &s, &head_fd, &head_url, head_loaded);

}

static void head_loaded(FSDevice *fs, FSFile *f, int64_t size, void *opaque)
{
    FSNetInitState *s = opaque;
    char *buf, *root_url, *url;
    char fname[32];
    FSFileID root_id;
    FSFile *new_filelist_fd;
    FSQID qid;
    uint64_t fs_max_size;

    void head_loaded__part950(int64_t size);
    head_loaded__part950(size);

    char *head_loaded__part951(FSDevice *fs, FSFile *f, int64_t size, FSFile *root_fd);
    buf = head_loaded__part951(fs, f, size, s->root_fd);

        void head_loaded__part952(const char *buf);
    head_loaded__part952(buf);


    void head_loaded__part953(FSFileID *root_id, const char *buf);
    head_loaded__part953(&root_id, buf);

    int head_loaded__part954(const char *buf, uint64_t *pfs_max_size);
    if (head_loaded__part954(buf, &fs_max_size) != 0) {
        fs_net_set_fs_max_size(fs, fs_max_size);
    }



        if (!fs_is_net(fs))
        __assert_fail("fs_is_net(fs1)", "fs_net.c", 1622, __PRETTY_FUNCTION__);
    {
        void head_loaded__part955(FSDevice *fs, FSNetInitState *s, FSQID *qid,
                                  char *fname, FSFileID root_id,
                                  FSWGetFileCB *cb);
        head_loaded__part955(fs, s, &qid, fname, root_id, filelist_loaded);
    }

}

static void filelist_loaded(FSDevice *fs, FSFile *f, int64_t size, void *opaque)
{
    FSNetInitState *s = opaque;
    uint8_t *buf;

    void filelist_loaded__part957(int64_t size);
    if (size < 0)
        { filelist_loaded__part957(size); }


    uint8_t *filelist_loaded__part958(FSDevice *fs, FSFile *f, int64_t size, FSNetInitState *s);
    buf = filelist_loaded__part958(fs, f, size, s);

    int filelist_loaded__part959(FSDevice *fs, const char *str);
    if (filelist_loaded__part959(fs, (char *)buf) != 0)
        fatal_error("error while parsing file list");


    void filelist_loaded__part960(FSDevice *fs, FSNetInitState *s);
    filelist_loaded__part960(fs, s);
}




static const char *kernel_file_list[2] = {
    ".preload",
    ".preload2/preload.txt",
};

static void kernel_load_cb(FSDevice *fs, FSQID *qid1, int err,
                           void *opaque)
{
    FSNetInitState *s = opaque;
    FSQID qid;



    int kernel_load_cb__part963(FSDevice *fs);
    if (kernel_load_cb__part963(fs))
        return;



        void kernel_load_cb__part964(FSDevice *fs, FSNetInitState *s);
    if (s->fd) {
        kernel_load_cb__part964(fs, s);
    }


        {
        int (*kernel_load_cb__part965)(FSDevice *fs, FSNetInitState *s,
            void (*open_cb)(FSDevice *, FSQID *, int, void *),
            int (*preload_parse_fn)(FSDevice *, const char *, int),
            const char *const *file_list);
        int recurse;

        (void)qid1;
        (void)qid;
        (void)err;

        recurse = kernel_load_cb__part965(fs, s, kernel_load_cb, preload_parse,
                                          kernel_file_list);
        if (recurse)
            kernel_load_cb(fs,
                              ((void *)0)
                                  , 0, s);
    }
}

static void preload_parse_str_old(FSDevice *fs1, const char *p)
{
    FSDeviceMem *fs = (FSDeviceMem *)fs1;
    char fname[1024];
    PreloadEntry *pe;
    PreloadFile *pf;
    FSINode *n;

        void preload_parse_str_old__part967(FSDevice *fs1, const char *p);
    (void)fs; (void)fname; (void)pe; (void)pf; (void)n;
    preload_parse_str_old__part967(fs1, p);

}

static void preload_parse_str(FSDevice *fs1, const char *p)
{
    FSDeviceMem *fs;
    PreloadEntry *pe;
    PreloadArchive *pa;
    FSINode *n;
    BOOL is_archive;
    char fname[1024];

    FSDeviceMem *preload_parse_str__part969(FSDevice *fs1,
                                           PreloadEntry **pe,
                                           PreloadArchive **pa);

    fs = preload_parse_str__part969(fs1, &pe, &pa);

        void preload_parse_str__part970(FSDeviceMem *fs, const char **pp,
                                   char *fname, int fname_size,
                                   PreloadEntry **ppe, PreloadArchive **ppa,
                                   FSINode **pn, BOOL *pis_archive);
    preload_parse_str__part970(fs, &p, fname, sizeof(fname), &pe, &pa, &n,
                               &is_archive);

}

static int preload_parse(FSDevice *fs, const char *fname, BOOL is_new)
{
    FSINode *preload_parse__part972(FSDevice *fs, const char *fname);
    FSINode *n;
    char *buf;
    size_t size;

    n = preload_parse__part972(fs, fname);

    int preload_parse__part973(const FSINode *n);
    if (preload_parse__part973(n))
        return -1;

        char *preload_parse__part974(void *fbuf, size_t size_in, size_t *size_out);
    buf = preload_parse__part974(&n->u.reg.fbuf, n->u.reg.size, &size);

    {
    void preload_parse__part975(FSDevice *fs, const char *buf, BOOL is_new);
    preload_parse__part975(fs, buf, is_new);
}

    int preload_parse__part976(char *buf);
    return preload_parse__part976(buf);

}






typedef struct CmdXHRState {
    FSFile *req_fd;
    FSFile *root_fd;
    FSFile *fd;
    FSFile *post_fd;
    AES_KEY aes_state;
} CmdXHRState;

static void fs_cmd_xhr_on_load(FSDevice *fs, FSFile *f, int64_t size,
                               void *opaque);

static int parse_hex_buf(uint8_t *buf, int buf_size, const char **pp)
{
    char buf1[1024];
    int len;

    int parse_hex_buf__part980(char *buf1, int buf_size, const char **pp);
    if (parse_hex_buf__part980(buf1, sizeof(buf1), pp) < 0)
        return -1;

    int parse_hex_buf__part981(const char *buf1);
    len = parse_hex_buf__part981(buf1);
    int parse_hex_buf__part982(int len);
    if (parse_hex_buf__part982(len) != 0)
        return -1;

    void parse_hex_buf__part983(int *len);
    parse_hex_buf__part983(&len);
    int parse_hex_buf__part984(int len, int buf_size);
    if (parse_hex_buf__part984(len, buf_size))
        return -1;

    int parse_hex_buf__part985(uint8_t *buf, const char *buf1, int len);
    if (parse_hex_buf__part985(buf, buf1, len) < 0)
        return -1;
        {
        int parse_hex_buf__part986(int len);
        return parse_hex_buf__part986(len);
    }
}

static int fs_cmd_xhr(FSDevice *fs, FSFile *f,
                      const char *p, uint32_t uid, uint32_t gid)
{
    char url[1024], post_filename[1024], filename[1024];
    char user_buf[128], *user;
    char password_buf[128], *password;
    FSQID qid;
    FSFile *fd, *root_fd, *post_fd;
    uint64_t post_data_len;
    int err, aes_key_len;
    CmdXHRState *s;
    char *name;
    AES_KEY *paes_state;
    uint8_t aes_key[16];
    uint32_t flags;
    FSCMDRequest *req;


    int fs_cmd_xhr__part988(FSFile *f);
    if (fs_cmd_xhr__part988(f))
        return -5;


    int fs_cmd_xhr__part989(char *url, const char **p);
    if (fs_cmd_xhr__part989(url, &p) < 0)
        goto fail;

    int fs_cmd_xhr__part990(char *user_buf, int buf_size, const char **pp);
    if (fs_cmd_xhr__part990(user_buf, sizeof(user_buf), &p) < 0)
        goto fail;

    int fs_cmd_xhr__part991(char *password_buf, const char **p);
    if (fs_cmd_xhr__part991(password_buf, &p) < 0)
        goto fail;

    int fs_cmd_xhr__part992(char *buf, int buf_size, const char **pp);
    if (fs_cmd_xhr__part992(post_filename, (int)sizeof(post_filename), &p) < 0)
        goto fail;

    int fs_cmd_xhr__part993(char *filename, int filename_size, const char **pp);
    if (fs_cmd_xhr__part993(filename, sizeof(filename), &p) < 0)
        goto fail;

    {
    int fs_cmd_xhr__part994(const char **pp, uint8_t *buf, int buf_size);
    aes_key_len = fs_cmd_xhr__part994(&p, aes_key, 16);
}
    int fs_cmd_xhr__part995(int aes_key_len);
    if (fs_cmd_xhr__part995(aes_key_len))
        goto fail;

    int fs_cmd_xhr__part996(uint32_t *flags, const char **p);
    if (fs_cmd_xhr__part996(&flags, &p) < 0)
        goto fail;

    int fs_cmd_xhr__part997(int aes_key_len);
    if (fs_cmd_xhr__part997(aes_key_len))
        goto fail;


        char *fs_cmd_xhr__part998(char *user_buf);
    user = fs_cmd_xhr__part998(user_buf);

    char *fs_cmd_xhr__part999(char *password_buf);
    password = fs_cmd_xhr__part999(password_buf);


    
       {
        int (*fs_cmd_xhr__part1000_ptr)(FSDevice *, uint32_t, const char *, FSFile **,
                                        FSFile **, FSFile **, FSQID *, char **);
        extern int fs_cmd_xhr__part1000(FSDevice *fs, uint32_t uid,
                                        const char *filename, FSFile **pfd,
                                        FSFile **proot_fd, FSFile **ppost_fd,
                                        FSQID *pqid, char **pname);
        (void)fs_cmd_xhr__part1000_ptr;
        (void)fs_cmd_xhr__part1000(fs, uid, filename, &fd, &root_fd, &post_fd,
                                   &qid, &name);
    }

    int fs_cmd_xhr__part1001(FSFile *fd);
    if (fs_cmd_xhr__part1001(fd)) {
        err = -2;
        goto fail1;
    }


    extern int fs_cmd_xhr__part1002(FSDevice *fs, FSFile *fd, const char *name, FSQID *qid, uint32_t gid);

    err = fs_cmd_xhr__part1002(fs, fd, name, &qid, gid);
        int fs_cmd_xhr__part1003(int err);
    if (fs_cmd_xhr__part1003(err)) {
        goto fail1;
    }

        {
        int fs_cmd_xhr__part1004(FSDevice *fs, FSFile *root_fd,
                                 const char *post_filename, FSQID *qid,
                                 FSFile **p_post_fd, uint64_t *p_post_data_len,
                                 int *p_err);
        int part1004_r;
        part1004_r = fs_cmd_xhr__part1004(fs, root_fd, post_filename, &qid,
                                         &post_fd, &post_data_len, &err);
        if (part1004_r)
            goto fail1;
    }


        CmdXHRState *fs_cmd_xhr__part1005(FSFile *rf, FSFile *fdp, FSFile *pf);
    s = fs_cmd_xhr__part1005(root_fd, fd, post_fd);

        {
        AES_KEY *fs_cmd_xhr__part1006(int aes_key_len, const uint8_t *aes_key, CmdXHRState *s);
        paes_state = fs_cmd_xhr__part1006(aes_key_len, aes_key, s);
    }


        {
        int fs_cmd_xhr__part1007(FSDevice *fs, FSFile *f, CmdXHRState *s,
                                 FSFile *fd, const char *url, const char *user,
                                 const char *password, FSFile *post_fd,
                                 uint64_t post_data_len, FSWGetFileCB *cb,
                                 AES_KEY *paes_state, FSCMDRequest **preq);
        return fs_cmd_xhr__part1007(fs, f, s, fd, url, user, password, post_fd,
                                    post_data_len, fs_cmd_xhr_on_load,
                                    paes_state, &req);
    }

  fail1:
    void fs_cmd_xhr__part1008(FSDevice *fs, FSFile *fd);
    fs_cmd_xhr__part1008(fs, fd);

        void fs_cmd_xhr__part1009(FSDevice *fs, FSFile *post_fd);
    if (post_fd)
        fs_cmd_xhr__part1009(fs, post_fd);
    int fs_cmd_xhr__part1010(FSDevice *fs, FSFile *root_fd, int err);
    return fs_cmd_xhr__part1010(fs, root_fd, err);
 int fs_cmd_xhr__part1011(void);
fail:
    return fs_cmd_xhr__part1011();

}

static void fs_cmd_xhr_on_load(FSDevice *fs, FSFile *f, int64_t size,
                               void *opaque)
{
    CmdXHRState *s = opaque;
    FSCMDRequest *req;
    int ret;



    void fs_cmd_xhr_on_load__part1013(FSDevice *fs, FSFile *fd);
    if (s->fd)
        fs_cmd_xhr_on_load__part1013(fs, s->fd);

    void fs_cmd_xhr_on_load__part1014(FSDevice *fs, CmdXHRState *s);
    fs_cmd_xhr_on_load__part1014(fs, s);

    void fs_cmd_xhr_on_load__part1015(FSDevice *fs, FSFile *root_fd);
    fs_cmd_xhr_on_load__part1015(fs, s->root_fd);

        {
        void fs_cmd_xhr_on_load__part1016(CmdXHRState *s, int64_t size);
        fs_cmd_xhr_on_load__part1016(s, size);
        (void)req;
        (void)ret;
    }
    void fs_cmd_xhr_on_load__part1017(void *s);
    fs_cmd_xhr_on_load__part1017(s);

}

static int fs_cmd_set_base_url(FSDevice *fs, const char *p)
{

    char url[1024], base_url_id[1024];
    char user_buf[128], *user;
    char password_buf[128], *password;
    AES_KEY aes_state, *paes_state;
    uint8_t aes_key[16];
    int aes_key_len;

    int fs_cmd_set_base_url__part1019(char *buf, int buf_size, const char **pp);
    if (fs_cmd_set_base_url__part1019(base_url_id, (int)sizeof(base_url_id), &p) < 0)
        goto fail;

    int fs_cmd_set_base_url__part1020(char *url, const char **p);
    if (fs_cmd_set_base_url__part1020(url, &p) < 0)
        goto fail;
    int fs_cmd_set_base_url__part1021(char *user_buf, int user_buf_size, const char **pp);
    if (fs_cmd_set_base_url__part1021(user_buf, (int)sizeof(user_buf), &p) < 0)
        goto fail;

    int fs_cmd_set_base_url__part1022(char *password_buf, int buf_size, const char **pp);
    if (fs_cmd_set_base_url__part1022(password_buf, sizeof(password_buf), &p) < 0)
        goto fail;
    int fs_cmd_set_base_url__part1023(uint8_t *aes_key, const char **p);
    aes_key_len = fs_cmd_set_base_url__part1023(aes_key, (const char **)&p);
        {
        int fs_cmd_set_base_url__part1024(int aes_key_len);
        if (fs_cmd_set_base_url__part1024(aes_key_len))
            goto fail;
    }


    char *fs_cmd_set_base_url__part1025(char *user_buf);
    user = fs_cmd_set_base_url__part1025(user_buf);

    char *fs_cmd_set_base_url__part1026(char password_buf[128]);
    password = fs_cmd_set_base_url__part1026(password_buf);

    int fs_cmd_set_base_url__part1027(int aes_key_len, uint8_t *aes_key, AES_KEY *aes_state, AES_KEY **paes_state);
    if (fs_cmd_set_base_url__part1027(aes_key_len, aes_key, &aes_state, &paes_state))
        goto fail;


        void fs_cmd_set_base_url__part1028(FSDevice *fs1, const char *base_url_id,
                                      const char *url, const char *user,
                                      const char *password, AES_KEY *aes_state);
    fs_cmd_set_base_url__part1028(fs, base_url_id, url, user, password,
                                 paes_state);
    return 0;

 fail:
    {
        int fs_cmd_set_base_url__part1029(void);
        return fs_cmd_set_base_url__part1029();
    }
}

static int fs_cmd_reset_base_url(FSDevice *fs, const char *p)
{
    char base_url_id[1024];

        int fs_cmd_reset_base_url__part1031(char *base_url_id, int buf_size, const char **pp);
    if (fs_cmd_reset_base_url__part1031(base_url_id, (int)sizeof(base_url_id), &p) < 0)
        goto fail;

        int fs_cmd_reset_base_url__part1032(FSDevice *fs, const char *base_url_id);
    return fs_cmd_reset_base_url__part1032(fs, base_url_id);

 int fs_cmd_reset_base_url__part1033(void);
 fail:
    return fs_cmd_reset_base_url__part1033();
}

static int fs_cmd_set_url(FSDevice *fs, const char *p)
{
    char base_url_id[1024];
    char filename[1024];
    FSFileID file_id;
    uint64_t size;
    FSINode *n;

        {
        int fs_cmd_set_url__part1035(char *filename, int filename_size, const char **pp);
        if (fs_cmd_set_url__part1035(filename, sizeof(filename), &p) < 0)
            goto fail;
    }

    int fs_cmd_set_url__part1036(char *base_url_id, int base_url_id_size, const char **p);
    if (fs_cmd_set_url__part1036(base_url_id, sizeof(base_url_id), &p) < 0)
        goto fail;
        int fs_cmd_set_url__part1037(FSFileID *file_id, const char **p);
    if (fs_cmd_set_url__part1037(&file_id, &p) < 0)
        goto fail;

        {
        int fs_cmd_set_url__part1038(uint64_t *pval, const char **pp);
        if (fs_cmd_set_url__part1038(&size, &p) < 0)
            goto fail;
    }


    FSINode *fs_cmd_set_url__part1039(FSDevice *fs, const char *path);
    n = fs_cmd_set_url__part1039(fs, filename);
    int fs_cmd_set_url__part1040(void *n);
    if (fs_cmd_set_url__part1040((void *)n)) {
        return -2;
    }

    int fs_cmd_set_url__part1041(FSDevice *fs, FSINode *n, const char *base_url_id, FSFileID file_id, uint64_t size);
    return fs_cmd_set_url__part1041(fs, n, base_url_id, file_id, size);
     int fs_cmd_set_url__part1042(void);
 fail:
    return fs_cmd_set_url__part1042();

}

static int fs_cmd_export_file(FSDevice *fs, const char *p)
{
    char filename[1024];
    FSINode *n;
    const char *name;
    uint8_t *buf;

    int fs_cmd_export_file__part1044(char *filename, int buf_size, const char **pp);
    if (fs_cmd_export_file__part1044(filename, sizeof(filename), &p) < 0)
        goto fail;

    FSINode *fs_cmd_export_file__part1045(void *fs1, const char *path);
    n = fs_cmd_export_file__part1045(fs, filename);

        int fs_cmd_export_file__part1046(const void *n);
    if (fs_cmd_export_file__part1046(n))
        return -2;

    {
        int fs_cmd_export_file__part1047(FSINode *n);
        if (fs_cmd_export_file__part1047(n))
            goto fail;
    }
    const char *fs_cmd_export_file__part1048(const char *filename);
    name = fs_cmd_export_file__part1048(filename);

        {
        const char *(*fs_cmd_export_file__part1049)(const char *name, const char *filename);
        name = fs_cmd_export_file__part1049(name, filename);
    }


    int fs_cmd_export_file__part1050(FSINode *n, const char *name);
    return fs_cmd_export_file__part1050(n, name);

 int fs_cmd_export_file__part1051(void);
 fail:
    return fs_cmd_export_file__part1051();

}


static int fs_cmd_pbkdf2(FSDevice *fs, FSFile *f, const char *p)
{
    uint8_t pwd[1024];
    uint8_t salt[128];
    uint32_t iter, key_len;
    int pwd_len, salt_len;
    FSCMDRequest *req;


    int fs_cmd_pbkdf2__part1053(FSFile *f);
    if (fs_cmd_pbkdf2__part1053(f) != 0)
        return -5;

    int fs_cmd_pbkdf2__part1054(uint8_t *pwd, const char **pp);
pwd_len = fs_cmd_pbkdf2__part1054(pwd, &p);
    int fs_cmd_pbkdf2__part1055(int pwd_len);
    if (fs_cmd_pbkdf2__part1055(pwd_len))
        goto fail;

        {
        int fs_cmd_pbkdf2__part1056(uint8_t *buf, int buf_size, const char **pp);
        salt_len = fs_cmd_pbkdf2__part1056(salt, sizeof(salt), &p);
    }

    int fs_cmd_pbkdf2__part1057(int pwd_len);
    if (fs_cmd_pbkdf2__part1057(pwd_len))
        goto fail;
    {
    int fs_cmd_pbkdf2__part1058(uint32_t *iter, const char **pp);
    if (fs_cmd_pbkdf2__part1058(&iter, &p) < 0)
        goto fail;
}

    int fs_cmd_pbkdf2__part1059(uint32_t *key_len, const char **p);
    if (fs_cmd_pbkdf2__part1059(&key_len, &p) < 0)
        goto fail;
    int fs_cmd_pbkdf2__part1060(uint32_t key_len);
    if (fs_cmd_pbkdf2__part1060(key_len))
        goto fail;
    int fs_cmd_pbkdf2__part1061(FSFile *f, const uint8_t *pwd, int pwd_len,
                                const uint8_t *salt, int salt_len,
                                uint32_t iter, uint32_t key_len);
    return fs_cmd_pbkdf2__part1061(f, pwd, pwd_len, salt, salt_len, iter, key_len);

 int fs_cmd_pbkdf2__part1062(void);
fail:
    return fs_cmd_pbkdf2__part1062();

}

static int fs_cmd_set_import_dir(FSDevice *fs, FSFile *f, const char *p)
{
    FSDeviceMem *fs1 = (FSDeviceMem *)fs;
    char filename[1024];

    int fs_cmd_set_import_dir__part1064(char *filename, int buf_size, const char **pp);
    if (fs_cmd_set_import_dir__part1064(filename, (int)sizeof(filename), &p) < 0)
        return -22;
    int fs_cmd_set_import_dir__part1065(char **import_dir, const char *filename);
    return fs_cmd_set_import_dir__part1065(&fs1->import_dir, filename);
}

static int fs_cmd_write(FSDevice *fs, FSFile *f, uint64_t offset,
                        const uint8_t *buf, int buf_len)
{
    char *buf1;
    const char *p;
    char cmd[64];
    int err;


    char *fs_cmd_write__part1067(const uint8_t *buf, int buf_len,
                                 const char **p, int *err);
    buf1 = fs_cmd_write__part1067(buf, buf_len, &p, &err);

        int fs_cmd_write__part1068(char *cmd, int buf_size, const char **pp);
    if (fs_cmd_write__part1068(cmd, (int)sizeof(cmd), &p) < 0)
        goto fail;
        int fs_cmd_write__part1069(FSDevice *fs, FSFile *f, const char *cmd,
                               const char *p,
                               int (*cmd_xhr)(FSDevice *, FSFile *,
                                              const char *, uint32_t, uint32_t),
                               int (*cmd_set_base_url)(FSDevice *, const char *),
                               int (*cmd_reset_base_url)(FSDevice *, const char *),
                               int (*cmd_set_url)(FSDevice *, const char *),
                               int (*cmd_export_file)(FSDevice *, const char *),
                               int (*cmd_pbkdf2)(FSDevice *, FSFile *,
                                                 const char *),
                               int (*cmd_set_import_dir)(FSDevice *, FSFile *,
                                                         const char *));
    err = fs_cmd_write__part1069(fs, f, cmd, p,
                                 fs_cmd_xhr,
                                 fs_cmd_set_base_url,
                                 fs_cmd_reset_base_url,
                                 fs_cmd_set_url,
                                 fs_cmd_export_file,
                                 fs_cmd_pbkdf2,
                                 fs_cmd_set_import_dir);
    goto fs_cmd_write_done;
fail:
    err = -5;
fs_cmd_write_done:
    ;

    void fs_cmd_write__part1070(char *buf1);
    fs_cmd_write__part1070(buf1);

    int fs_cmd_write__part1071(int err, int buf_len);
    return fs_cmd_write__part1071(err, buf_len);
}

static int fs_cmd_read(FSDevice *fs, FSFile *f, uint64_t offset,
                       uint8_t *buf, int buf_len)
{
        FSCMDRequest *req;
    int l;

    extern FSCMDRequest *fs_cmd_read__part1073(FSFile *f);
    req = fs_cmd_read__part1073(f);

    int fs_cmd_read__part1074(FSCMDRequest *req);
{
    int _fs_ret = fs_cmd_read__part1074(req);
    if (_fs_ret)
        return _fs_ret;
}

    int fs_cmd_read__part1075(const FSCMDRequest *req, uint8_t *buf, int buf_len);
    l = fs_cmd_read__part1075(req, buf, buf_len);
    return l;

}

static void fs_cmd_close(FSDevice *fs, FSFile *f)
{
    FSCMDRequest *fs_cmd_close__part1077(FSFile *f);
    FSCMDRequest *req;
    req = fs_cmd_close__part1077(f);


    void fs_cmd_close__part1078(FSCMDRequest *req, FSFile *f);
    fs_cmd_close__part1078(req, f);
}



void fs_net_set_pwd(FSDevice *fs, const char *pwd)
{
    void fs_net_set_pwd__part1080(FSDevice *fs, const char *pwd, FSFile **proot_fd,
                             FSQID *pqid, void (*fs_mem_end_p)(FSDevice *));
FSFile *root_fd;
    FSQID qid;
    fs_net_set_pwd__part1080(fs, pwd, &root_fd, &qid, fs_mem_end);

}
void fs_export_file(const char *filename,
                    const uint8_t *buf, int buf_len)
{
}
