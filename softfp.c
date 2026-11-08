





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




extern void __assert_fail (const char *__assertion, const char *__file,
      unsigned int __line, const char *__function)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__noreturn__)) __attribute__ ((__cold__));


extern void __assert_perror_fail (int __errnum, const char *__file,
      unsigned int __line, const char *__function)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__noreturn__)) __attribute__ ((__cold__));




extern void __assert (const char *__assertion, const char *__file, int __line)
     __attribute__ ((__nothrow__ , __leaf__)) __attribute__ ((__noreturn__)) __attribute__ ((__cold__));






extern _Bool __assert_single_arg (_Bool);





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
typedef enum {
    RM_RNE,
    RM_RTZ,
    RM_RDN,
    RM_RUP,
    RM_RMM,
} RoundingModeEnum;
typedef uint32_t sfloat32;
typedef uint64_t sfloat64;

typedef uint128_t sfloat128;






sfloat32 add_sf32(sfloat32 a, sfloat32 b, RoundingModeEnum rm, uint32_t *pfflags);
sfloat32 sub_sf32(sfloat32 a, sfloat32 b, RoundingModeEnum rm, uint32_t *pfflags);
sfloat32 mul_sf32(sfloat32 a, sfloat32 b, RoundingModeEnum rm, uint32_t *pfflags);
sfloat32 div_sf32(sfloat32 a, sfloat32 b, RoundingModeEnum rm, uint32_t *pfflags);
sfloat32 sqrt_sf32(sfloat32 a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat32 fma_sf32(sfloat32 a, sfloat32 b, sfloat32 c, RoundingModeEnum rm, uint32_t *pfflags);

sfloat32 min_sf32(sfloat32 a, sfloat32 b, uint32_t *pfflags);
sfloat32 max_sf32(sfloat32 a, sfloat32 b, uint32_t *pfflags);
int eq_quiet_sf32(sfloat32 a, sfloat32 b, uint32_t *pfflags);
int le_sf32(sfloat32 a, sfloat32 b, uint32_t *pfflags);
int lt_sf32(sfloat32 a, sfloat32 b, uint32_t *pfflags);
uint32_t fclass_sf32(sfloat32 a);

sfloat64 cvt_sf32_sf64(sfloat32 a, uint32_t *pfflags);
sfloat32 cvt_sf64_sf32(sfloat64 a, RoundingModeEnum rm, uint32_t *pfflags);
int32_t cvt_sf32_i32(sfloat32 a, RoundingModeEnum rm, uint32_t *pfflags);
uint32_t cvt_sf32_u32(sfloat32 a, RoundingModeEnum rm, uint32_t *pfflags);
int64_t cvt_sf32_i64(sfloat32 a, RoundingModeEnum rm, uint32_t *pfflags);
uint64_t cvt_sf32_u64(sfloat32 a, RoundingModeEnum rm, uint32_t *pfflags);

int128_t cvt_sf32_i128(sfloat32 a, RoundingModeEnum rm, uint32_t *pfflags);
uint128_t cvt_sf32_u128(sfloat32 a, RoundingModeEnum rm, uint32_t *pfflags);

sfloat32 cvt_i32_sf32(int32_t a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat32 cvt_u32_sf32(uint32_t a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat32 cvt_i64_sf32(int64_t a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat32 cvt_u64_sf32(uint64_t a, RoundingModeEnum rm, uint32_t *pfflags);

sfloat32 cvt_i128_sf32(int128_t a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat32 cvt_u128_sf32(uint128_t a, RoundingModeEnum rm, uint32_t *pfflags);






sfloat64 add_sf64(sfloat64 a, sfloat64 b, RoundingModeEnum rm, uint32_t *pfflags);
sfloat64 sub_sf64(sfloat64 a, sfloat64 b, RoundingModeEnum rm, uint32_t *pfflags);
sfloat64 mul_sf64(sfloat64 a, sfloat64 b, RoundingModeEnum rm, uint32_t *pfflags);
sfloat64 div_sf64(sfloat64 a, sfloat64 b, RoundingModeEnum rm, uint32_t *pfflags);
sfloat64 sqrt_sf64(sfloat64 a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat64 fma_sf64(sfloat64 a, sfloat64 b, sfloat64 c, RoundingModeEnum rm, uint32_t *pfflags);

sfloat64 min_sf64(sfloat64 a, sfloat64 b, uint32_t *pfflags);
sfloat64 max_sf64(sfloat64 a, sfloat64 b, uint32_t *pfflags);
int eq_quiet_sf64(sfloat64 a, sfloat64 b, uint32_t *pfflags);
int le_sf64(sfloat64 a, sfloat64 b, uint32_t *pfflags);
int lt_sf64(sfloat64 a, sfloat64 b, uint32_t *pfflags);
uint32_t fclass_sf64(sfloat64 a);

sfloat64 cvt_sf32_sf64(sfloat32 a, uint32_t *pfflags);
sfloat32 cvt_sf64_sf32(sfloat64 a, RoundingModeEnum rm, uint32_t *pfflags);
int32_t cvt_sf64_i32(sfloat64 a, RoundingModeEnum rm, uint32_t *pfflags);
uint32_t cvt_sf64_u32(sfloat64 a, RoundingModeEnum rm, uint32_t *pfflags);
int64_t cvt_sf64_i64(sfloat64 a, RoundingModeEnum rm, uint32_t *pfflags);
uint64_t cvt_sf64_u64(sfloat64 a, RoundingModeEnum rm, uint32_t *pfflags);

int128_t cvt_sf64_i128(sfloat64 a, RoundingModeEnum rm, uint32_t *pfflags);
uint128_t cvt_sf64_u128(sfloat64 a, RoundingModeEnum rm, uint32_t *pfflags);

sfloat64 cvt_i32_sf64(int32_t a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat64 cvt_u32_sf64(uint32_t a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat64 cvt_i64_sf64(int64_t a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat64 cvt_u64_sf64(uint64_t a, RoundingModeEnum rm, uint32_t *pfflags);

sfloat64 cvt_i128_sf64(int128_t a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat64 cvt_u128_sf64(uint128_t a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat128 add_sf128(sfloat128 a, sfloat128 b, RoundingModeEnum rm, uint32_t *pfflags);
sfloat128 sub_sf128(sfloat128 a, sfloat128 b, RoundingModeEnum rm, uint32_t *pfflags);
sfloat128 mul_sf128(sfloat128 a, sfloat128 b, RoundingModeEnum rm, uint32_t *pfflags);
sfloat128 div_sf128(sfloat128 a, sfloat128 b, RoundingModeEnum rm, uint32_t *pfflags);
sfloat128 sqrt_sf128(sfloat128 a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat128 fma_sf128(sfloat128 a, sfloat128 b, sfloat128 c, RoundingModeEnum rm, uint32_t *pfflags);

sfloat128 min_sf128(sfloat128 a, sfloat128 b, uint32_t *pfflags);
sfloat128 max_sf128(sfloat128 a, sfloat128 b, uint32_t *pfflags);
int eq_quiet_sf128(sfloat128 a, sfloat128 b, uint32_t *pfflags);
int le_sf128(sfloat128 a, sfloat128 b, uint32_t *pfflags);
int lt_sf128(sfloat128 a, sfloat128 b, uint32_t *pfflags);
uint32_t fclass_sf128(sfloat128 a);

sfloat128 cvt_sf32_sf128(sfloat32 a, uint32_t *pfflags);
sfloat32 cvt_sf128_sf32(sfloat128 a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat128 cvt_sf64_sf128(sfloat64 a, uint32_t *pfflags);
sfloat64 cvt_sf128_sf64(sfloat128 a, RoundingModeEnum rm, uint32_t *pfflags);

int32_t cvt_sf128_i32(sfloat128 a, RoundingModeEnum rm, uint32_t *pfflags);
uint32_t cvt_sf128_u32(sfloat128 a, RoundingModeEnum rm, uint32_t *pfflags);
int64_t cvt_sf128_i64(sfloat128 a, RoundingModeEnum rm, uint32_t *pfflags);
uint64_t cvt_sf128_u64(sfloat128 a, RoundingModeEnum rm, uint32_t *pfflags);
int128_t cvt_sf128_i128(sfloat128 a, RoundingModeEnum rm, uint32_t *pfflags);
uint128_t cvt_sf128_u128(sfloat128 a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat128 cvt_i32_sf128(int32_t a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat128 cvt_u32_sf128(uint32_t a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat128 cvt_i64_sf128(int64_t a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat128 cvt_u64_sf128(uint64_t a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat128 cvt_i128_sf128(int128_t a, RoundingModeEnum rm, uint32_t *pfflags);
sfloat128 cvt_u128_sf128(uint128_t a, RoundingModeEnum rm, uint32_t *pfflags);

static inline int clz32(uint32_t a)
{
    int r;
    int clz32__part2916(uint32_t a);
    r = clz32__part2916(a);
    extern int clz32__part2917(int r);
return clz32__part2917(r);

}

static inline int clz64(uint64_t a)
{
    int r;
    int clz64__part2919(uint64_t a);
    r = clz64__part2919(a);

    int clz64__part2920(int r);
    return clz64__part2920(r);

}


static inline int clz128(uint128_t a)
{
    int r;
    int clz128__part2922(uint128_t a);
    r = clz128__part2922(a);

    int clz128__part2923(int r);
    return clz128__part2923(r);

}



static const uint32_t F_QNAN32 = (((uint32_t)((1 << 8) - 1) << 23) | ((uint32_t)1 << (23 - 1)));

static inline uint32_t pack_sf32(uint32_t a_sign, uint32_t a_exp, uint32_t a_mant)
{
        uint32_t pack_sf32__part2927(uint32_t a_sign, uint32_t a_exp, uint32_t a_mant);
    return pack_sf32__part2927(a_sign, a_exp, a_mant);

}

static inline uint32_t unpack_sf32(uint32_t *pa_sign, int32_t *pa_exp,
                               uint32_t a)
{
    {
    uint32_t unpack_sf32__part2929(uint32_t *pa_sign, int32_t *pa_exp, uint32_t a);
    return unpack_sf32__part2929(pa_sign, pa_exp, a);
}

}

static uint32_t rshift_rnd32(uint32_t a, int d)
{
    uint32_t mask;
    unsigned int rshift_rnd32__part2931(unsigned int a, int d);
        a = rshift_rnd32__part2931(a, d);
    uint32_t rshift_rnd32__part2932(uint32_t a);
    return rshift_rnd32__part2932(a);
}


static uint32_t roundpack_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
                            RoundingModeEnum rm, uint32_t *pfflags)
{
    int diff;
    uint32_t addend, rnd_bits;

        {
        uint32_t roundpack_sf32__part2934(uint32_t a_sign, RoundingModeEnum rm);
        addend = roundpack_sf32__part2934(a_sign, rm);
    }



    void roundpack_sf32__part2935(int *a_exp, uint32_t *a_mant, uint32_t addend,
                              int *diff, uint32_t *rnd_bits, uint32_t *pfflags);
    roundpack_sf32__part2935(&a_exp, &a_mant, addend, &diff, &rnd_bits, pfflags);

    extern void roundpack_sf32__part2936(uint32_t rnd_bits, uint32_t *pfflags);
    roundpack_sf32__part2936(rnd_bits, pfflags);

        uint32_t roundpack_sf32__part2937(uint32_t a_mant, uint32_t addend);
    a_mant = roundpack_sf32__part2937(a_mant, addend);


    uint32_t roundpack_sf32__part2938(int rm, uint32_t rnd_bits, uint32_t a_mant);
    a_mant = roundpack_sf32__part2938(rm, rnd_bits, a_mant);



        extern int roundpack_sf32__part2939(int a_exp, uint32_t a_mant);
    a_exp = roundpack_sf32__part2939(a_exp, a_mant);
        void roundpack_sf32__part2940(int *a_exp, uint32_t *a_mant, uint32_t addend, uint32_t *pfflags);
    roundpack_sf32__part2940(&a_exp, &a_mant, addend, pfflags);

        uint32_t roundpack_sf32__part2941(uint32_t a_sign, int a_exp, uint32_t a_mant);
    return roundpack_sf32__part2941(a_sign, a_exp, a_mant);
}


static uint32_t normalize_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant,
                           RoundingModeEnum rm, uint32_t *pfflags)
{
    uint32_t normalize_sf32__part2943(uint32_t a_sign, int a_exp, uint32_t a_mant, RoundingModeEnum rm, uint32_t *pfflags);
    return normalize_sf32__part2943(a_sign, a_exp, a_mant, rm, pfflags);

}



static uint32_t normalize2_sf32(uint32_t a_sign, int a_exp, uint32_t a_mant1, uint32_t a_mant0,
                            RoundingModeEnum rm, uint32_t *pfflags)
{
    int l, shift;
    int normalize2_sf32__part2945(uint32_t a_mant1, uint32_t a_mant0);
    l = normalize2_sf32__part2945(a_mant1, a_mant0);

    int normalize2_sf32__part2946(int l, int a_exp);
    shift = l - (32 - 1 - (32 - 2));
    a_exp = normalize2_sf32__part2946(l, a_exp);

    uint32_t normalize2_sf32__part2947(uint32_t a_mant1, uint32_t a_mant0, int shift);
    a_mant1 = normalize2_sf32__part2947(a_mant1, a_mant0, shift);

    uint32_t normalize2_sf32__part2948(uint32_t a_sign, int a_exp, uint32_t a_mant1, RoundingModeEnum rm, uint32_t *pfflags);
    return normalize2_sf32__part2948(a_sign, a_exp, a_mant1, rm, pfflags);
}

BOOL issignan_sf32(uint32_t a)
{
        int issignan_sf32__part2950(uint32_t a);
    return issignan_sf32__part2950(a);

}

BOOL isnan_sf32(uint32_t a)
{
        BOOL isnan_sf32__part2952(uint32_t a);
    return isnan_sf32__part2952(a);

}


uint32_t add_sf32(uint32_t a, uint32_t b, RoundingModeEnum rm,
              uint32_t *pfflags)
{
    uint32_t a_sign, b_sign, a_exp, b_exp;
    uint32_t tmp, a_mant, b_mant;


    void add_sf32__part2954(uint32_t *a, uint32_t *b);
    add_sf32__part2954(&a, &b);

    void add_sf32__part2955(uint32_t a, uint32_t b, uint32_t *a_sign, uint32_t *b_sign, uint32_t *a_exp, uint32_t *b_exp, uint32_t *a_mant, uint32_t *b_mant);
    add_sf32__part2955(a, b, &a_sign, &b_sign, &a_exp, &b_exp, &a_mant, &b_mant);
    {
        uint32_t add_sf32__part2956(uint32_t a_exp, uint32_t a_mant, uint32_t b, uint32_t b_exp, uint32_t a_sign, uint32_t a, uint32_t *pfflags, int *ptaken);
        int add_sf32__part2956_taken;
        uint32_t add_sf32__part2956_ret;
        add_sf32__part2956_ret = add_sf32__part2956(a_exp, a_mant, b, b_exp, a_sign, a, pfflags, &add_sf32__part2956_taken);
        if (add_sf32__part2956_taken)
            return add_sf32__part2956_ret;
    }
    void add_sf32__part2957(uint32_t *a_exp, uint32_t *a_mant);
    add_sf32__part2957(&a_exp, &a_mant);

    void add_sf32__part2958(uint32_t *b_exp, uint32_t *b_mant);
    add_sf32__part2958(&b_exp, &b_mant);

    uint32_t add_sf32__part2959(uint32_t b_mant, uint32_t a_exp, uint32_t b_exp);
    b_mant = add_sf32__part2959(b_mant, a_exp, b_exp);

    uint32_t add_sf32__part2960(uint32_t a_sign, uint32_t b_sign, uint32_t a_mant, uint32_t b_mant, uint32_t rm, uint32_t *p_a_sign);
    a_mant = add_sf32__part2960(a_sign, b_sign, a_mant, b_mant, (uint32_t)rm, &a_sign);
        uint32_t add_sf32__part2961(uint32_t a_sign, uint32_t a_exp, uint32_t a_mant, unsigned int rm, uint32_t *pfflags);
    a_exp += (((32 - 2) - 23) - 3);
    return add_sf32__part2961(a_sign, a_exp, a_mant, rm, pfflags);
}

uint32_t sub_sf32(uint32_t a, uint32_t b, RoundingModeEnum rm,
                            uint32_t *pfflags)
{
    uint32_t sub_sf32__part2963(uint32_t a, uint32_t b, RoundingModeEnum rm, uint32_t *pfflags);
    return sub_sf32__part2963(a, b, rm, pfflags);

}

static inline uint32_t normalize_subnormal_sf32(int32_t *pa_exp, uint32_t a_mant)
{
    uint32_t normalize_subnormal_sf32__part2965(int32_t *pa_exp, uint32_t a_mant);
    return normalize_subnormal_sf32__part2965(pa_exp, a_mant);
}



static uint32_t mul_u32(uint32_t *plow, uint32_t a, uint32_t b)
{
    uint32_t mul_u32__part2967(uint32_t *plow, uint32_t a, uint32_t b);
    return mul_u32__part2967(plow, a, b);
}
uint32_t mul_sf32(uint32_t a, uint32_t b, RoundingModeEnum rm,
              uint32_t *pfflags)
{
    uint32_t a_sign, b_sign, r_sign;
    int32_t a_exp, b_exp, r_exp;
    uint32_t a_mant, b_mant, r_mant, r_mant_low;
    void mul_sf32__part2969(uint32_t a, uint32_t b, uint32_t *a_sign, uint32_t *b_sign, uint32_t *r_sign, int32_t *a_exp, int32_t *b_exp, uint32_t *a_mant, uint32_t *b_mant);
    mul_sf32__part2969(a, b, &a_sign, &b_sign, &r_sign, &a_exp, &b_exp, &a_mant, &b_mant);

        {
        int mul_sf32__part2970(uint32_t a, uint32_t b, int32_t a_exp, int32_t b_exp, uint32_t a_mant, uint32_t b_mant, uint32_t r_sign, uint32_t *pfflags, uint32_t *ret);
        uint32_t mul_sf32__part2970_ret;
        if (mul_sf32__part2970(a, b, a_exp, b_exp, a_mant, b_mant, r_sign, pfflags, &mul_sf32__part2970_ret))
            return mul_sf32__part2970_ret;
    }

    {
        int mul_sf32__part2971(int32_t *pa_exp, uint32_t *pa_mant, uint32_t r_sign, uint32_t *pret);
        uint32_t mul_sf32_part2971_ret;
        if (mul_sf32__part2971(&a_exp, &a_mant, r_sign, &mul_sf32_part2971_ret))
            return mul_sf32_part2971_ret;
    }
        {
        int _early2972;
        uint32_t _r2972 = 0;
        int32_t mul_sf32__part2972(uint32_t r_sign, int32_t *pb_exp, uint32_t *pb_mant, uint32_t *pret);
        _early2972 = mul_sf32__part2972(r_sign, &b_exp, &b_mant, &_r2972);
        if (_early2972)
            return _r2972;
    }

        uint32_t mul_sf32__part2973(uint32_t r_sign, int32_t a_exp, int32_t b_exp, uint32_t a_mant, uint32_t b_mant, RoundingModeEnum rm, uint32_t *pfflags);
    (void)r_exp;
    (void)r_mant;
    (void)r_mant_low;
    return mul_sf32__part2973(r_sign, a_exp, b_exp, a_mant, b_mant, rm, pfflags);

}


uint32_t fma_sf32(uint32_t a, uint32_t b, uint32_t c, RoundingModeEnum rm,
              uint32_t *pfflags)
{
    uint32_t a_sign, b_sign, c_sign, r_sign;
    int32_t a_exp, b_exp, c_exp, r_exp, shift;
    uint32_t a_mant, b_mant, c_mant, r_mant1, r_mant0, c_mant1, c_mant0, mask;

    void fma_sf32__part2975(uint32_t a, uint32_t b, uint32_t c,
                            uint32_t *a_sign, uint32_t *b_sign, uint32_t *c_sign,
                            uint32_t *r_sign, int32_t *a_exp, int32_t *b_exp,
                            int32_t *c_exp, uint32_t *a_mant, uint32_t *b_mant,
                            uint32_t *c_mant);
    fma_sf32__part2975(a, b, c, &a_sign, &b_sign, &c_sign, &r_sign,
                       &a_exp, &b_exp, &c_exp, &a_mant, &b_mant, &c_mant);
    {
        int fma_sf32__part2976(uint32_t a, uint32_t b, uint32_t c, int32_t a_exp, int32_t b_exp, int32_t c_exp, uint32_t a_mant, uint32_t b_mant, uint32_t c_mant, uint32_t r_sign, uint32_t c_sign, uint32_t *pfflags, uint32_t *pret);
        uint32_t fma_part_ret = 0;
        if (fma_sf32__part2976(a, b, c, a_exp, b_exp, c_exp, a_mant, b_mant, c_mant, r_sign, c_sign, pfflags, &fma_part_ret))
            return fma_part_ret;
    }
    int fma_sf32__part2977(int32_t *pa_exp, uint32_t *pa_mant);
    if (!fma_sf32__part2977(&a_exp, &a_mant))
        goto mul_zero;

    {
        uint32_t fma_sf32__part2978(int32_t *pb_exp, uint32_t *pb_mant,
                                    uint32_t *pr_sign, int32_t c_exp,
                                    uint32_t c_mant, uint32_t c_sign,
                                    uint32_t c, int rm, int mode,
                                    int *pdo_ret);
        int mode;
        int do_ret = 0;
        uint32_t res;

        if (b_exp != 0) {
            mode = 0;
        } else if (b_mant != 0) {
            mode = 1;
        } else {
        mul_zero:
            mode = 2;
        }
        res = fma_sf32__part2978(&b_exp, &b_mant, &r_sign, c_exp, c_mant,
                                 c_sign, c, (int)rm, mode, &do_ret);
        if (do_ret)
            return res;
    }

        void fma_sf32__part2979(int32_t a_exp, int32_t b_exp, uint32_t a_mant, uint32_t b_mant, int32_t *r_exp, uint32_t *r_mant1, uint32_t *r_mant0);
    fma_sf32__part2979(a_exp, b_exp, a_mant, b_mant, &r_exp, &r_mant1, &r_mant0);


        void fma_sf32__part2980(uint32_t *p_r_mant1, uint32_t *p_r_mant0, int32_t *p_r_exp);
    fma_sf32__part2980(&r_mant1, &r_mant0, &r_exp);



    {
        uint32_t fma_sf32__part2981(int32_t *pc_exp, uint32_t *pc_mant,
                                    uint32_t r_sign, int32_t r_exp,
                                    uint32_t r_mant1, uint32_t r_mant0,
                                    uint32_t rm, uint32_t *pfflags,
                                    int32_t *pret);
        int32_t part2981_ret_flag = 0;
        uint32_t part2981_ret;

        part2981_ret = fma_sf32__part2981(&c_exp, &c_mant, r_sign, r_exp,
                                          r_mant1, r_mant0, (uint32_t)rm,
                                          pfflags, &part2981_ret_flag);
        if (part2981_ret_flag) {
            return part2981_ret;
        }
    }
    uint32_t fma_sf32__part2982(int32_t *c_exp, uint32_t c_mant);
    c_mant1 = fma_sf32__part2982(&c_exp, c_mant);
    c_mant0 = 0;






        void fma_sf32__part2983(int32_t *r_exp, int32_t *c_exp, uint32_t *r_mant1, uint32_t *r_mant0, uint32_t *c_mant1, uint32_t *c_mant0, uint32_t *r_sign, uint32_t *c_sign);
    fma_sf32__part2983(&r_exp, &c_exp, &r_mant1, &r_mant0, &c_mant1, &c_mant0, &r_sign, &c_sign);


    int32_t fma_sf32__part2984(int32_t r_exp, int32_t c_exp);
    shift = fma_sf32__part2984(r_exp, c_exp);

    void fma_sf32__part2985(int32_t shift, uint32_t *c_mant0, uint32_t *c_mant1, uint32_t *mask);
    fma_sf32__part2985(shift, &c_mant0, &c_mant1, &mask);



        {
        void fma_sf32__part2986(uint32_t *r_mant0, uint32_t *r_mant1, uint32_t *r_sign,
                                uint32_t c_mant0, uint32_t c_mant1, uint32_t c_sign,
                                RoundingModeEnum rm);
        fma_sf32__part2986(&r_mant0, &r_mant1, &r_sign, c_mant0, c_mant1, c_sign, rm);
    }
    uint32_t fma_sf32__part2987(uint32_t r_sign, int32_t r_exp, uint32_t r_mant1, uint32_t r_mant0, int32_t rm, uint32_t *pfflags);
    return fma_sf32__part2987(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags);

}



static uint32_t divrem_u32(uint32_t *pr, uint32_t ah, uint32_t al, uint32_t b)
{
    uint32_t divrem_u32__part2989(uint32_t *pr, uint32_t ah, uint32_t al, uint32_t b);
    return divrem_u32__part2989(pr, ah, al, b);

}
uint32_t div_sf32(uint32_t a, uint32_t b, RoundingModeEnum rm,
              uint32_t *pfflags)
{
    uint32_t a_sign, b_sign, r_sign;
    int32_t a_exp, b_exp, r_exp;
    uint32_t a_mant, b_mant, r_mant, r;
    void div_sf32__part2991(uint32_t a, uint32_t b, uint32_t *pa_sign, uint32_t *pb_sign, uint32_t *pr_sign, int32_t *pa_exp, int32_t *pb_exp, uint32_t *pa_mant, uint32_t *pb_mant);
    div_sf32__part2991(a, b, &a_sign, &b_sign, &r_sign, &a_exp, &b_exp, &a_mant, &b_mant);

    uint32_t div_sf32__part2992(uint32_t a, uint32_t b, int32_t a_exp, uint32_t a_mant,
                           int32_t b_exp, uint32_t b_mant, uint32_t r_sign,
                           uint32_t *pfflags, int *p_handled);
    {
        int part2992_handled;
        uint32_t part2992_res = div_sf32__part2992(a, b, a_exp, a_mant, b_exp, b_mant,
                                                   r_sign, pfflags, &part2992_handled);
        if (part2992_handled) {
            return part2992_res;
        }
    }


    uint32_t div_sf32__part2993(uint32_t r_sign, int32_t a_exp, uint32_t a_mant, int32_t *pb_exp, uint32_t *pb_mant, uint32_t *pfflags, int32_t *pdone);
{
        int32_t done2993;
        uint32_t ret2993 = div_sf32__part2993(r_sign, a_exp, a_mant, &b_exp, &b_mant, pfflags, &done2993);
        if (done2993)
            return ret2993;
    }
        {
        int div_sf32__part2994(int32_t *a_exp, uint32_t *a_mant, uint32_t r_sign, uint32_t *pret);
        uint32_t div_sf32__part2994_ret;
        if (div_sf32__part2994(&a_exp, &a_mant, r_sign, &div_sf32__part2994_ret))
            return div_sf32__part2994_ret;
    }

    {
    void div_sf32__part2995(int32_t a_exp, int32_t b_exp, uint32_t a_mant, uint32_t b_mant, int32_t *pr_exp, uint32_t *pr_mant, uint32_t *pr);
    div_sf32__part2995(a_exp, b_exp, a_mant, b_mant, &r_exp, &r_mant, &r);
}
    void div_sf32__part2996(uint32_t r, uint32_t *r_mant);
    div_sf32__part2996(r, &r_mant);

    uint32_t div_sf32__part2997(uint32_t r_sign, int32_t r_exp, uint32_t r_mant, RoundingModeEnum rm, uint32_t *pfflags);
    return div_sf32__part2997(r_sign, r_exp, r_mant, rm, pfflags);
}





static int sqrtrem_u32(uint32_t *pr, uint32_t ah, uint32_t al)
{
    uint64_t a, u, s;
    int l, inexact;


        {
        int sqrtrem_u32__part2999(uint32_t *pr, uint32_t ah, uint32_t al, int *pl);
        if (sqrtrem_u32__part2999(pr, ah, al, &l)) {
            return 0;
        }
    }
    void sqrtrem_u32__part3000(uint64_t *pa, uint64_t *pu, uint32_t ah, uint32_t al, int l);
    sqrtrem_u32__part3000(&a, &u, ah, al, l);

    extern void sqrtrem_u32__part3001(uint64_t a, uint64_t *ps, uint64_t *pu);
    sqrtrem_u32__part3001(a, &s, &u);

    int sqrtrem_u32__part3002(uint32_t *pr, uint64_t a, uint64_t s);
    return sqrtrem_u32__part3002(pr, a, s);

}
uint32_t sqrt_sf32(uint32_t a, RoundingModeEnum rm,
               uint32_t *pfflags)
{
    void sqrt_sf32__part3004(uint32_t a, uint32_t *pa_sign, int32_t *pa_exp, uint32_t *pa_mant);
    uint32_t a_sign;
    int32_t a_exp;
    uint32_t a_mant;

    sqrt_sf32__part3004(a, &a_sign, &a_exp, &a_mant);

    {
    uint32_t sqrt_sf32__part3005_ret = 0;
    int sqrt_sf32__part3005_st;
    extern int sqrt_sf32__part3005(uint32_t a, uint32_t a_sign, int32_t a_exp,
                                   uint32_t a_mant, uint32_t *pfflags,
                                   uint32_t *p_ret);
    sqrt_sf32__part3005_st = sqrt_sf32__part3005(a, a_sign, a_exp, a_mant,
                                                 pfflags, &sqrt_sf32__part3005_ret);
    if (sqrt_sf32__part3005_st == 1)
        return sqrt_sf32__part3005_ret;
    if (sqrt_sf32__part3005_st == 2)
        goto neg_error;
}

        {
        int32_t part3006_action;
        uint32_t part3006_ret;
        extern uint32_t sqrt_sf32__part3006(uint32_t a, uint32_t a_sign, int32_t a_exp, uint32_t a_mant, int32_t *action);
neg_error:
        part3006_ret = sqrt_sf32__part3006(a, a_sign, a_exp, a_mant, &part3006_action);
        if (part3006_action == 1)
            return part3006_ret;
        if (part3006_action == 2) {
            *pfflags |= (1 << 4);
            return F_QNAN32;
        }
    }

        {
        uint32_t sqrt_sf32__part3007(int32_t *pa_exp, uint32_t a_mant, int *pzero);
        int sqrt_sf32__pzero = 0;
        a_mant = sqrt_sf32__part3007(&a_exp, a_mant, &sqrt_sf32__pzero);
        if (sqrt_sf32__pzero)
            return pack_sf32(0, 0, 0);
    }

    void sqrt_sf32__part3008(int32_t *pa_exp);
sqrt_sf32__part3008(&a_exp);


    void sqrt_sf32__part3009(int32_t *a_exp, uint32_t *a_mant);
    sqrt_sf32__part3009(&a_exp, &a_mant);

    void sqrt_sf32__part3010(int32_t *pa_exp, uint32_t *pa_mant);
    sqrt_sf32__part3010(&a_exp, &a_mant);

    int sqrt_sf32__part3011(uint32_t *a_mant);
    if (sqrt_sf32__part3011(&a_mant))
        a_mant |= 1;

        uint32_t sqrt_sf32__part3012(uint32_t a_sign, int32_t a_exp, uint32_t a_mant, RoundingModeEnum rm, uint32_t *pfflags);
    return sqrt_sf32__part3012(a_sign, a_exp, a_mant, rm, pfflags);

}



uint32_t min_sf32(uint32_t a, uint32_t b, uint32_t *pfflags)
{
    uint32_t a_sign, b_sign;

    uint32_t min_sf32__part3014(uint32_t a, uint32_t b, uint32_t *pfflags, int *pret);
    int _min_sf32_part3014_ret = 0;
    uint32_t _min_sf32_part3014_val = min_sf32__part3014(a, b, pfflags, &_min_sf32_part3014_ret);
    if (_min_sf32_part3014_ret)
        return _min_sf32_part3014_val;

    void min_sf32__part3015(uint32_t a, uint32_t b, uint32_t *a_sign, uint32_t *b_sign);
    min_sf32__part3015(a, b, &a_sign, &b_sign);

    uint32_t min_sf32__part3016(uint32_t a, uint32_t b, uint32_t a_sign, uint32_t b_sign);
    return min_sf32__part3016(a, b, a_sign, b_sign);
}

uint32_t max_sf32(uint32_t a, uint32_t b, uint32_t *pfflags)
{
    uint32_t a_sign, b_sign;

    uint32_t max_sf32__part3018(uint32_t a, uint32_t b, uint32_t *pfflags, int *p_ret);
    {
        int max_sf32_part3018_ret;
        uint32_t max_sf32_part3018_val = max_sf32__part3018(a, b, pfflags, &max_sf32_part3018_ret);
        if (max_sf32_part3018_ret)
            return max_sf32_part3018_val;
    }
    void max_sf32__part3019(uint32_t a, uint32_t b, uint32_t *a_sign, uint32_t *b_sign);
    max_sf32__part3019(a, b, &a_sign, &b_sign);


    uint32_t max_sf32__part3020(uint32_t a, uint32_t b, uint32_t a_sign, uint32_t b_sign);
    return max_sf32__part3020(a, b, a_sign, b_sign);

}

int eq_quiet_sf32(uint32_t a, uint32_t b, uint32_t *pfflags)
{
    int eq_quiet_sf32__part3022(uint32_t a, uint32_t b, uint32_t *pfflags);
    if (eq_quiet_sf32__part3022(a, b, pfflags)) {
        return 0;
    }


    int eq_quiet_sf32__part3023(uint32_t a, uint32_t b);
    if (eq_quiet_sf32__part3023(a, b))
        return 1;

    int eq_quiet_sf32__part3024(uint32_t a, uint32_t b);
return eq_quiet_sf32__part3024(a, b);

}

int le_sf32(uint32_t a, uint32_t b, uint32_t *pfflags)
{
    uint32_t a_sign, b_sign;

    int le_sf32__part3026(uint32_t a, uint32_t b, uint32_t *pfflags);
    if (le_sf32__part3026(a, b, pfflags)) {
        return 0;
    }


    void le_sf32__part3027(uint32_t a, uint32_t b, uint32_t *a_sign, uint32_t *b_sign);
    le_sf32__part3027(a, b, &a_sign, &b_sign);
    int le_sf32__part3028(uint32_t a, uint32_t b, uint32_t a_sign, uint32_t b_sign);
    return le_sf32__part3028(a, b, a_sign, b_sign);

}

int lt_sf32(uint32_t a, uint32_t b, uint32_t *pfflags)
{
    uint32_t a_sign, b_sign;

    int lt_sf32__part3030(uint32_t a, uint32_t b, uint32_t *pfflags);
    if (lt_sf32__part3030(a, b, pfflags)) {
        return 0;
    }


    void lt_sf32__part3031(uint32_t a, uint32_t b, uint32_t *a_sign, uint32_t *b_sign);
    lt_sf32__part3031(a, b, &a_sign, &b_sign);

        int lt_sf32__part3032(uint32_t a, uint32_t b, uint32_t a_sign, uint32_t b_sign);
    return lt_sf32__part3032(a, b, a_sign, b_sign);
}

uint32_t fclass_sf32(uint32_t a)
{
    uint32_t a_sign;
    int32_t a_exp;
    uint32_t a_mant;
    uint32_t ret;

    void fclass_sf32__part3034(uint32_t a, uint32_t *a_sign, int32_t *a_exp, uint32_t *a_mant);
    fclass_sf32__part3034(a, &a_sign, &a_exp, &a_mant);

    uint32_t fclass_sf32__part3035(uint32_t a_sign, int32_t a_exp, uint32_t a_mant);
    ret = fclass_sf32__part3035(a_sign, a_exp, a_mant);
    uint32_t fclass_sf32__part3036(uint32_t ret);
    return fclass_sf32__part3036(ret);

}
static int32_t internal_cvt_sf32_i32(uint32_t a, RoundingModeEnum rm,
                                                                         uint32_t *pfflags, BOOL is_unsigned)
{
    uint32_t a_sign, addend, rnd_bits;
    int32_t a_exp;
    uint32_t a_mant;
    uint32_t r, r_max;

    void internal_cvt_sf32_i32__part3295(uint32_t a, uint32_t *a_sign, int32_t *a_exp, uint32_t *a_mant);

    internal_cvt_sf32_i32__part3295(a, &a_sign, &a_exp, &a_mant);

    uint32_t internal_cvt_sf32_i32__part3296(int32_t a_exp, uint32_t a_mant, uint32_t a_sign);
    a_sign = internal_cvt_sf32_i32__part3296(a_exp, a_mant, a_sign);

    void internal_cvt_sf32_i32__part3297(int32_t *a_exp, uint32_t *a_mant);
    internal_cvt_sf32_i32__part3297(&a_exp, &a_mant);

    uint32_t internal_cvt_sf32_i32__part3298(uint32_t a_mant, int32_t *a_exp);
    a_mant = internal_cvt_sf32_i32__part3298(a_mant, &a_exp);


    uint32_t internal_cvt_sf32_i32__part3299(uint32_t a_sign, int is_unsigned);
    r_max = internal_cvt_sf32_i32__part3299(a_sign, is_unsigned);
        {
        int internal_cvt_sf32_i32__part3300(uint32_t, uint32_t, int32_t,
                                            uint32_t *, int, uint32_t, uint32_t *);
        int ovf;
        ovf = internal_cvt_sf32_i32__part3300(a_sign, a_mant, a_exp, pfflags,
                                              (int)rm, r_max, &r);
        if (ovf)
            return r_max;
    }

    uint32_t internal_cvt_sf32_i32__part3301(uint32_t a_sign, uint32_t r);
    r = internal_cvt_sf32_i32__part3301(a_sign, r);
        {
        int32_t internal_cvt_sf32_i32__part3302(uint32_t r);
        return internal_cvt_sf32_i32__part3302(r);
    }

}

int32_t cvt_sf32_i32(uint32_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
        int32_t cvt_sf32_i32__part3304(uint32_t a, int rm, uint32_t *pfflags);
    return cvt_sf32_i32__part3304(a, rm, pfflags);
}

uint32_t cvt_sf32_u32(uint32_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
        uint32_t cvt_sf32_u32__part3306(uint32_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_sf32_u32__part3306(a, rm, pfflags);
}


static uint32_t internal_cvt_i32_sf32(int32_t a,
                                                                       RoundingModeEnum rm,
                                                                       uint32_t *pfflags,
                                                                       BOOL is_unsigned)
{
    uint32_t a_sign;
    int32_t a_exp;
    uint32_t a_mant;
    uint32_t r, mask;
    int l;

    void internal_cvt_i32_sf32__part3308(int32_t a, BOOL is_unsigned, uint32_t *a_sign, uint32_t *r);
    internal_cvt_i32_sf32__part3308(a, is_unsigned, &a_sign, &r);

        void internal_cvt_i32_sf32__part3309(uint32_t r, int32_t *p_a_exp, int *p_l);
    internal_cvt_i32_sf32__part3309(r, &a_exp, &l);

    void internal_cvt_i32_sf32__part3310(int l, uint32_t *r, uint32_t *mask, int32_t *a_exp);
    internal_cvt_i32_sf32__part3310(l, &r, &mask, &a_exp);

    uint32_t internal_cvt_i32_sf32__part3311(uint32_t a_sign, int32_t a_exp, uint32_t r, RoundingModeEnum rm, uint32_t *pfflags);
    return internal_cvt_i32_sf32__part3311(a_sign, a_exp, r, rm, pfflags);

}

uint32_t cvt_i32_sf32(int32_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
    extern uint32_t cvt_i32_sf32__part3313(int32_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_i32_sf32__part3313(a, rm, pfflags);

}

uint32_t cvt_u32_sf32(uint32_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
    uint32_t cvt_u32_sf32__part3315(uint32_t a, int rm, uint32_t *pfflags);
    return cvt_u32_sf32__part3315(a, rm, pfflags);
}


static int64_t internal_cvt_sf32_i64(uint32_t a, RoundingModeEnum rm,
                                                                         uint32_t *pfflags, BOOL is_unsigned)
{
    uint32_t a_sign, addend, rnd_bits;
    int32_t a_exp;
    uint32_t a_mant;
    uint64_t r, r_max;

    void internal_cvt_sf32_i64__part3317(uint32_t a, uint32_t *a_sign, int32_t *a_exp, uint32_t *a_mant);
    internal_cvt_sf32_i64__part3317(a, &a_sign, &a_exp, &a_mant);

    uint32_t internal_cvt_sf32_i64__part3318(int32_t a_exp, uint32_t a_mant, uint32_t a_sign);
    a_sign = internal_cvt_sf32_i64__part3318(a_exp, a_mant, a_sign);
    void internal_cvt_sf32_i64__part3319(int32_t *a_exp, uint32_t *a_mant);
    internal_cvt_sf32_i64__part3319(&a_exp, &a_mant);

    void internal_cvt_sf32_i64__part3320(uint32_t *a_mant, int32_t *a_exp);
    internal_cvt_sf32_i64__part3320(&a_mant, &a_exp);


    uint64_t internal_cvt_sf32_i64__part3321(uint32_t a_sign, int is_unsigned);
    r_max = internal_cvt_sf32_i64__part3321(a_sign, is_unsigned);
        {
        int part_status;
        int internal_cvt_sf32_i64__part3322(int32_t a_exp, uint32_t a_mant,
                                            uint32_t a_sign, RoundingModeEnum rm,
                                            uint64_t r_max, uint32_t *pfflags,
                                            uint64_t *pr);
        part_status = internal_cvt_sf32_i64__part3322(a_exp, a_mant, a_sign, rm,
                                                      r_max, pfflags, &r);
        if (part_status)
            return r;
    }

        {
        uint64_t internal_cvt_sf32_i64__part3323(uint32_t a_sign, uint64_t *r);
        r = internal_cvt_sf32_i64__part3323(a_sign, &r);
    }

    int64_t internal_cvt_sf32_i64__part3324(uint64_t r);
    return internal_cvt_sf32_i64__part3324(r);

}

int64_t cvt_sf32_i64(uint32_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
    int64_t cvt_sf32_i64__part3326(uint32_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_sf32_i64__part3326(a, rm, pfflags);
}

uint64_t cvt_sf32_u64(uint32_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
    extern uint64_t cvt_sf32_u64__part3328(uint32_t a, RoundingModeEnum rm,
                                       uint32_t *pfflags);
return cvt_sf32_u64__part3328(a, rm, pfflags);

}


static uint32_t internal_cvt_i64_sf32(int64_t a,
                                                                       RoundingModeEnum rm,
                                                                       uint32_t *pfflags,
                                                                       BOOL is_unsigned)
{
    uint32_t a_sign;
    int32_t a_exp;
    uint32_t a_mant;
    uint64_t r, mask;
    int l;

    void internal_cvt_i64_sf32__part3330(int64_t a, BOOL is_unsigned, uint32_t *a_sign, uint64_t *r);
    internal_cvt_i64_sf32__part3330(a, is_unsigned, &a_sign, &r);
        void internal_cvt_i64_sf32__part3331(uint64_t r, int32_t *pa_exp, int *pl);
    internal_cvt_i64_sf32__part3331(r, &a_exp, &l);

    void internal_cvt_i64_sf32__part3332(uint64_t *r, int32_t *a_exp, int l);
    internal_cvt_i64_sf32__part3332(&r, &a_exp, l);

    uint32_t internal_cvt_i64_sf32__part3333(uint32_t a_sign, int32_t a_exp, uint32_t a_mant, RoundingModeEnum rm, uint32_t *pfflags);
    a_mant = r;
    return internal_cvt_i64_sf32__part3333(a_sign, a_exp, a_mant, rm, pfflags);
}

uint32_t cvt_i64_sf32(int64_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
        uint32_t cvt_i64_sf32__part3335(int64_t a, int rm, uint32_t *pfflags);
    return cvt_i64_sf32__part3335(a, rm, pfflags);
}

uint32_t cvt_u64_sf32(uint64_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
    uint32_t cvt_u64_sf32__part3337(uint64_t a, RoundingModeEnum rm, uint32_t *pfflags);
return cvt_u64_sf32__part3337(a, rm, pfflags);
}



static int128_t internal_cvt_sf32_i128(uint32_t a, RoundingModeEnum rm,
                                                                         uint32_t *pfflags, BOOL is_unsigned)
{
    uint32_t a_sign, addend, rnd_bits;
    int32_t a_exp;
    uint32_t a_mant;
    uint128_t r, r_max;

    void internal_cvt_sf32_i128__part3339(uint32_t a, uint32_t *a_sign, int32_t *a_exp, uint32_t *a_mant);
    internal_cvt_sf32_i128__part3339(a, &a_sign, &a_exp, &a_mant);

    void internal_cvt_sf32_i128__part3340(int32_t a_exp, uint32_t a_mant, uint32_t *a_sign);
    internal_cvt_sf32_i128__part3340(a_exp, a_mant, &a_sign);

    void internal_cvt_sf32_i128__part3341(int32_t *a_exp, uint32_t *a_mant);
    internal_cvt_sf32_i128__part3341(&a_exp, &a_mant);

        void internal_cvt_sf32_i128__part3342(uint32_t *pa_mant, int32_t *pa_exp);
    internal_cvt_sf32_i128__part3342(&a_mant, &a_exp);


        {
        uint128_t internal_cvt_sf32_i128__part3343(BOOL is_unsigned, uint32_t a_sign);
        r_max = internal_cvt_sf32_i128__part3343(is_unsigned, a_sign);
    }

        {
        int _cvt_part_st;
        int internal_cvt_sf32_i128__part3344(int a_exp, int rm, uint32_t a_sign,
                                            uint32_t *a_mant, const uint128_t *r_max,
                                            uint128_t *r, uint32_t *pfflags);
        _cvt_part_st = internal_cvt_sf32_i128__part3344(a_exp, (int)rm, a_sign,
                                                       &a_mant, &r_max, &r, pfflags);
        if (_cvt_part_st == 1)
            return r_max;
    }

    void internal_cvt_sf32_i128__part3345(uint32_t a_sign, uint128_t *r);
    internal_cvt_sf32_i128__part3345(a_sign, &r);

        int128_t internal_cvt_sf32_i128__part3346(uint128_t r);
    return internal_cvt_sf32_i128__part3346(r);

}

int128_t cvt_sf32_i128(uint32_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
        {
        int128_t cvt_sf32_i128__part3348(uint32_t a, RoundingModeEnum rm, uint32_t *pfflags);
        return cvt_sf32_i128__part3348(a, rm, pfflags);
    }

}

uint128_t cvt_sf32_u128(uint32_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
        uint128_t cvt_sf32_u128__part3350(uint32_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_sf32_u128__part3350(a, rm, pfflags);
}


static uint32_t internal_cvt_i128_sf32(int128_t a,
                                                                       RoundingModeEnum rm,
                                                                       uint32_t *pfflags,
                                                                       BOOL is_unsigned)
{
    uint32_t a_sign;
    int32_t a_exp;
    uint32_t a_mant;
    uint128_t r, mask;
    int l;

    void internal_cvt_i128_sf32__part3352(int128_t a, BOOL is_unsigned, uint32_t *a_sign, uint128_t *r);
    internal_cvt_i128_sf32__part3352(a, is_unsigned, &a_sign, &r);

    void internal_cvt_i128_sf32__part3353(uint64_t r_hi, uint64_t r_lo, int32_t *pa_exp, int *pl);
    internal_cvt_i128_sf32__part3353((uint64_t)(r >> 64), (uint64_t)r, &a_exp, &l);

    extern void internal_cvt_i128_sf32__part3354(uint128_t *pr, uint128_t *pmask, int32_t *pa_exp, int l);
    internal_cvt_i128_sf32__part3354(&r, &mask, &a_exp, l);

        uint32_t internal_cvt_i128_sf32__part3355(uint32_t a_sign, int32_t a_exp, uint128_t r, RoundingModeEnum rm, uint32_t *pfflags);
    return internal_cvt_i128_sf32__part3355(a_sign, a_exp, r, rm, pfflags);

}

uint32_t cvt_i128_sf32(int128_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
        uint32_t cvt_i128_sf32__part3357(int128_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_i128_sf32__part3357(a, rm, pfflags);
}

uint32_t cvt_u128_sf32(uint128_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
        {
        extern uint32_t cvt_u128_sf32__part3359(uint128_t a, RoundingModeEnum rm,
                                               uint32_t *pfflags);
        return cvt_u128_sf32__part3359(a, rm, pfflags);
    }
}


static const uint64_t F_QNAN64 = (((uint64_t)((1 << 11) - 1) << 52) | ((uint64_t)1 << (52 - 1)));

static inline uint64_t pack_sf64(uint32_t a_sign, uint32_t a_exp, uint64_t a_mant)
{
        extern uint64_t pack_sf64__part3039(uint32_t a_sign, uint32_t a_exp, uint64_t a_mant);
    return pack_sf64__part3039(a_sign, a_exp, a_mant);

}

static inline uint64_t unpack_sf64(uint32_t *pa_sign, int32_t *pa_exp,
                               uint64_t a)
{
    {
    extern uint64_t unpack_sf64__part3041(uint32_t *pa_sign, int32_t *pa_exp, uint64_t a);
    return unpack_sf64__part3041(pa_sign, pa_exp, a);
}

}

static uint64_t rshift_rnd64(uint64_t a, int d)
{
    uint64_t mask;
    void rshift_rnd64__part3043(uint64_t *a, int d, uint64_t *mask);
    rshift_rnd64__part3043(&a, d, &mask);
    uint64_t rshift_rnd64__part3044(uint64_t a);
return rshift_rnd64__part3044(a);
}


static uint64_t roundpack_sf64(uint32_t a_sign, int a_exp, uint64_t a_mant,
                            RoundingModeEnum rm, uint32_t *pfflags)
{
    int diff;
    uint32_t addend, rnd_bits;

    uint32_t roundpack_sf64__part3046(uint32_t a_sign, int rm);
    addend = roundpack_sf64__part3046(a_sign, (int)rm);


    uint32_t roundpack_sf64__part3047(uint64_t *a_mant, int *a_exp, uint32_t addend, uint32_t *pfflags);
    rnd_bits = roundpack_sf64__part3047(&a_mant, &a_exp, addend, pfflags);

    void roundpack_sf64__part3048(uint32_t rnd_bits, uint32_t *pfflags);
    roundpack_sf64__part3048(rnd_bits, pfflags);

    uint64_t roundpack_sf64__part3049(uint64_t a_mant, uint32_t addend);
a_mant = roundpack_sf64__part3049(a_mant, addend);


    uint64_t roundpack_sf64__part3050(int rm, uint32_t rnd_bits, uint64_t a_mant);
    a_mant = roundpack_sf64__part3050(rm, rnd_bits, a_mant);



    int roundpack_sf64__part3051(int a_exp, uint64_t a_mant);
a_exp = roundpack_sf64__part3051(a_exp, a_mant);
    void roundpack_sf64__part3052(int *a_exp, uint64_t *a_mant, uint32_t addend, uint32_t *pfflags);
    roundpack_sf64__part3052(&a_exp, &a_mant, addend, pfflags);
        uint64_t roundpack_sf64__part3053(uint32_t a_sign, int a_exp, uint64_t a_mant);
    return roundpack_sf64__part3053(a_sign, a_exp, a_mant);

}


static uint64_t normalize_sf64(uint32_t a_sign, int a_exp, uint64_t a_mant,
                           RoundingModeEnum rm, uint32_t *pfflags)
{
    uint64_t normalize_sf64__part3055(uint32_t a_sign, int a_exp, uint64_t a_mant, RoundingModeEnum rm, uint32_t *pfflags);
    return normalize_sf64__part3055(a_sign, a_exp, a_mant, rm, pfflags);
}



static uint64_t normalize2_sf64(uint32_t a_sign, int a_exp, uint64_t a_mant1, uint64_t a_mant0,
                            RoundingModeEnum rm, uint32_t *pfflags)
{
    int l, shift;
    int normalize2_sf64__part3057(uint64_t a_mant1, uint64_t a_mant0);
    l = normalize2_sf64__part3057(a_mant1, a_mant0);
    void normalize2_sf64__part3058(int l, int *shift, int *a_exp);
    normalize2_sf64__part3058(l, &shift, &a_exp);

    {
        void normalize2_sf64__part3059(int shift, uint64_t *a_mant1, uint64_t a_mant0);
        normalize2_sf64__part3059(shift, &a_mant1, a_mant0);
    }
    uint64_t normalize2_sf64__part3060(uint32_t a_sign, int a_exp, uint64_t a_mant1, RoundingModeEnum rm, uint32_t *pfflags);
    return normalize2_sf64__part3060(a_sign, a_exp, a_mant1, rm, pfflags);
}

BOOL issignan_sf64(uint64_t a)
{
    int issignan_sf64__part3062(uint64_t a);
    return issignan_sf64__part3062(a);

}

BOOL isnan_sf64(uint64_t a)
{
    int isnan_sf64__part3064(uint64_t a);
    return isnan_sf64__part3064(a);
}


uint64_t add_sf64(uint64_t a, uint64_t b, RoundingModeEnum rm,
              uint32_t *pfflags)
{
    uint32_t a_sign, b_sign, a_exp, b_exp;
    uint64_t tmp, a_mant, b_mant;


    void add_sf64__part3066(uint64_t *a, uint64_t *b);
    add_sf64__part3066(&a, &b);
    void add_sf64__part3067(uint64_t a, uint64_t b, uint32_t *a_sign, uint32_t *b_sign, uint32_t *a_exp, uint32_t *b_exp, uint64_t *a_mant, uint64_t *b_mant);
    add_sf64__part3067(a, b, &a_sign, &b_sign, &a_exp, &b_exp, &a_mant, &b_mant);

        {
        int add_sf64__part3068(uint32_t a_exp, uint32_t a_sign, uint64_t a_mant,
                               uint64_t a, uint64_t b, uint32_t b_exp,
                               uint32_t *pfflags, uint64_t *presult);
        uint64_t add_sf64__part3068_result;
        if (add_sf64__part3068(a_exp, a_sign, a_mant, a, b, b_exp,
                               pfflags, &add_sf64__part3068_result))
            return add_sf64__part3068_result;
    }

        void add_sf64__part3069(uint32_t *a_exp, uint64_t *a_mant);
    add_sf64__part3069(&a_exp, &a_mant);

    void add_sf64__part3070(uint32_t *b_exp, uint64_t *b_mant);
    add_sf64__part3070(&b_exp, &b_mant);

        uint64_t add_sf64__part3071(uint64_t b_mant, uint32_t a_exp, uint32_t b_exp);
    b_mant = add_sf64__part3071(b_mant, a_exp, b_exp);
    void add_sf64__part3072(uint32_t *a_sign, uint32_t b_sign, uint64_t *a_mant, uint64_t b_mant, int rm);
    add_sf64__part3072(&a_sign, b_sign, &a_mant, b_mant, (int)rm);

    uint64_t add_sf64__part3073(uint32_t a_sign, uint32_t a_exp, uint64_t a_mant, int rm, uint32_t *pfflags);
    a_exp += (((64 - 2) - 52) - 3);
    return add_sf64__part3073(a_sign, a_exp, a_mant, rm, pfflags);

}

uint64_t sub_sf64(uint64_t a, uint64_t b, RoundingModeEnum rm,
                            uint32_t *pfflags)
{
    uint64_t sub_sf64__part3075(uint64_t a, uint64_t b, RoundingModeEnum rm, uint32_t *pfflags);
    return sub_sf64__part3075(a, b, rm, pfflags);

}

static inline uint64_t normalize_subnormal_sf64(int32_t *pa_exp, uint64_t a_mant)
{
    uint64_t normalize_subnormal_sf64__part3077(int32_t *pa_exp, uint64_t a_mant);
    return normalize_subnormal_sf64__part3077(pa_exp, a_mant);

}



static uint64_t mul_u64(uint64_t *plow, uint64_t a, uint64_t b)
{
        uint64_t mul_u64__part3079(uint64_t *plow, uint64_t a, uint64_t b);
    return mul_u64__part3079(plow, a, b);

}
uint64_t mul_sf64(uint64_t a, uint64_t b, RoundingModeEnum rm,
              uint32_t *pfflags)
{
    uint32_t a_sign, b_sign, r_sign;
    int32_t a_exp, b_exp, r_exp;
    uint64_t a_mant, b_mant, r_mant, r_mant_low;
    void mul_sf64__part3081(uint64_t a, uint64_t b, uint32_t *a_sign, uint32_t *b_sign, uint32_t *r_sign, int32_t *a_exp, int32_t *b_exp, uint64_t *a_mant, uint64_t *b_mant);
    mul_sf64__part3081(a, b, &a_sign, &b_sign, &r_sign, &a_exp, &b_exp, &a_mant, &b_mant);

    uint64_t mul_sf64__part3082(uint64_t a, uint64_t b, int32_t a_exp, int32_t b_exp,
                                uint32_t r_sign, uint64_t a_mant, uint64_t b_mant,
                                uint32_t *pfflags, int *p_returned);
    {
        int part3082_returned = 0;
        uint64_t part3082_value = mul_sf64__part3082(a, b, a_exp, b_exp, r_sign,
                                                     a_mant, b_mant, pfflags,
                                                     &part3082_returned);
        if (part3082_returned) {
            return part3082_value;
        }
    }

        {
        uint64_t mul_sf64__part3083(uint32_t r_sign, int32_t *pa_exp, uint64_t *pa_mant, int32_t *pearly);
        int32_t part3083_early = 0;
        uint64_t part3083_val;
        part3083_val = mul_sf64__part3083(r_sign, &a_exp, &a_mant, &part3083_early);
        if (part3083_early)
            return part3083_val;
    }

        {
        int32_t mul_sf64__part3084(int32_t *pb_exp, uint64_t *pb_mant);
        int32_t mul_sf64__part3084_ret;
        mul_sf64__part3084_ret = mul_sf64__part3084(&b_exp, &b_mant);
        if (mul_sf64__part3084_ret)
            return pack_sf64(r_sign, 0, 0);
    }

    uint64_t mul_sf64__part3085(uint32_t r_sign, int32_t a_exp, int32_t b_exp,
                            uint64_t a_mant, uint64_t b_mant,
                            RoundingModeEnum rm, uint32_t *pfflags);
    return mul_sf64__part3085(r_sign, a_exp, b_exp, a_mant, b_mant, rm, pfflags);

}


uint64_t fma_sf64(uint64_t a, uint64_t b, uint64_t c, RoundingModeEnum rm,
              uint32_t *pfflags)
{
    uint32_t a_sign, b_sign, c_sign, r_sign;
    int32_t a_exp, b_exp, c_exp, r_exp, shift;
    uint64_t a_mant, b_mant, c_mant, r_mant1, r_mant0, c_mant1, c_mant0, mask;

    void fma_sf64__part3087(uint64_t a, uint64_t b, uint64_t c,
                            uint32_t *a_sign, uint32_t *b_sign,
                            uint32_t *c_sign, uint32_t *r_sign,
                            int32_t *a_exp, int32_t *b_exp, int32_t *c_exp,
                            uint64_t *a_mant, uint64_t *b_mant,
                            uint64_t *c_mant);

    fma_sf64__part3087(a, b, c,
                       &a_sign, &b_sign, &c_sign, &r_sign,
                       &a_exp, &b_exp, &c_exp,
                       &a_mant, &b_mant, &c_mant);

        {
        extern uint64_t fma_sf64__part3088(uint64_t a, uint64_t b, uint64_t c,
                                           int32_t a_exp, int32_t b_exp, int32_t c_exp,
                                           uint64_t a_mant, uint64_t b_mant, uint64_t c_mant,
                                           uint32_t r_sign, uint32_t c_sign,
                                           uint32_t *pfflags, int *ptaken);
        int taken_3088 = 0;
        uint64_t ret_3088 = fma_sf64__part3088(a, b, c, a_exp, b_exp, c_exp,
                                              a_mant, b_mant, c_mant,
                                              r_sign, c_sign, pfflags, &taken_3088);
        if (taken_3088) {
            return ret_3088;
        }
    }

        {
        int fma_sf64__part3089(int32_t *pa_exp, uint64_t *pa_mant);
        if (fma_sf64__part3089(&a_exp, &a_mant))
            goto mul_zero;
    }

        {
        void fma_sf64__part3090(int32_t b_exp, uint64_t b_mant, uint64_t c, int32_t c_exp, uint64_t c_mant, uint32_t c_sign, uint32_t r_sign, RoundingModeEnum rm, int32_t from_zero, uint64_t *pret, int32_t *pret_flag, uint64_t *pb_mant, int32_t *pb_exp);
        uint64_t ret3090;
        int32_t retflag3090, z3090;
        uint64_t bmant3090;
        int32_t bexp3090;

        if (b_exp == 0 && b_mant == 0) {
        mul_zero:
            z3090 = 1;
        } else {
            z3090 = 0;
        }
        fma_sf64__part3090(b_exp, b_mant, c, c_exp, c_mant, c_sign, r_sign, rm, z3090,
                           &ret3090, &retflag3090, &bmant3090, &bexp3090);
        if (retflag3090)
            return ret3090;
        b_mant = bmant3090;
        b_exp = bexp3090;
    }

    uint64_t fma_sf64__part3091(int32_t *p_r_exp, uint64_t *p_r_mant0, int32_t a_exp, int32_t b_exp, uint64_t a_mant, uint64_t b_mant);
    r_mant1 = fma_sf64__part3091(&r_exp, &r_mant0, a_exp, b_exp, a_mant, b_mant);


    void fma_sf64__part3092(uint64_t *r_mant1, uint64_t *r_mant0, int32_t *r_exp);
fma_sf64__part3092(&r_mant1, &r_mant0, &r_exp);



    {
        int32_t _part3093_ret;
        int32_t fma_sf64__part3093(int32_t *p_c_exp, uint64_t *p_c_mant,
                                  uint64_t *p_r_mant1, uint64_t r_mant0);
        _part3093_ret = fma_sf64__part3093(&c_exp, &c_mant, &r_mant1, r_mant0);
        if (_part3093_ret) {
            return normalize_sf64(r_sign, r_exp, r_mant1, rm, pfflags);
        }
    }

    extern void fma_sf64__part3094(int32_t *c_exp, uint64_t c_mant, uint64_t *c_mant1, uint64_t *c_mant0);
    fma_sf64__part3094(&c_exp, c_mant, &c_mant1, &c_mant0);





        void fma_sf64__part3095(int32_t *r_exp, int32_t *c_exp, uint64_t *r_mant1, uint64_t *c_mant1, uint64_t *r_mant0, uint64_t *c_mant0, uint32_t *r_sign, uint32_t *c_sign);
    fma_sf64__part3095(&r_exp, &c_exp, &r_mant1, &c_mant1, &r_mant0, &c_mant0, &r_sign, &c_sign);


    int32_t fma_sf64__part3096(int32_t r_exp, int32_t c_exp);
    shift = fma_sf64__part3096(r_exp, c_exp);

        void fma_sf64__part3097(int32_t shift, uint64_t *pc_mant0, uint64_t *pc_mant1, uint64_t *pmask);
    fma_sf64__part3097(shift, &c_mant0, &c_mant1, &mask);



    void fma_sf64__part3098(uint32_t *r_sign, uint32_t c_sign, uint64_t *r_mant0, uint64_t *r_mant1, uint64_t c_mant0, uint64_t c_mant1, unsigned int rm);
    fma_sf64__part3098(&r_sign, c_sign, &r_mant0, &r_mant1, c_mant0, c_mant1, rm);

    uint64_t fma_sf64__part3099(uint32_t r_sign, int32_t r_exp, uint64_t r_mant1, uint64_t r_mant0, int rm, uint32_t *pfflags);
    return fma_sf64__part3099(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags);

}



static uint64_t divrem_u64(uint64_t *pr, uint64_t ah, uint64_t al, uint64_t b)
{
    uint64_t divrem_u64__part3101(uint64_t *pr, uint64_t ah, uint64_t al, uint64_t b);
    return divrem_u64__part3101(pr, ah, al, b);

}
uint64_t div_sf64(uint64_t a, uint64_t b, RoundingModeEnum rm,
              uint32_t *pfflags)
{
        uint32_t a_sign, b_sign, r_sign;
    int32_t a_exp, b_exp, r_exp;
    uint64_t a_mant, b_mant, r_mant, r;

    void div_sf64__part3103(uint64_t a, uint64_t b, uint32_t *a_sign,
                            uint32_t *b_sign, uint32_t *r_sign,
                            int32_t *a_exp, int32_t *b_exp,
                            uint64_t *a_mant, uint64_t *b_mant);
    div_sf64__part3103(a, b, &a_sign, &b_sign, &r_sign,
                       &a_exp, &b_exp, &a_mant, &b_mant);

        {
        uint64_t div_sf64__part3104(uint64_t, uint64_t, uint32_t *, uint32_t, int32_t, int32_t, uint64_t, uint64_t, int *);
        int part3104_ret;
        uint64_t part3104_val = div_sf64__part3104(a, b, pfflags, r_sign, a_exp, b_exp, a_mant, b_mant, &part3104_ret);
        if (part3104_ret) {
            return part3104_val;
        }
    }


        {
        int div_sf64__part3105(int32_t a_exp, uint64_t a_mant, uint32_t r_sign,
                               int32_t *pb_exp, uint64_t *pb_mant,
                               uint32_t *pfflags, uint64_t *pret);
        uint64_t part_ret = 0;
        if (div_sf64__part3105(a_exp, a_mant, r_sign, &b_exp, &b_mant,
                               pfflags, &part_ret)) {
            return part_ret;
        }
    }

    uint64_t div_sf64__part3106(int32_t *pa_exp, uint64_t *pa_mant, uint32_t r_sign, int *p_early);
    {
        int part3106_early;
        uint64_t part3106_ret = div_sf64__part3106(&a_exp, &a_mant, r_sign, &part3106_early);
        if (part3106_early)
            return part3106_ret;
    }
        {
        uint64_t div_sf64__part3107(int32_t a_exp, int32_t b_exp, uint64_t a_mant, uint64_t b_mant, int32_t *r_exp, uint64_t *r);
        r_mant = div_sf64__part3107(a_exp, b_exp, a_mant, b_mant, &r_exp, &r);
    }

    uint64_t div_sf64__part3108(uint64_t r, uint64_t r_mant);
    r_mant = div_sf64__part3108(r, r_mant);

        uint64_t div_sf64__part3109(uint32_t r_sign, int32_t r_exp, uint64_t r_mant, RoundingModeEnum rm, uint32_t *pfflags);
    return div_sf64__part3109(r_sign, r_exp, r_mant, rm, pfflags);
}





static int sqrtrem_u64(uint64_t *pr, uint64_t ah, uint64_t al)
{
    uint128_t a, u, s;
    int l, inexact;


        {
        int sqrtrem_u64__part3111(uint64_t *pr, uint64_t ah, uint64_t al, int *pl);
        int sqrtrem_u64__early = sqrtrem_u64__part3111(pr, ah, al, &l);
        if (sqrtrem_u64__early)
            return 0;
    }

        void sqrtrem_u64__part3112(uint128_t *pa, uint128_t *pu, uint64_t ah, uint64_t al, int l);
    sqrtrem_u64__part3112(&a, &u, ah, al, l);

        {
        void sqrtrem_u64__part3113(uint128_t *ps, uint128_t u, uint128_t a);
        sqrtrem_u64__part3113(&s, u, a);
    }

    int sqrtrem_u64__part3114(uint64_t *pr, uint64_t a_hi, uint64_t a_lo,
                          uint64_t s_hi, uint64_t s_lo);
    inexact = sqrtrem_u64__part3114(pr, (uint64_t)(a >> 64), (uint64_t)a,
                                   (uint64_t)(s >> 64), (uint64_t)s);
    return inexact;

}
uint64_t sqrt_sf64(uint64_t a, RoundingModeEnum rm,
               uint32_t *pfflags)
{
    uint32_t a_sign;
    int32_t a_exp;
    uint64_t a_mant;

    void sqrt_sf64__part3116(uint64_t a, uint32_t *a_sign, int32_t *a_exp, uint64_t *a_mant);
    sqrt_sf64__part3116(a, &a_sign, &a_exp, &a_mant);

    {
        int32_t sqrt_sf64__part3117(uint64_t a, uint32_t a_sign, int32_t a_exp,
                                    uint64_t a_mant, uint32_t *pfflags);
        int32_t _sqrt_sf64_code = sqrt_sf64__part3117(a, a_sign, a_exp, a_mant, pfflags);
        if (_sqrt_sf64_code == 1) {
            return F_QNAN64;
        } else if (_sqrt_sf64_code == 2) {
            return a;
        }
    }
    uint64_t sqrt_sf64__part3118(uint64_t a, uint32_t a_sign, int32_t a_exp, uint64_t a_mant, uint32_t *pfflags);
    if (a_sign) {
        if (a_exp == 0 && a_mant == 0)
            return a;
    neg_error:
        return sqrt_sf64__part3118(a, a_sign, a_exp, a_mant, pfflags);
    }

    int sqrt_sf64__part3119(int32_t *pa_exp, uint64_t *pa_mant);
    if (sqrt_sf64__part3119(&a_exp, &a_mant))
        return pack_sf64(0, 0, 0);

        int32_t sqrt_sf64__part3120(int32_t a_exp);
    a_exp = sqrt_sf64__part3120(a_exp);


    void sqrt_sf64__part3121(int32_t *a_exp, uint64_t *a_mant);
    sqrt_sf64__part3121(&a_exp, &a_mant);

        void sqrt_sf64__part3122(int32_t a_exp, uint64_t a_mant, int32_t *a_exp_out, uint64_t *a_mant_out);
    sqrt_sf64__part3122(a_exp, a_mant, &a_exp, &a_mant);

    void sqrt_sf64__part3123(uint64_t *pa_mant);
    sqrt_sf64__part3123(&a_mant);

    uint64_t sqrt_sf64__part3124(uint32_t a_sign, int32_t a_exp, uint64_t a_mant, int rm, uint32_t *pfflags);
return sqrt_sf64__part3124(a_sign, a_exp, a_mant, rm, pfflags);

}



uint64_t min_sf64(uint64_t a, uint64_t b, uint32_t *pfflags)
{
    uint32_t a_sign, b_sign;

    extern int min_sf64__part3126(uint64_t a, uint64_t b, uint32_t *pfflags, uint64_t *out);
    {
        uint64_t _part3126_val;
        if (min_sf64__part3126(a, b, pfflags, &_part3126_val))
            return _part3126_val;
    }

    void min_sf64__part3127(uint64_t a, uint64_t b, uint32_t *a_sign, uint32_t *b_sign);
    min_sf64__part3127(a, b, &a_sign, &b_sign);


    uint64_t min_sf64__part3128(uint64_t a, uint64_t b, uint32_t a_sign, uint32_t b_sign);
    return min_sf64__part3128(a, b, a_sign, b_sign);

}

uint64_t max_sf64(uint64_t a, uint64_t b, uint32_t *pfflags)
{
    uint32_t a_sign, b_sign;

        {
        int max_sf64__part3130(uint64_t a, uint64_t b, uint32_t *pfflags, uint64_t *pret);
        uint64_t max_sf64__ret;
        if (max_sf64__part3130(a, b, pfflags, &max_sf64__ret))
            return max_sf64__ret;
    }

    void max_sf64__part3131(uint64_t a, uint64_t b, uint32_t *a_sign, uint32_t *b_sign);
    max_sf64__part3131(a, b, &a_sign, &b_sign);


    uint64_t max_sf64__part3132(uint64_t a, uint64_t b, uint32_t a_sign, uint32_t b_sign);
    return max_sf64__part3132(a, b, a_sign, b_sign);

}

int eq_quiet_sf64(uint64_t a, uint64_t b, uint32_t *pfflags)
{
    int eq_quiet_sf64__part3134(uint64_t a, uint64_t b, uint32_t *pfflags);
    if (eq_quiet_sf64__part3134(a, b, pfflags))
        return 0;


    int eq_quiet_sf64__part3135(uint64_t a, uint64_t b);
    if (eq_quiet_sf64__part3135(a, b))
        return 1;
    int eq_quiet_sf64__part3136(uint64_t a, uint64_t b);
    return eq_quiet_sf64__part3136(a, b);
}

int le_sf64(uint64_t a, uint64_t b, uint32_t *pfflags)
{
    uint32_t a_sign, b_sign;

    int le_sf64__part3138(uint64_t a, uint64_t b, uint32_t *pfflags);
    if (le_sf64__part3138(a, b, pfflags)) {
        return 0;
    }


    void le_sf64__part3139(uint64_t a, uint64_t b, uint32_t *a_sign, uint32_t *b_sign);
    le_sf64__part3139(a, b, &a_sign, &b_sign);
        {
        int le_sf64__part3140(uint64_t a, uint64_t b, uint32_t a_sign);
        return le_sf64__part3140(a, b, a_sign);
    }

}

int lt_sf64(uint64_t a, uint64_t b, uint32_t *pfflags)
{
    uint32_t a_sign, b_sign;

    int lt_sf64__part3142(uint64_t a, uint64_t b, uint32_t *pfflags);
    if (lt_sf64__part3142(a, b, pfflags)) {
        return 0;
    }

    void lt_sf64__part3143(uint64_t a, uint64_t b, uint32_t *a_sign, uint32_t *b_sign);
    lt_sf64__part3143(a, b, &a_sign, &b_sign);
        int lt_sf64__part3144(uint64_t a, uint64_t b, uint32_t a_sign, uint32_t b_sign);
    return lt_sf64__part3144(a, b, a_sign, b_sign);

}

uint32_t fclass_sf64(uint64_t a)
{
        uint32_t a_sign;
    int32_t a_exp;
    uint64_t a_mant;
    uint32_t ret;

    void fclass_sf64__part3146(uint64_t a, uint32_t *a_sign, int32_t *a_exp, uint64_t *a_mant, uint32_t *ret);
    fclass_sf64__part3146(a, &a_sign, &a_exp, &a_mant, &ret);

    uint32_t fclass_sf64__part3147(uint32_t a_sign, int32_t a_exp, uint64_t a_mant);
    ret = fclass_sf64__part3147(a_sign, a_exp, a_mant);

        {
        uint32_t fclass_sf64__part3148(uint32_t ret);
        return fclass_sf64__part3148(ret);
    }

}





uint64_t cvt_sf32_sf64(uint32_t a, uint32_t *pfflags)
{
    uint32_t a_sign;
    int32_t a_exp;
    uint64_t a_mant;

    void cvt_sf32_sf64__part3150(uint32_t a, uint32_t *pa_sign, int32_t *pa_exp, uint64_t *pa_mant);
    cvt_sf32_sf64__part3150(a, &a_sign, &a_exp, &a_mant);

    {
    extern uint64_t cvt_sf32_sf64__part3151(uint32_t a, int32_t a_exp, uint64_t a_mant, uint32_t a_sign, uint32_t *pfflags, int32_t *p_done);
    int32_t part3151_done = 0;
    uint64_t part3151_ret = cvt_sf32_sf64__part3151(a, a_exp, a_mant, a_sign, pfflags, &part3151_done);
    if (part3151_done)
        return part3151_ret;
}

    uint64_t cvt_sf32_sf64__part3152(uint32_t a_sign, int32_t *pa_exp, uint64_t *pa_mant, int *pret);
    {
        int part3152_ret;
        uint64_t part3152_val = cvt_sf32_sf64__part3152(a_sign, &a_exp, &a_mant, &part3152_ret);
        if (part3152_ret)
            return part3152_val;
    }


        uint64_t cvt_sf32_sf64__part3153(uint32_t a_sign, int32_t a_exp, uint64_t a_mant);

    return cvt_sf32_sf64__part3153(a_sign, a_exp, a_mant);
}

uint32_t cvt_sf64_sf32(uint64_t a, RoundingModeEnum rm,
                                           uint32_t *pfflags)
{
    void cvt_sf64_sf32__part3155(uint32_t *pa_sign, int32_t *pa_exp, uint64_t *pa_mant, uint64_t a);
    uint32_t a_sign;
    int32_t a_exp;
    uint64_t a_mant;

    cvt_sf64_sf32__part3155(&a_sign, &a_exp, &a_mant, a);

    {
        int cvt_sf64_sf32__part3156(int32_t a_exp, uint64_t a, uint32_t a_sign, uint64_t a_mant, uint32_t *pfflags, uint32_t *presult);
        uint32_t part3156_result;
        if (cvt_sf64_sf32__part3156(a_exp, a, a_sign, a_mant, pfflags, &part3156_result))
            return part3156_result;
    }
        {
        uint32_t part3157_early;
        uint32_t part3157_ret;
        extern uint32_t cvt_sf64_sf32__part3157(uint32_t a_sign, int32_t *pa_exp,
                                               uint64_t *pa_mant, uint32_t *p_early);
        part3157_ret = cvt_sf64_sf32__part3157(a_sign, &a_exp, &a_mant, &part3157_early);
        if (part3157_early)
            return part3157_ret;
    }

    uint32_t cvt_sf64_sf32__part3158(uint32_t a_sign, int32_t a_exp, uint64_t a_mant, int rm, uint32_t *pfflags);
    return cvt_sf64_sf32__part3158(a_sign, a_exp, a_mant, rm, pfflags);

}
static int32_t internal_cvt_sf64_i32(uint64_t a, RoundingModeEnum rm,
                                                                         uint32_t *pfflags, BOOL is_unsigned)
{
    uint32_t a_sign, addend, rnd_bits;
    int32_t a_exp;
    uint64_t a_mant;
    uint32_t r, r_max;

    void internal_cvt_sf64_i32__part3361(uint64_t a, uint32_t *a_sign, int32_t *a_exp, uint64_t *a_mant);
    internal_cvt_sf64_i32__part3361(a, &a_sign, &a_exp, &a_mant);
    uint32_t internal_cvt_sf64_i32__part3362(int32_t a_exp, uint64_t a_mant, uint32_t a_sign);
    a_sign = internal_cvt_sf64_i32__part3362(a_exp, a_mant, a_sign);

    void internal_cvt_sf64_i32__part3363(int32_t *a_exp, uint64_t *a_mant);
    internal_cvt_sf64_i32__part3363(&a_exp, &a_mant);

    void internal_cvt_sf64_i32__part3364(uint64_t *a_mant, int32_t *a_exp);
    internal_cvt_sf64_i32__part3364(&a_mant, &a_exp);

        uint32_t internal_cvt_sf64_i32__part3365(uint32_t a_sign, int is_unsigned);
    r_max = internal_cvt_sf64_i32__part3365(a_sign, is_unsigned);

        {
        int32_t internal_cvt_sf64_i32__part3366(int32_t a_exp, uint64_t a_mant,
            RoundingModeEnum rm, uint32_t a_sign, uint32_t r_max,
            uint32_t *pfflags, uint32_t *r_out, uint64_t *a_mant_out);
        uint32_t _part_r = 0;
        uint64_t _part_a_mant = a_mant;
        if (internal_cvt_sf64_i32__part3366(a_exp, a_mant, rm, a_sign, r_max,
                                           pfflags, &_part_r, &_part_a_mant)) {
            return r_max;
        }
        r = _part_r;
        a_mant = _part_a_mant;
    }

    uint32_t internal_cvt_sf64_i32__part3367(uint32_t a_sign, uint32_t r);
    r = internal_cvt_sf64_i32__part3367(a_sign, r);

        int32_t internal_cvt_sf64_i32__part3368(uint32_t r);
    return internal_cvt_sf64_i32__part3368(r);

}

int32_t cvt_sf64_i32(uint64_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
        int32_t cvt_sf64_i32__part3370(uint64_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_sf64_i32__part3370(a, rm, pfflags);
}

uint32_t cvt_sf64_u32(uint64_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
    uint32_t cvt_sf64_u32__part3372(uint64_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_sf64_u32__part3372(a, rm, pfflags);
}


static uint64_t internal_cvt_i32_sf64(int32_t a,
                                                                       RoundingModeEnum rm,
                                                                       uint32_t *pfflags,
                                                                       BOOL is_unsigned)
{
    uint32_t a_sign;
    int32_t a_exp;
    uint64_t a_mant;
    uint32_t r, mask;
    int l;

    void internal_cvt_i32_sf64__part3374(int32_t a, BOOL is_unsigned, uint32_t *a_sign, uint32_t *r);
    internal_cvt_i32_sf64__part3374(a, is_unsigned, &a_sign, &r);

    void internal_cvt_i32_sf64__part3375(uint32_t r, int32_t *a_exp, int *l);
    internal_cvt_i32_sf64__part3375(r, &a_exp, &l);

    extern void internal_cvt_i32_sf64__part3376(uint32_t *r, int32_t *a_exp, int l);
    internal_cvt_i32_sf64__part3376(&r, &a_exp, l);

    uint64_t internal_cvt_i32_sf64__part3377(uint32_t a_sign, int32_t a_exp, uint32_t r, uint32_t rm, uint32_t *pfflags);
    a_mant = r;
    return internal_cvt_i32_sf64__part3377(a_sign, a_exp, a_mant, rm, pfflags);
}

uint64_t cvt_i32_sf64(int32_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
        uint64_t cvt_i32_sf64__part3379(int32_t a, int rm, uint32_t *pfflags);
    return cvt_i32_sf64__part3379(a, rm, pfflags);

}

uint64_t cvt_u32_sf64(uint32_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
        uint64_t cvt_u32_sf64__part3381(uint32_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_u32_sf64__part3381(a, rm, pfflags);
}


static int64_t internal_cvt_sf64_i64(uint64_t a, RoundingModeEnum rm,
                                                                         uint32_t *pfflags, BOOL is_unsigned)
{
    uint32_t a_sign, addend, rnd_bits;
    int32_t a_exp;
    uint64_t a_mant;
    uint64_t r, r_max;
    void internal_cvt_sf64_i64__part3383(uint64_t a, uint32_t *a_sign, int32_t *a_exp, uint64_t *a_mant);

    internal_cvt_sf64_i64__part3383(a, &a_sign, &a_exp, &a_mant);

    uint32_t internal_cvt_sf64_i64__part3384(int32_t a_exp, uint64_t a_mant, uint32_t a_sign);
    a_sign = internal_cvt_sf64_i64__part3384(a_exp, a_mant, a_sign);

        void internal_cvt_sf64_i64__part3385(int32_t *a_exp, uint64_t *a_mant);
    internal_cvt_sf64_i64__part3385(&a_exp, &a_mant);

    void internal_cvt_sf64_i64__part3386(uint64_t *a_mant, int32_t *a_exp);
    internal_cvt_sf64_i64__part3386(&a_mant, &a_exp);


    uint64_t internal_cvt_sf64_i64__part3387(int is_unsigned, uint32_t a_sign);
    r_max = internal_cvt_sf64_i64__part3387(is_unsigned, a_sign);
        {
        uint64_t part_r;
        extern int internal_cvt_sf64_i64__part3388(uint64_t a_mant, int32_t a_exp,
                                                   uint32_t a_sign, uint32_t rm,
                                                   uint64_t r_max, uint32_t *pfflags,
                                                   uint64_t *r_out);
        if (internal_cvt_sf64_i64__part3388(a_mant, a_exp, a_sign, (uint32_t)rm,
                                            r_max, pfflags, &part_r))
            return (int64_t)r_max;
        r = part_r;
    }

        uint64_t internal_cvt_sf64_i64__part3389(uint32_t a_sign, uint64_t r);
    r = internal_cvt_sf64_i64__part3389(a_sign, r);

    int64_t internal_cvt_sf64_i64__part3390(uint64_t r);
    return internal_cvt_sf64_i64__part3390(r);
}

int64_t cvt_sf64_i64(uint64_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
    int64_t cvt_sf64_i64__part3392(uint64_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_sf64_i64__part3392(a, rm, pfflags);
}

uint64_t cvt_sf64_u64(uint64_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
    uint64_t cvt_sf64_u64__part3394(uint64_t a, int rm, uint32_t *pfflags);
    return cvt_sf64_u64__part3394(a, rm, pfflags);
}


static uint64_t internal_cvt_i64_sf64(int64_t a,
                                                                       RoundingModeEnum rm,
                                                                       uint32_t *pfflags,
                                                                       BOOL is_unsigned)
{
    uint32_t a_sign;
    int32_t a_exp;
    uint64_t a_mant;
    uint64_t r, mask;
    int l;

    void internal_cvt_i64_sf64__part3396(int64_t a, BOOL is_unsigned, uint32_t *a_sign, uint64_t *r);
    internal_cvt_i64_sf64__part3396(a, is_unsigned, &a_sign, &r);
        void internal_cvt_i64_sf64__part3397(uint64_t r, int32_t *a_exp, int *l);
    internal_cvt_i64_sf64__part3397(r, &a_exp, &l);

        void internal_cvt_i64_sf64__part3398(int l, uint64_t *r, int32_t *a_exp, uint64_t *mask);
    if (l > 0) {
        internal_cvt_i64_sf64__part3398(l, &r, &a_exp, &mask);
    }

        a_mant = r;
    {
        extern uint64_t internal_cvt_i64_sf64__part3399(uint32_t a_sign, int32_t a_exp, uint64_t r, RoundingModeEnum rm, uint32_t *pfflags);
        return internal_cvt_i64_sf64__part3399(a_sign, a_exp, r, rm, pfflags);
    }
}

uint64_t cvt_i64_sf64(int64_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
    uint64_t cvt_i64_sf64__part3401(int64_t a, int rm, uint32_t *pfflags);
    return cvt_i64_sf64__part3401(a, rm, pfflags);
}

uint64_t cvt_u64_sf64(uint64_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
    uint64_t cvt_u64_sf64__part3403(uint64_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_u64_sf64__part3403(a, rm, pfflags);
}



static int128_t internal_cvt_sf64_i128(uint64_t a, RoundingModeEnum rm,
                                                                         uint32_t *pfflags, BOOL is_unsigned)
{
    uint32_t a_sign, addend, rnd_bits;
    int32_t a_exp;
    uint64_t a_mant;
    uint128_t r, r_max;

    void internal_cvt_sf64_i128__part3405(uint64_t a, uint32_t *a_sign, int32_t *a_exp, uint64_t *a_mant);
    internal_cvt_sf64_i128__part3405(a, &a_sign, &a_exp, &a_mant);

    extern void internal_cvt_sf64_i128__part3406(int32_t a_exp, uint64_t a_mant, uint32_t *a_sign);
    internal_cvt_sf64_i128__part3406(a_exp, a_mant, &a_sign);

    void internal_cvt_sf64_i128__part3407(int32_t *a_exp, uint64_t *a_mant);
    internal_cvt_sf64_i128__part3407(&a_exp, &a_mant);

    void internal_cvt_sf64_i128__part3408(uint64_t *a_mant, int32_t *a_exp);
    internal_cvt_sf64_i128__part3408(&a_mant, &a_exp);


    {
        uint128_t internal_cvt_sf64_i128__part3409(uint32_t a_sign, BOOL is_unsigned);
        r_max = internal_cvt_sf64_i128__part3409(a_sign, is_unsigned);
    }
    int internal_cvt_sf64_i128__part3410(int32_t a_exp, uint32_t a_sign, unsigned int rm, uint32_t *pfflags, const uint128_t *p_r_max, uint64_t a_mant, uint128_t *p_r);
if (internal_cvt_sf64_i128__part3410(a_exp, a_sign, (unsigned int)rm, pfflags, &r_max, a_mant, &r))
    return r_max;

        {
        extern void internal_cvt_sf64_i128__part3411(uint32_t a_sign, uint128_t *pr);
        internal_cvt_sf64_i128__part3411(a_sign, &r);
    }

        int128_t internal_cvt_sf64_i128__part3412(uint128_t r);
    return internal_cvt_sf64_i128__part3412(r);

}

int128_t cvt_sf64_i128(uint64_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
        int128_t cvt_sf64_i128__part3414(uint64_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_sf64_i128__part3414(a, rm, pfflags);

}

uint128_t cvt_sf64_u128(uint64_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
        uint128_t cvt_sf64_u128__part3416(uint64_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_sf64_u128__part3416(a, rm, pfflags);

}


static uint64_t internal_cvt_i128_sf64(int128_t a,
                                                                       RoundingModeEnum rm,
                                                                       uint32_t *pfflags,
                                                                       BOOL is_unsigned)
{
    uint32_t a_sign;
    int32_t a_exp;
    uint64_t a_mant;
    uint128_t r, mask;
    int l;

    void internal_cvt_i128_sf64__part3418(int128_t a, BOOL is_unsigned, uint32_t *pa_sign, uint128_t *pr);
    internal_cvt_i128_sf64__part3418(a, is_unsigned, &a_sign, &r);
    void internal_cvt_i128_sf64__part3419(uint128_t r, int32_t *pa_exp, int *pl);
    internal_cvt_i128_sf64__part3419(r, &a_exp, &l);

    void internal_cvt_i128_sf64__part3420(uint128_t *r, int32_t *a_exp, int l);
    internal_cvt_i128_sf64__part3420(&r, &a_exp, l);
    uint64_t internal_cvt_i128_sf64__part3421(uint32_t a_sign, int32_t a_exp, uint64_t a_mant, uint32_t rm, uint32_t *pfflags);
    a_mant = r;
    return internal_cvt_i128_sf64__part3421(a_sign, a_exp, a_mant, rm, pfflags);

}

uint64_t cvt_i128_sf64(int128_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
    uint64_t cvt_i128_sf64__part3423(int128_t a, RoundingModeEnum rm, uint32_t *pfflags);
return cvt_i128_sf64__part3423(a, rm, pfflags);

}

uint64_t cvt_u128_sf64(uint128_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
        uint64_t cvt_u128_sf64__part3425(uint128_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_u128_sf64__part3425(a, rm, pfflags);

}




static const uint128_t F_QNAN128 = (((uint128_t)((1 << 15) - 1) << 112) | ((uint128_t)1 << (112 - 1)));

static inline uint128_t pack_sf128(uint32_t a_sign, uint32_t a_exp, uint128_t a_mant)
{
    uint128_t pack_sf128__part3161(uint32_t a_sign, uint32_t a_exp, uint128_t a_mant);
    return pack_sf128__part3161(a_sign, a_exp, a_mant);

}

static inline uint128_t unpack_sf128(uint32_t *pa_sign, int32_t *pa_exp,
                               uint128_t a)
{
    uint128_t unpack_sf128__part3163(uint32_t *pa_sign, int32_t *pa_exp, uint128_t a);
    return unpack_sf128__part3163(pa_sign, pa_exp, a);
}

static uint128_t rshift_rnd128(uint128_t a, int d)
{
    uint128_t mask;
    uint128_t rshift_rnd128__part3165(uint128_t a, int d);
    a = rshift_rnd128__part3165(a, d);

    uint128_t rshift_rnd128__part3166(uint128_t a);
return rshift_rnd128__part3166(a);
}


static uint128_t roundpack_sf128(uint32_t a_sign, int a_exp, uint128_t a_mant,
                            RoundingModeEnum rm, uint32_t *pfflags)
{
    int diff;
    uint32_t addend, rnd_bits;

        uint32_t roundpack_sf128__part3168(uint32_t a_sign, int rm);
    addend = roundpack_sf128__part3168(a_sign, rm);



        {
        void roundpack_sf128__part3169(int a_exp, uint128_t a_mant, uint32_t addend,
                                       uint32_t *pfflags, int *pa_exp,
                                       uint128_t *pa_mant, uint32_t *prnd_bits);
        roundpack_sf128__part3169(a_exp, a_mant, addend, pfflags,
                                  &a_exp, &a_mant, &rnd_bits);
    }

    void roundpack_sf128__part3170(uint32_t rnd_bits, uint32_t *pfflags);
    roundpack_sf128__part3170(rnd_bits, pfflags);

        {
        uint128_t roundpack_sf128__part3171(uint128_t a_mant, uint32_t addend);
        a_mant = roundpack_sf128__part3171(a_mant, addend);
    }


    void roundpack_sf128__part3172(int rm, uint32_t rnd_bits, uint128_t *a_mant);
    roundpack_sf128__part3172(rm, rnd_bits, &a_mant);



        {
        int roundpack_sf128__part3173(int a_exp, uint128_t a_mant);
        a_exp = roundpack_sf128__part3173(a_exp, a_mant);
    }

    void roundpack_sf128__part3174(uint128_t *pa_mant, int *pa_exp, uint32_t addend, uint32_t *pfflags);
    roundpack_sf128__part3174(&a_mant, &a_exp, addend, pfflags);

        uint128_t roundpack_sf128__part3175(uint32_t a_sign, int a_exp, uint128_t a_mant);
    return roundpack_sf128__part3175(a_sign, a_exp, a_mant);

}


static uint128_t normalize_sf128(uint32_t a_sign, int a_exp, uint128_t a_mant,
                           RoundingModeEnum rm, uint32_t *pfflags)
{
    uint128_t normalize_sf128__part3177(uint32_t a_sign, int a_exp, uint128_t a_mant, RoundingModeEnum rm, uint32_t *pfflags);
    return normalize_sf128__part3177(a_sign, a_exp, a_mant, rm, pfflags);
}



static uint128_t normalize2_sf128(uint32_t a_sign, int a_exp, uint128_t a_mant1, uint128_t a_mant0,
                            RoundingModeEnum rm, uint32_t *pfflags)
{
    int l, shift;
    int normalize2_sf128__part3179(uint128_t a_mant1, uint128_t a_mant0);
    l = normalize2_sf128__part3179(a_mant1, a_mant0);

    int normalize2_sf128__part3180(int l, int *a_exp);
    shift = normalize2_sf128__part3180(l, &a_exp);

        {
        void normalize2_sf128__part3181(int shift, uint128_t *pa_mant1, uint128_t *pa_mant0);
        normalize2_sf128__part3181(shift, &a_mant1, &a_mant0);
    }

        uint128_t normalize2_sf128__part3182(uint32_t a_sign, int a_exp, uint128_t a_mant1, RoundingModeEnum rm, uint32_t *pfflags);
    return normalize2_sf128__part3182(a_sign, a_exp, a_mant1, rm, pfflags);

}

BOOL issignan_sf128(uint128_t a)
{
    BOOL issignan_sf128__part3184(uint128_t a);
    return issignan_sf128__part3184(a);

}

BOOL isnan_sf128(uint128_t a)
{
    int isnan_sf128__part3186(uint128_t a);
    return isnan_sf128__part3186(a);

}


uint128_t add_sf128(uint128_t a, uint128_t b, RoundingModeEnum rm,
              uint32_t *pfflags)
{
    uint32_t a_sign, b_sign, a_exp, b_exp;
    uint128_t tmp, a_mant, b_mant;


    extern void add_sf128__part3188(uint128_t *a, uint128_t *b);
    add_sf128__part3188(&a, &b);

    void add_sf128__part3189(const uint128_t *a_ptr, const uint128_t *b_ptr,
                         uint32_t *a_sign, uint32_t *b_sign,
                         uint32_t *a_exp, uint32_t *b_exp,
                         uint128_t *a_mant, uint128_t *b_mant);
    add_sf128__part3189(&a, &b, &a_sign, &b_sign, &a_exp, &b_exp, &a_mant, &b_mant);

        {
        int add_sf128__part3190(uint128_t a, uint128_t b, uint32_t a_exp,
                                uint32_t b_exp, uint32_t a_sign, uint32_t b_sign,
                                uint128_t a_mant, uint32_t *pfflags,
                                uint128_t *pr);
        uint128_t add_sf128_part3190_r;
        if (add_sf128__part3190(a, b, a_exp, b_exp, a_sign, b_sign, a_mant,
                                pfflags, &add_sf128_part3190_r))
            return add_sf128_part3190_r;
    }

    void add_sf128__part3191(uint32_t *a_exp, uint128_t *a_mant);
    add_sf128__part3191(&a_exp, &a_mant);

    uint32_t add_sf128__part3192(uint32_t b_exp, uint128_t *b_mant);
    b_exp = add_sf128__part3192(b_exp, &b_mant);

        void add_sf128__part3193(uint128_t *b_mant, uint32_t a_exp, uint32_t b_exp);
    add_sf128__part3193(&b_mant, a_exp, b_exp);

    void add_sf128__part3194(uint32_t *a_sign, uint32_t b_sign, uint128_t *a_mant, uint128_t b_mant, int rm);
    add_sf128__part3194(&a_sign, b_sign, &a_mant, b_mant, (int)rm);

    uint128_t add_sf128__part3195(uint32_t a_sign, int a_exp, uint128_t a_mant, RoundingModeEnum rm, uint32_t *pfflags);
    a_exp += (((128 - 2) - 112) - 3);
    return add_sf128__part3195(a_sign, a_exp, a_mant, rm, pfflags);

}

uint128_t sub_sf128(uint128_t a, uint128_t b, RoundingModeEnum rm,
                            uint32_t *pfflags)
{
    uint128_t sub_sf128__part3197(uint128_t a, uint128_t b, RoundingModeEnum rm, uint32_t *pfflags);
    return sub_sf128__part3197(a, b, rm, pfflags);

}

static inline uint128_t normalize_subnormal_sf128(int32_t *pa_exp, uint128_t a_mant)
{
    uint128_t normalize_subnormal_sf128__part3199(int32_t *pa_exp, uint128_t a_mant);
    return normalize_subnormal_sf128__part3199(pa_exp, a_mant);

}
static uint128_t mul_u128(uint128_t *plow, uint128_t a, uint128_t b)
{
    uint128_t mul_u128__part3201(uint128_t *plow, uint128_t a, uint128_t b);
    return mul_u128__part3201(plow, a, b);
}





uint128_t mul_sf128(uint128_t a, uint128_t b, RoundingModeEnum rm,
              uint32_t *pfflags)
{
    uint32_t a_sign, b_sign, r_sign;
    int32_t a_exp, b_exp, r_exp;
    uint128_t a_mant, b_mant, r_mant, r_mant_low;

    void mul_sf128__part3203(uint128_t a, uint128_t b, uint32_t *a_sign, uint32_t *b_sign, uint32_t *r_sign, int32_t *a_exp, int32_t *b_exp, uint128_t *a_mant, uint128_t *b_mant);
    mul_sf128__part3203(a, b, &a_sign, &b_sign, &r_sign, &a_exp, &b_exp, &a_mant, &b_mant);

        {
        int mul_sf128__part3204(uint128_t a, uint128_t b, int32_t a_exp, int32_t b_exp, uint128_t a_mant, uint128_t b_mant, uint32_t r_sign, uint32_t *pfflags, uint128_t *pout);
        uint128_t r_special;
        if (mul_sf128__part3204(a, b, a_exp, b_exp, a_mant, b_mant, r_sign, pfflags, &r_special)) {
            return r_special;
        }
    }

    {
    int mul_sf128__part3205(uint32_t r_sign, int32_t *pa_exp,
                            uint128_t *pa_mant, uint128_t *p_result);
    uint128_t part3205_result;
    if (mul_sf128__part3205(r_sign, &a_exp, &a_mant, &part3205_result))
        return part3205_result;
}

    {
    uint128_t mul_sf128_part3206_ret;
    int32_t mul_sf128_part3206_done;
    int32_t mul_sf128__part3206(int32_t *pa_exp, uint128_t *pa_mant, uint32_t r_sign, uint128_t *p_ret);
    mul_sf128_part3206_done = mul_sf128__part3206(&b_exp, &b_mant, r_sign, &mul_sf128_part3206_ret);
    if (mul_sf128_part3206_done)
        return mul_sf128_part3206_ret;
}
    uint128_t mul_sf128__part3207(uint32_t r_sign, int32_t a_exp, int32_t b_exp, uint128_t a_mant, uint128_t b_mant, RoundingModeEnum rm, uint32_t *pfflags);

    return mul_sf128__part3207(r_sign, a_exp, b_exp, a_mant, b_mant, rm, pfflags);

}


uint128_t fma_sf128(uint128_t a, uint128_t b, uint128_t c, RoundingModeEnum rm,
              uint32_t *pfflags)
{
    uint32_t a_sign, b_sign, c_sign, r_sign;
    int32_t a_exp, b_exp, c_exp, r_exp, shift;
    uint128_t a_mant, b_mant, c_mant, r_mant1, r_mant0, c_mant1, c_mant0, mask;

    void fma_sf128__part3209(uint128_t a, uint128_t b, uint128_t c,
                             uint32_t *a_sign, uint32_t *b_sign,
                             uint32_t *c_sign, uint32_t *r_sign,
                             int32_t *a_exp, int32_t *b_exp, int32_t *c_exp,
                             uint128_t *a_mant, uint128_t *b_mant,
                             uint128_t *c_mant);

    fma_sf128__part3209(a, b, c, &a_sign, &b_sign, &c_sign, &r_sign,
                        &a_exp, &b_exp, &c_exp, &a_mant, &b_mant, &c_mant);

        {
        uint128_t fma_sf128__part3210(uint128_t a, uint128_t b, uint128_t c,
                                      uint32_t *pfflags, int32_t *ptaken);
        int32_t part3210_taken;
        uint128_t part3210_val;

        part3210_val = fma_sf128__part3210(a, b, c, pfflags, &part3210_taken);
        if (part3210_taken)
            return part3210_val;
    }

        {
        int32_t fma_sf128__part3211(int32_t *pa_exp, uint128_t *pa_mant);
        int goto_mul_zero = fma_sf128__part3211(&a_exp, &a_mant);
        if (goto_mul_zero)
            goto mul_zero;
    }
        {
        int fma_sf128__part3212(int32_t *pb_exp, uint128_t *pb_mant, int32_t c_exp,
                                uint128_t *pc_mant, uint32_t c_sign, uint32_t *pr_sign,
                                uint32_t rm, int mul_zero, uint128_t *pc,
                                uint128_t *presult);
        int part_st = 0;
        uint128_t part_res = 0;
        int part_via_mul_zero = 0;
        goto part3212_entry;
    mul_zero:
        part_via_mul_zero = 1;
    part3212_entry:
        part_st = fma_sf128__part3212(&b_exp, &b_mant, c_exp, &c_mant, c_sign,
                                      &r_sign, (uint32_t)rm, part_via_mul_zero,
                                      &c, &part_res);
        if (part_st != 0)
            return part_res;
    }


        uint128_t fma_sf128__part3213(int32_t a_exp, int32_t b_exp, uint128_t a_mant, uint128_t b_mant, int32_t *r_exp, uint128_t *r_mant0);
    r_mant1 = fma_sf128__part3213(a_exp, b_exp, a_mant, b_mant, &r_exp, &r_mant0);

    void fma_sf128__part3214(uint128_t *r_mant1, uint128_t *r_mant0, int32_t *r_exp);
    fma_sf128__part3214(&r_mant1, &r_mant0, &r_exp);



    int fma_sf128__part3215(int32_t *pc_exp, uint128_t *pc_mant, uint32_t r_sign,
                        int32_t r_exp, uint128_t r_mant1, uint128_t r_mant0,
                        int rm, uint32_t *pfflags, uint128_t *p_result);
{
    uint128_t fma_sf128_part3215_val;
    if (fma_sf128__part3215(&c_exp, &c_mant, r_sign, r_exp, r_mant1, r_mant0,
                            (int)rm, pfflags, &fma_sf128_part3215_val))
        return fma_sf128_part3215_val;
}

        void fma_sf128__part3216(int32_t *c_exp, const uint128_t *c_mant,
                             uint128_t *c_mant1, uint128_t *c_mant0);
    fma_sf128__part3216(&c_exp, &c_mant, &c_mant1, &c_mant0);






        void fma_sf128__part3217(int32_t *r_exp, int32_t *c_exp,
                             uint128_t *r_mant1, uint128_t *c_mant1,
                             uint128_t *r_mant0, uint128_t *c_mant0,
                             uint32_t *r_sign, uint32_t *c_sign);
    fma_sf128__part3217(&r_exp, &c_exp, &r_mant1, &c_mant1,
                        &r_mant0, &c_mant0, &r_sign, &c_sign);


    int32_t fma_sf128__part3218(int32_t r_exp, int32_t c_exp);
    shift = fma_sf128__part3218(r_exp, c_exp);

        {
        void fma_sf128__part3219(int32_t shift, uint128_t *c_mant0, uint128_t *c_mant1, uint128_t *mask);
        fma_sf128__part3219(shift, &c_mant0, &c_mant1, &mask);
    }




        {
        uint32_t fma_sf128__part3220(uint32_t r_sign_in, uint32_t c_sign,
                                     uint128_t *r_mant0, uint128_t *r_mant1,
                                     uint128_t c_mant0, uint128_t c_mant1,
                                     uint32_t rm);
        r_sign = fma_sf128__part3220(r_sign, c_sign, &r_mant0, &r_mant1,
                                     c_mant0, c_mant1, (uint32_t)rm);
    }
    uint128_t fma_sf128__part3221(uint32_t a_sign, int a_exp, uint128_t a_mant1, uint128_t a_mant0, RoundingModeEnum rm, uint32_t *pfflags);
    return fma_sf128__part3221(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags);


}
static uint128_t divrem_u128(uint128_t *pr, uint128_t a1, uint128_t a0, uint128_t b)
{
    int i, qb, ab;

void divrem_u128__part3223(uint128_t a1, uint128_t b);
divrem_u128__part3223(a1, b);

    void divrem_u128__part3224(uint128_t *a1, uint128_t *a0, const uint128_t *b);
    divrem_u128__part3224(&a1, &a0, &b);
    uint128_t divrem_u128__part3225(uint128_t *pr, uint128_t a1, uint128_t a0);
    return divrem_u128__part3225(pr, a1, a0);
}



uint128_t div_sf128(uint128_t a, uint128_t b, RoundingModeEnum rm,
              uint32_t *pfflags)
{
    uint32_t a_sign, b_sign, r_sign;
    int32_t a_exp, b_exp, r_exp;
    uint128_t a_mant, b_mant, r_mant, r;

    void div_sf128__part3227(uint128_t a, uint128_t b,
                             uint32_t *a_sign, uint32_t *b_sign,
                             uint32_t *r_sign, int32_t *a_exp, int32_t *b_exp,
                             uint128_t *a_mant, uint128_t *b_mant);
    div_sf128__part3227(a, b, &a_sign, &b_sign, &r_sign, &a_exp, &b_exp,
                        &a_mant, &b_mant);

        {
        uint128_t _special_r;
        int div_sf128__part3228(uint128_t a, uint128_t b, int32_t a_exp,
                                uint128_t a_mant, int32_t b_exp, uint128_t b_mant,
                                uint32_t r_sign, uint32_t *pfflags,
                                uint128_t *presult);
        if (div_sf128__part3228(a, b, a_exp, a_mant, b_exp, b_mant, r_sign,
                                pfflags, &_special_r))
            return _special_r;
    }


        {
        uint128_t part3229_val;
        extern int div_sf128__part3229(uint32_t r_sign, int32_t a_exp, uint128_t a_mant,
                                       int32_t *pb_exp, uint128_t *pb_mant,
                                       uint32_t *pfflags, uint128_t *presult);
        if (div_sf128__part3229(r_sign, a_exp, a_mant, &b_exp, &b_mant,
                                pfflags, &part3229_val))
            return part3229_val;
    }

        {
        int div_sf128__part3230(int32_t *a_exp, uint128_t *a_mant, uint32_t r_sign, uint128_t *pret);
        uint128_t part_ret;
        if (div_sf128__part3230(&a_exp, &a_mant, r_sign, &part_ret))
            return part_ret;
    }

        {
        void div_sf128__part3231(int32_t *pr_exp, uint128_t *pr_mant, uint128_t *pr_rem,
                                 int32_t a_exp, int32_t b_exp, uint128_t a_mant,
                                 uint128_t b_mant);
        int32_t r_exp__out;
        uint128_t r_mant__out, r__out;
        div_sf128__part3231(&r_exp__out, &r_mant__out, &r__out,
                            a_exp, b_exp, a_mant, b_mant);
        r_exp = r_exp__out;
        r_mant = r_mant__out;
        r = r__out;
    }

    uint128_t div_sf128__part3232(uint128_t r, uint128_t r_mant);
    r_mant = div_sf128__part3232(r, r_mant);
        {
        extern uint128_t div_sf128__part3233(uint32_t r_sign, int32_t r_exp,
                                             uint128_t r_mant, RoundingModeEnum rm,
                                             uint32_t *pfflags);
        return div_sf128__part3233(r_sign, r_exp, r_mant, rm, pfflags);
    }
}
static int sqrtrem_u128(uint128_t *pr, uint128_t a1, uint128_t a0)
{
    int l, inexact;
    uint128_t u, s, r, q, sq0, sq1;


    {
        int _sqrtrem_early;
        extern int sqrtrem_u128__part3235(uint128_t a1, uint128_t a0, int *pl, uint128_t *pr);
        _sqrtrem_early = sqrtrem_u128__part3235(a1, a0, &l, pr);
        if (_sqrtrem_early)
            return 0;
    }

    extern uint128_t sqrtrem_u128__part3236(int l);
    u = sqrtrem_u128__part3236(l);
        uint128_t sqrtrem_u128__part3237(uint128_t u, uint128_t a1, uint128_t a0);
    s = sqrtrem_u128__part3237(u, a1, a0);

        {
        int sqrtrem_u128__part3238(uint128_t *pr, uint128_t a1, uint128_t a0, uint128_t s);
        inexact = sqrtrem_u128__part3238(pr, a1, a0, s);
    }
    return inexact;

}



uint128_t sqrt_sf128(uint128_t a, RoundingModeEnum rm,
               uint32_t *pfflags)
{
    void sqrt_sf128__part3240(const uint128_t *a, uint32_t *a_sign, int32_t *a_exp, uint128_t *a_mant);
    uint32_t a_sign;
    int32_t a_exp;
    uint128_t a_mant;

    sqrt_sf128__part3240(&a, &a_sign, &a_exp, &a_mant);

    {
        uint128_t res_part3241;
        int32_t status_part3241;
        uint128_t sqrt_sf128__part3241(uint128_t a, uint32_t a_sign, int32_t a_exp, uint128_t a_mant, uint32_t *pfflags, int32_t *pstatus);
        res_part3241 = sqrt_sf128__part3241(a, a_sign, a_exp, a_mant, pfflags, &status_part3241);
        if (status_part3241 == 1)
            return res_part3241;
        if (status_part3241 == 2)
            goto neg_error;
    }
        uint128_t sqrt_sf128__part3242(uint32_t *pfflags);
    if (a_sign) {
        if (a_exp == 0 && a_mant == 0)
            return a;
    neg_error:
        return sqrt_sf128__part3242(pfflags);
    }

    int sqrt_sf128__part3243(int32_t *pa_exp, uint128_t *pa_mant);
    if (sqrt_sf128__part3243(&a_exp, &a_mant))
        return pack_sf128(0, 0, 0);
    int32_t sqrt_sf128__part3244(int32_t a_exp);
a_exp = sqrt_sf128__part3244(a_exp);


    void sqrt_sf128__part3245(int32_t *a_exp, uint128_t *a_mant);
    sqrt_sf128__part3245(&a_exp, &a_mant);

    void sqrt_sf128__part3246(int32_t *a_exp, uint128_t *a_mant);
    sqrt_sf128__part3246(&a_exp, &a_mant);
    void sqrt_sf128__part3247(uint128_t *pa_mant);
    sqrt_sf128__part3247(&a_mant);

        uint128_t sqrt_sf128__part3248(uint32_t a_sign, int32_t a_exp, uint128_t a_mant, uint32_t rm, uint32_t *pfflags);
    return sqrt_sf128__part3248(a_sign, a_exp, a_mant, (uint32_t)rm, pfflags);
}



uint128_t min_sf128(uint128_t a, uint128_t b, uint32_t *pfflags)
{
    uint32_t a_sign, b_sign;

    int min_sf128__part3250(uint128_t a, uint128_t b, uint32_t *pfflags, uint128_t *presult);
    {
        uint128_t min_sf128__result;
        if (min_sf128__part3250(a, b, pfflags, &min_sf128__result))
            return min_sf128__result;
    }

    void min_sf128__part3251(uint128_t a, uint128_t b, uint32_t *a_sign, uint32_t *b_sign);
    min_sf128__part3251(a, b, &a_sign, &b_sign);


    {
    uint32_t min_sf128__part3252(const uint128_t *a, const uint128_t *b, uint32_t a_sign, uint32_t b_sign);
    uint32_t pick_a = min_sf128__part3252(&a, &b, a_sign, b_sign);
    if (pick_a)
        return a;
    else
        return b;
}

}

uint128_t max_sf128(uint128_t a, uint128_t b, uint32_t *pfflags)
{
    uint32_t a_sign, b_sign;

        {
        int max_sf128__part3254(uint128_t a, uint128_t b, uint32_t *pfflags, uint128_t *pval);
        uint128_t max_sf128__r;
        if (max_sf128__part3254(a, b, pfflags, &max_sf128__r))
            return max_sf128__r;
    }

    void max_sf128__part3255(uint128_t a, uint128_t b, uint32_t *a_sign, uint32_t *b_sign);
    max_sf128__part3255(a, b, &a_sign, &b_sign);


        {
        uint128_t max_sf128__part3256(uint128_t a, uint128_t b, uint32_t a_sign, uint32_t b_sign);
        return max_sf128__part3256(a, b, a_sign, b_sign);
    }

}

int eq_quiet_sf128(uint128_t a, uint128_t b, uint32_t *pfflags)
{
    int eq_quiet_sf128__part3258(uint128_t a, uint128_t b, uint32_t *pfflags);
    if (eq_quiet_sf128__part3258(a, b, pfflags)) {
        return 0;
    }


    int eq_quiet_sf128__part3259(uint128_t a, uint128_t b);
    if (eq_quiet_sf128__part3259(a, b))
        return 1;

        {
        int eq_quiet_sf128__part3260(uint128_t a, uint128_t b);
        return eq_quiet_sf128__part3260(a, b);
    }

}

int le_sf128(uint128_t a, uint128_t b, uint32_t *pfflags)
{
    uint32_t a_sign, b_sign;

        int le_sf128__part3262(uint128_t a, uint128_t b, uint32_t *pfflags);
    if (le_sf128__part3262(a, b, pfflags)) {
        return 0;
    }


    void le_sf128__part3263(uint128_t a, uint128_t b, uint32_t *a_sign, uint32_t *b_sign);
    le_sf128__part3263(a, b, &a_sign, &b_sign);
        {
        int le_sf128__part3264(uint128_t a, uint128_t b, uint32_t a_sign, uint32_t b_sign);
        return le_sf128__part3264(a, b, a_sign, b_sign);
    }

}

int lt_sf128(uint128_t a, uint128_t b, uint32_t *pfflags)
{
    uint32_t a_sign, b_sign;

    int lt_sf128__part3266(uint128_t a, uint128_t b, uint32_t *pfflags);
    if (lt_sf128__part3266(a, b, pfflags)) {
        return 0;
    }


    void lt_sf128__part3267(uint128_t a, uint128_t b, uint32_t *a_sign, uint32_t *b_sign);
    lt_sf128__part3267(a, b, &a_sign, &b_sign);

        int lt_sf128__part3268(const uint128_t *pa, const uint128_t *pb, uint32_t a_sign, uint32_t b_sign);
    return lt_sf128__part3268(&a, &b, a_sign, b_sign);

}

uint32_t fclass_sf128(uint128_t a)
{
    void fclass_sf128__part3270(uint128_t a, uint32_t *a_sign, int32_t *a_exp, uint128_t *a_mant);
    uint32_t a_sign;
    int32_t a_exp;
    uint128_t a_mant;
    uint32_t ret;

    fclass_sf128__part3270(a, &a_sign, &a_exp, &a_mant);
        uint32_t fclass_sf128__part3271(uint32_t a_sign, int32_t a_exp, uint128_t a_mant);
    ret = fclass_sf128__part3271(a_sign, a_exp, a_mant);

    uint32_t fclass_sf128__part3272(uint32_t ret);
    return fclass_sf128__part3272(ret);
}





uint128_t cvt_sf32_sf128(uint32_t a, uint32_t *pfflags)
{
    void cvt_sf32_sf128__part3274(uint32_t *pa_sign, int32_t *pa_exp, uint128_t *pa_mant, uint32_t a);
    uint32_t a_sign;
    int32_t a_exp;
    uint128_t a_mant;

    cvt_sf32_sf128__part3274(&a_sign, &a_exp, &a_mant, a);

    uint128_t cvt_sf32_sf128__part3275(uint32_t a, uint32_t a_sign, uint128_t a_mant, uint32_t *pfflags);
    if (a_exp == 0xff) {
        return cvt_sf32_sf128__part3275(a, a_sign, a_mant, pfflags);
    }
    {
        extern int cvt_sf32_sf128__part3276(uint32_t a_sign, int32_t *pa_exp, uint128_t *pa_mant, uint128_t *p_ret);
        uint128_t part3276_ret = 0;
        if (cvt_sf32_sf128__part3276(a_sign, &a_exp, &a_mant, &part3276_ret))
            return part3276_ret;
    }


        uint128_t cvt_sf32_sf128__part3277(uint32_t a_sign, int32_t a_exp, uint128_t a_mant);
    return cvt_sf32_sf128__part3277(a_sign, a_exp, a_mant);

}

uint32_t cvt_sf128_sf32(uint128_t a, RoundingModeEnum rm,
                                           uint32_t *pfflags)
{
    void cvt_sf128_sf32__part3279(const uint128_t *pa, uint32_t *pa_sign, int32_t *pa_exp, uint128_t *pa_mant);

    uint32_t a_sign;
    int32_t a_exp;
    uint128_t a_mant;

    cvt_sf128_sf32__part3279(&a, &a_sign, &a_exp, &a_mant);

        {
        uint64_t _r3280;
        extern uint64_t cvt_sf128_sf32__part3280(uint32_t, int32_t, uint64_t,
                                                 uint64_t, uint64_t, uint64_t,
                                                 uint32_t *);
        _r3280 = cvt_sf128_sf32__part3280(a_sign, a_exp,
                                          (uint64_t)a_mant, (uint64_t)(a_mant >> 64),
                                          (uint64_t)a, (uint64_t)(a >> 64), pfflags);
        if (_r3280 >> 32)
            return (uint32_t)_r3280;
    }

        {
        int part3281_ret;
        uint32_t part3281_val;
        int cvt_sf128_sf32__part3281(int32_t *pa_exp, uint128_t *pa_mant,
                                     uint32_t a_sign, uint32_t *p_ret);
        part3281_ret = cvt_sf128_sf32__part3281(&a_exp, &a_mant, a_sign,
                                                &part3281_val);
        if (part3281_ret)
            return part3281_val;
    }


    uint32_t cvt_sf128_sf32__part3282(uint32_t a_sign, int32_t a_exp, uint128_t a_mant, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_sf128_sf32__part3282(a_sign, a_exp, a_mant, rm, pfflags);
}





uint128_t cvt_sf64_sf128(uint64_t a, uint32_t *pfflags)
{
        uint32_t a_sign;
    int32_t a_exp;
    uint128_t a_mant;

    uint128_t cvt_sf64_sf128__part3284(uint64_t a, uint32_t *pa_sign, int32_t *pa_exp);
    a_mant = cvt_sf64_sf128__part3284(a, &a_sign, &a_exp);


    int cvt_sf64_sf128__part3285(uint32_t a_sign, int32_t a_exp, const uint128_t *a_mant, uint64_t a, uint32_t *pfflags, uint128_t *presult);
    {
        uint128_t __part3285_result;
        int __part3285_taken = cvt_sf64_sf128__part3285(a_sign, a_exp, &a_mant, a, pfflags, &__part3285_result);
        if (__part3285_taken) {
            return __part3285_result;
        }
    }
    {
        int cvt_part_ret;
        int cvt_sf64_sf128__part3286(uint32_t a_sign, int32_t *pa_exp, uint128_t *pa_mant);
        cvt_part_ret = cvt_sf64_sf128__part3286(a_sign, &a_exp, &a_mant);
        if (cvt_part_ret)
            return a_mant;
    }


        uint128_t cvt_sf64_sf128__part3287(uint32_t a_sign, int32_t a_exp, uint128_t a_mant);
    return cvt_sf64_sf128__part3287(a_sign, a_exp, a_mant);

}

uint64_t cvt_sf128_sf64(uint128_t a, RoundingModeEnum rm,
                                               uint32_t *pfflags)
{
    uint32_t a_sign;
    int32_t a_exp;
    uint128_t a_mant;

    void cvt_sf128_sf64__part3289(uint32_t *pa_sign, int32_t *pa_exp,
                                  uint128_t *pa_mant, const uint128_t *pa);
    cvt_sf128_sf64__part3289(&a_sign, &a_exp, &a_mant, &a);

    {
        extern uint64_t cvt_sf128_sf64__part3290(uint128_t a, uint32_t a_sign, int32_t a_exp, uint128_t a_mant, uint32_t *pfflags, int *done);
        int part3290_done;
        uint64_t part3290_res = cvt_sf128_sf64__part3290(a, a_sign, a_exp, a_mant, pfflags, &part3290_done);
        if (part3290_done) {
            return part3290_res;
        }
    }
        {
        uint64_t cvt_sf128_sf64__part3291(uint32_t a_sign, int32_t *a_exp, uint128_t *a_mant, uint64_t *ret);
        uint64_t part3291_ret = 0;
        if (cvt_sf128_sf64__part3291(a_sign, &a_exp, &a_mant, &part3291_ret))
            return part3291_ret;
    }


        {
        uint64_t cvt_sf128_sf64__part3292(uint32_t a_sign, int32_t a_exp, uint128_t a_mant, RoundingModeEnum rm, uint32_t *pfflags);
        return cvt_sf128_sf64__part3292(a_sign, a_exp, a_mant, rm, pfflags);
    }

}






static int32_t internal_cvt_sf128_i32(uint128_t a, RoundingModeEnum rm,
                                                                         uint32_t *pfflags, BOOL is_unsigned)
{
    void internal_cvt_sf128_i32__part3427(uint128_t a, uint32_t *pa_sign, int32_t *pa_exp, uint128_t *pa_mant);
    uint32_t a_sign, addend, rnd_bits;
    int32_t a_exp;
    uint128_t a_mant;
    uint32_t r, r_max;

    internal_cvt_sf128_i32__part3427(a, &a_sign, &a_exp, &a_mant);
    void internal_cvt_sf128_i32__part3428(int32_t a_exp, uint128_t a_mant, uint32_t *a_sign);
    internal_cvt_sf128_i32__part3428(a_exp, a_mant, &a_sign);
        void internal_cvt_sf128_i32__part3429(int32_t *a_exp, uint128_t *a_mant);
    internal_cvt_sf128_i32__part3429(&a_exp, &a_mant);

    void internal_cvt_sf128_i32__part3430(uint128_t *a_mant, int32_t *a_exp);
    internal_cvt_sf128_i32__part3430(&a_mant, &a_exp);

        uint32_t internal_cvt_sf128_i32__part3431(uint32_t a_sign, BOOL is_unsigned);
    r_max = internal_cvt_sf128_i32__part3431(a_sign, is_unsigned);

        {
        int32_t internal_cvt_sf128_i32__part3432(int32_t a_exp, uint128_t a_mant, uint32_t a_sign, int rm, uint32_t r_max, uint32_t *pfflags, uint32_t *r_out);
        if (internal_cvt_sf128_i32__part3432(a_exp, a_mant, a_sign, rm, r_max, pfflags, &r))
            return r_max;
    }

    uint32_t internal_cvt_sf128_i32__part3433(uint32_t a_sign, uint32_t r);
    r = internal_cvt_sf128_i32__part3433(a_sign, r);

    int32_t internal_cvt_sf128_i32__part3434(uint32_t r);
    return internal_cvt_sf128_i32__part3434(r);

}

int32_t cvt_sf128_i32(uint128_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
    extern int32_t cvt_sf128_i32__part3436(uint128_t a, RoundingModeEnum rm,
                                       uint32_t *pfflags);
return cvt_sf128_i32__part3436(a, rm, pfflags);

}

uint32_t cvt_sf128_u32(uint128_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
        {
        uint32_t cvt_sf128_u32__part3438(uint128_t a, RoundingModeEnum rm, uint32_t *pfflags);
        return cvt_sf128_u32__part3438(a, rm, pfflags);
    }

}


static uint128_t internal_cvt_i32_sf128(int32_t a,
                                                                       RoundingModeEnum rm,
                                                                       uint32_t *pfflags,
                                                                       BOOL is_unsigned)
{
    uint32_t a_sign;
    int32_t a_exp;
    uint128_t a_mant;
    uint32_t r, mask;
    int l;

    void internal_cvt_i32_sf128__part3440(int32_t a, BOOL is_unsigned, uint32_t *out_a_sign, uint32_t *out_r);
    internal_cvt_i32_sf128__part3440(a, is_unsigned, &a_sign, &r);

    void internal_cvt_i32_sf128__part3441(int32_t *a_exp, uint32_t r, int32_t *l);
    internal_cvt_i32_sf128__part3441(&a_exp, r, &l);

        void internal_cvt_i32_sf128__part3442(int l, uint32_t *r, uint32_t *mask, int32_t *a_exp);
    internal_cvt_i32_sf128__part3442(l, &r, &mask, &a_exp);

        a_mant = r;
    {
        uint128_t internal_cvt_i32_sf128__part3443(uint32_t a_sign, int32_t a_exp, uint128_t a_mant, RoundingModeEnum rm, uint32_t *pfflags);
        return internal_cvt_i32_sf128__part3443(a_sign, a_exp, a_mant, rm, pfflags);
    }

}

uint128_t cvt_i32_sf128(int32_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
    uint128_t cvt_i32_sf128__part3445(int32_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_i32_sf128__part3445(a, rm, pfflags);

}

uint128_t cvt_u32_sf128(uint32_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
    uint128_t cvt_u32_sf128__part3447(uint32_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_u32_sf128__part3447(a, rm, pfflags);

}


static int64_t internal_cvt_sf128_i64(uint128_t a, RoundingModeEnum rm,
                                                                         uint32_t *pfflags, BOOL is_unsigned)
{
    void internal_cvt_sf128_i64__part3449(uint128_t a, uint32_t *a_sign, int32_t *a_exp, uint128_t *a_mant);

uint32_t a_sign, addend, rnd_bits;
    int32_t a_exp;
    uint128_t a_mant;
    uint64_t r, r_max;

    internal_cvt_sf128_i64__part3449(a, &a_sign, &a_exp, &a_mant);
    void internal_cvt_sf128_i64__part3450(int32_t a_exp, uint128_t a_mant, uint32_t *a_sign);
    internal_cvt_sf128_i64__part3450(a_exp, a_mant, &a_sign);
    void internal_cvt_sf128_i64__part3451(int32_t *pa_exp, uint128_t *pa_mant);
    internal_cvt_sf128_i64__part3451(&a_exp, &a_mant);

    void internal_cvt_sf128_i64__part3452(uint128_t *a_mant, int32_t *a_exp);
    internal_cvt_sf128_i64__part3452(&a_mant, &a_exp);


    uint64_t internal_cvt_sf128_i64__part3453(uint32_t a_sign, int is_unsigned);
    r_max = internal_cvt_sf128_i64__part3453(a_sign, is_unsigned);

        {
        int part3454_ovf;
        int internal_cvt_sf128_i64__part3454(int32_t a_exp, uint128_t a_mant, uint64_t r_max, RoundingModeEnum rm, uint32_t a_sign, uint32_t *pfflags, uint64_t *pr);
        part3454_ovf = internal_cvt_sf128_i64__part3454(a_exp, a_mant, r_max, rm, a_sign, pfflags, &r);
        if (part3454_ovf)
            return r_max;
    }
        {
        uint64_t internal_cvt_sf128_i64__part3455(uint32_t a_sign, uint64_t r);
        r = internal_cvt_sf128_i64__part3455(a_sign, r);
    }

    int64_t internal_cvt_sf128_i64__part3456(uint64_t r, uint32_t a_sign);
    return internal_cvt_sf128_i64__part3456(r, a_sign);

}

int64_t cvt_sf128_i64(uint128_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
    int64_t cvt_sf128_i64__part3458(uint128_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_sf128_i64__part3458(a, rm, pfflags);
}

uint64_t cvt_sf128_u64(uint128_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
    uint64_t cvt_sf128_u64__part3460(uint128_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_sf128_u64__part3460(a, rm, pfflags);
}


static uint128_t internal_cvt_i64_sf128(int64_t a,
                                                                       RoundingModeEnum rm,
                                                                       uint32_t *pfflags,
                                                                       BOOL is_unsigned)
{
    uint32_t a_sign;
    int32_t a_exp;
    uint128_t a_mant;
    uint64_t r, mask;
    int l;

    void internal_cvt_i64_sf128__part3462(int64_t a, int is_unsigned, uint32_t *a_sign, uint64_t *r);
    internal_cvt_i64_sf128__part3462(a, is_unsigned, &a_sign, &r);

        void internal_cvt_i64_sf128__part3463(uint64_t r, int32_t *a_exp, int *l);
    internal_cvt_i64_sf128__part3463(r, &a_exp, &l);

    extern void internal_cvt_i64_sf128__part3464(uint64_t *r, int32_t *a_exp, int l);
    internal_cvt_i64_sf128__part3464(&r, &a_exp, l);
        uint128_t internal_cvt_i64_sf128__part3465(uint32_t a_sign, int a_exp, uint64_t r, int rm, uint32_t *pfflags);
    return internal_cvt_i64_sf128__part3465(a_sign, a_exp, r, (int)rm, pfflags);

}

uint128_t cvt_i64_sf128(int64_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
    uint128_t cvt_i64_sf128__part3467(int64_t a, int rm, uint32_t *pfflags);
    return cvt_i64_sf128__part3467(a, rm, pfflags);
}

uint128_t cvt_u64_sf128(uint64_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
    uint128_t cvt_u64_sf128__part3469(uint64_t a, uint32_t rm, uint32_t *pfflags);
return cvt_u64_sf128__part3469(a, rm, pfflags);

}



static int128_t internal_cvt_sf128_i128(uint128_t a, RoundingModeEnum rm,
                                                                         uint32_t *pfflags, BOOL is_unsigned)
{
    uint32_t a_sign, addend, rnd_bits;
    int32_t a_exp;
    uint128_t a_mant;
    uint128_t r, r_max;

    void internal_cvt_sf128_i128__part3471(uint128_t a, uint32_t *a_sign, int32_t *a_exp, uint128_t *a_mant);
    internal_cvt_sf128_i128__part3471(a, &a_sign, &a_exp, &a_mant);

    uint32_t internal_cvt_sf128_i128__part3472(int32_t a_exp, uint128_t a_mant, uint32_t a_sign);
    a_sign = internal_cvt_sf128_i128__part3472(a_exp, a_mant, a_sign);

    void internal_cvt_sf128_i128__part3473(int32_t *a_exp, uint128_t *a_mant);
    internal_cvt_sf128_i128__part3473(&a_exp, &a_mant);

        void internal_cvt_sf128_i128__part3474(uint128_t *a_mant, int32_t *a_exp);
    internal_cvt_sf128_i128__part3474(&a_mant, &a_exp);


        uint128_t internal_cvt_sf128_i128__part3475(uint32_t a_sign, int is_unsigned);
    r_max = internal_cvt_sf128_i128__part3475(a_sign, is_unsigned);

        {
        extern int internal_cvt_sf128_i128__part3476(uint128_t part_a_mant,
                                                     int32_t part_a_exp,
                                                     uint128_t part_r_max,
                                                     uint32_t part_a_sign,
                                                     int part_rm,
                                                     uint32_t *part_pfflags,
                                                     uint128_t *part_pr);
        uint128_t part_r = 0;

        if (internal_cvt_sf128_i128__part3476(a_mant, a_exp, r_max, a_sign,
                                              (int)rm, pfflags, &part_r)) {
            return r_max;
        }
        r = part_r;
    }

    {
        uint128_t internal_cvt_sf128_i128__part3477(uint32_t a_sign, uint128_t r);
        r = internal_cvt_sf128_i128__part3477(a_sign, r);
    }

    int128_t internal_cvt_sf128_i128__part3478(uint128_t r);
    return internal_cvt_sf128_i128__part3478(r);

}

int128_t cvt_sf128_i128(uint128_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
        {
        int128_t cvt_sf128_i128__part3480(uint128_t a, RoundingModeEnum rm, uint32_t *pfflags);
        return cvt_sf128_i128__part3480(a, rm, pfflags);
    }

}

uint128_t cvt_sf128_u128(uint128_t a, RoundingModeEnum rm,
                                                          uint32_t *pfflags)
{
    extern uint128_t cvt_sf128_u128__part3482(uint128_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_sf128_u128__part3482(a, rm, pfflags);
}


static uint128_t internal_cvt_i128_sf128(int128_t a,
                                                                       RoundingModeEnum rm,
                                                                       uint32_t *pfflags,
                                                                       BOOL is_unsigned)
{
    uint32_t a_sign;
    int32_t a_exp;
    uint128_t a_mant;
    uint128_t r, mask;
    int l;

        void internal_cvt_i128_sf128__part3484(BOOL is_unsigned, int128_t a, uint32_t *a_sign, uint128_t *r);
    internal_cvt_i128_sf128__part3484(is_unsigned, a, &a_sign, &r);

    int internal_cvt_i128_sf128__part3485(uint128_t r, int32_t *pa_exp);

    l = internal_cvt_i128_sf128__part3485(r, &a_exp);

    uint128_t internal_cvt_i128_sf128__part3486(int l, uint128_t r, int32_t *pa_exp);
    r = internal_cvt_i128_sf128__part3486(l, r, &a_exp);

    uint128_t internal_cvt_i128_sf128__part3487(uint32_t a_sign, int32_t a_exp, uint128_t r, RoundingModeEnum rm, uint32_t *pfflags);
    a_mant = r;
    return internal_cvt_i128_sf128__part3487(a_sign, a_exp, r, rm, pfflags);

}

uint128_t cvt_i128_sf128(int128_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
    uint128_t cvt_i128_sf128__part3489(int128_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_i128_sf128__part3489(a, rm, pfflags);
}

uint128_t cvt_u128_sf128(uint128_t a,
                                                       RoundingModeEnum rm,
                                                       uint32_t *pfflags)
{
        uint128_t cvt_u128_sf128__part3491(uint128_t a, RoundingModeEnum rm, uint32_t *pfflags);
    return cvt_u128_sf128__part3491(a, rm, pfflags);

}
