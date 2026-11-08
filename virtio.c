





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
typedef void DeviceWriteFunc(void *opaque, uint32_t offset,
                             uint32_t val, int size_log2);
typedef uint32_t DeviceReadFunc(void *opaque, uint32_t offset, int size_log2);
typedef struct PhysMemoryMap PhysMemoryMap;

typedef struct {
    PhysMemoryMap *map;
    uint64_t addr;
    uint64_t org_size;
    uint64_t size;
    BOOL is_ram;

    int devram_flags;
    uint8_t *phys_mem;
    int dirty_bits_size;
    uint32_t *dirty_bits;
    uint32_t *dirty_bits_tab[2];
    int dirty_bits_index;

    void *opaque;
    DeviceReadFunc *read_func;
    DeviceWriteFunc *write_func;
    int devio_flags;
} PhysMemoryRange;



struct PhysMemoryMap {
    int n_phys_mem_range;
    PhysMemoryRange phys_mem_range[32];
    PhysMemoryRange *(*register_ram)(PhysMemoryMap *s, uint64_t addr,
                                     uint64_t size, int devram_flags);
    void (*free_ram)(PhysMemoryMap *s, PhysMemoryRange *pr);
    const uint32_t *(*get_dirty_bits)(PhysMemoryMap *s, PhysMemoryRange *pr);
    void (*set_ram_addr)(PhysMemoryMap *s, PhysMemoryRange *pr, uint64_t addr,
                         BOOL enabled);
    void *opaque;
    void (*flush_tlb_write_range)(void *opaque, uint8_t *ram_addr,
                                  size_t ram_size);
};


PhysMemoryMap *phys_mem_map_init(void);
void phys_mem_map_end(PhysMemoryMap *s);
PhysMemoryRange *register_ram_entry(PhysMemoryMap *s, uint64_t addr,
                                    uint64_t size, int devram_flags);
static inline PhysMemoryRange *cpu_register_ram(PhysMemoryMap *s, uint64_t addr,
                                  uint64_t size, int devram_flags)
{
    return s->register_ram(s, addr, size, devram_flags);
}
PhysMemoryRange *cpu_register_device(PhysMemoryMap *s, uint64_t addr,
                                     uint64_t size, void *opaque,
                                     DeviceReadFunc *read_func, DeviceWriteFunc *write_func,
                                     int devio_flags);
PhysMemoryRange *get_phys_mem_range(PhysMemoryMap *s, uint64_t paddr);
void phys_mem_set_addr(PhysMemoryRange *pr, uint64_t addr, BOOL enabled);

static inline const uint32_t *phys_mem_get_dirty_bits(PhysMemoryRange *pr)
{
    PhysMemoryMap *map = pr->map;
    return map->get_dirty_bits(map, pr);
}

static inline void phys_mem_set_dirty_bit(PhysMemoryRange *pr, size_t offset)
{
    size_t page_index;
    uint32_t mask, *dirty_bits_ptr;
    if (pr->dirty_bits) {
        page_index = offset >> 12;
        mask = 1 << (page_index & 0x1f);
        dirty_bits_ptr = pr->dirty_bits + (page_index >> 5);
        *dirty_bits_ptr |= mask;
    }
}

static inline BOOL phys_mem_is_dirty_bit(PhysMemoryRange *pr, size_t offset)
{
    size_t page_index;
    uint32_t *dirty_bits_ptr;
    if (!pr->dirty_bits)
        return TRUE;
    page_index = offset >> 12;
    dirty_bits_ptr = pr->dirty_bits + (page_index >> 5);
    return (*dirty_bits_ptr >> (page_index & 0x1f)) & 1;
}

void phys_mem_reset_dirty_bit(PhysMemoryRange *pr, size_t offset);
uint8_t *phys_mem_get_ram_ptr(PhysMemoryMap *map, uint64_t paddr, BOOL is_rw);



typedef void SetIRQFunc(void *opaque, int irq_num, int level);

typedef struct {
    SetIRQFunc *set_irq;
    void *opaque;
    int irq_num;
} IRQSignal;

void irq_init(IRQSignal *irq, SetIRQFunc *set_irq, void *opaque, int irq_num);

static inline void set_irq(IRQSignal *irq, int level)
{
    irq->set_irq(irq->opaque, irq->irq_num, level);
}
typedef struct PCIBus PCIBus;
typedef struct PCIDevice PCIDevice;
typedef void PCIBarSetFunc(void *opaque, int bar_num, uint32_t addr,
                           BOOL enabled);

PCIDevice *pci_register_device(PCIBus *b, const char *name, int devfn,
                               uint16_t vendor_id, uint16_t device_id,
                               uint8_t revision, uint16_t class_id);
PhysMemoryMap *pci_device_get_mem_map(PCIDevice *d);
PhysMemoryMap *pci_device_get_port_map(PCIDevice *d);
void pci_register_bar(PCIDevice *d, unsigned int bar_num,
                      uint32_t size, int type,
                      void *opaque, PCIBarSetFunc *bar_set);
IRQSignal *pci_device_get_irq(PCIDevice *d, unsigned int irq_num);
uint8_t *pci_device_get_dma_ptr(PCIDevice *d, uint64_t addr, BOOL is_rw);
void pci_device_set_config8(PCIDevice *d, uint8_t addr, uint8_t val);
void pci_device_set_config16(PCIDevice *d, uint8_t addr, uint16_t val);
int pci_device_get_devfn(PCIDevice *d);
int pci_add_capability(PCIDevice *d, const uint8_t *buf, int size);

typedef struct I440FXState I440FXState;

I440FXState *i440fx_init(PCIBus **pbus, int *ppiix3_devfn,
                         PhysMemoryMap *mem_map, PhysMemoryMap *port_map,
                         IRQSignal *pic_irqs);
void i440fx_map_interrupts(I440FXState *s, uint8_t *elcr,
                           const uint8_t *pci_irqs);
typedef uint64_t virtio_phys_addr_t;




typedef struct {

    PCIBus *pci_bus;

    PhysMemoryMap *mem_map;
    uint64_t addr;
    IRQSignal *irq;
} VIRTIOBusDef;

typedef struct VIRTIODevice VIRTIODevice;




void virtio_set_debug(VIRTIODevice *s, int debug_flags);



typedef void BlockDeviceCompletionFunc(void *opaque, int ret);

typedef struct BlockDevice BlockDevice;

struct BlockDevice {
    int64_t (*get_sector_count)(BlockDevice *bs);
    int (*read_async)(BlockDevice *bs,
                      uint64_t sector_num, uint8_t *buf, int n,
                      BlockDeviceCompletionFunc *cb, void *opaque);
    int (*write_async)(BlockDevice *bs,
                       uint64_t sector_num, const uint8_t *buf, int n,
                       BlockDeviceCompletionFunc *cb, void *opaque);
    void *opaque;
};

VIRTIODevice *virtio_block_init(VIRTIOBusDef *bus, BlockDevice *bs);



typedef struct EthernetDevice EthernetDevice;

struct EthernetDevice {
    uint8_t mac_addr[6];
    void (*write_packet)(EthernetDevice *net,
                         const uint8_t *buf, int len);
    void *opaque;

    void (*select_fill)(EthernetDevice *net, int *pfd_max,
                        fd_set *rfds, fd_set *wfds, fd_set *efds,
                        int *pdelay);
    void (*select_poll)(EthernetDevice *net,
                        fd_set *rfds, fd_set *wfds, fd_set *efds,
                        int select_ret);


    void *device_opaque;
    BOOL (*device_can_write_packet)(EthernetDevice *net);
    void (*device_write_packet)(EthernetDevice *net,
                                const uint8_t *buf, int len);
    void (*device_set_carrier)(EthernetDevice *net, BOOL carrier_state);
};

VIRTIODevice *virtio_net_init(VIRTIOBusDef *bus, EthernetDevice *es);



typedef struct {
    void *opaque;
    void (*write_data)(void *opaque, const uint8_t *buf, int len);
    int (*read_data)(void *opaque, uint8_t *buf, int len);
} CharacterDevice;

VIRTIODevice *virtio_console_init(VIRTIOBusDef *bus, CharacterDevice *cs);
BOOL virtio_console_can_write_data(VIRTIODevice *s);
int virtio_console_get_write_len(VIRTIODevice *s);
int virtio_console_write_data(VIRTIODevice *s, const uint8_t *buf, int buf_len);
void virtio_console_resize_event(VIRTIODevice *s, int width, int height);



typedef enum {
    VIRTIO_INPUT_TYPE_KEYBOARD,
    VIRTIO_INPUT_TYPE_MOUSE,
    VIRTIO_INPUT_TYPE_TABLET,
} VirtioInputTypeEnum;



int virtio_input_send_key_event(VIRTIODevice *s, BOOL is_down,
                                uint16_t key_code);
int virtio_input_send_mouse_event(VIRTIODevice *s, int dx, int dy, int dz,
                                  unsigned int buttons);

VIRTIODevice *virtio_input_init(VIRTIOBusDef *bus, VirtioInputTypeEnum type);



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

VIRTIODevice *virtio_9p_init(VIRTIOBusDef *bus, FSDevice *fs,
                             const char *mount_tag);
typedef struct {
    uint32_t ready;
    uint32_t num;
    uint16_t last_avail_idx;
    virtio_phys_addr_t desc_addr;
    virtio_phys_addr_t avail_addr;
    virtio_phys_addr_t used_addr;
    BOOL manual_recv;
} QueueState;





typedef struct {
    uint64_t addr;
    uint32_t len;
    uint16_t flags;
    uint16_t next;
} VIRTIODesc;



typedef int VIRTIODeviceRecvFunc(VIRTIODevice *s1, int queue_idx,
                                 int desc_idx, int read_size,
                                 int write_size);


typedef uint8_t *VIRTIOGetRAMPtrFunc(VIRTIODevice *s, virtio_phys_addr_t paddr, BOOL is_rw);

struct VIRTIODevice {
    PhysMemoryMap *mem_map;
    PhysMemoryRange *mem_range;

    PCIDevice *pci_dev;

    IRQSignal *irq;
    VIRTIOGetRAMPtrFunc *get_ram_ptr;
    int debug;

    uint32_t int_status;
    uint32_t status;
    uint32_t device_features_sel;
    uint32_t queue_sel;
    QueueState queue[8];


    uint32_t device_id;
    uint32_t vendor_id;
    uint32_t device_features;
    VIRTIODeviceRecvFunc *device_recv;
    void (*config_write)(VIRTIODevice *s);

    uint32_t config_space_size;
    uint8_t config_space[256];
};

static uint32_t virtio_mmio_read(void *opaque, uint32_t offset1, int size_log2);
static void virtio_mmio_write(void *opaque, uint32_t offset,
                              uint32_t val, int size_log2);
static uint32_t virtio_pci_read(void *opaque, uint32_t offset, int size_log2);
static void virtio_pci_write(void *opaque, uint32_t offset,
                             uint32_t val, int size_log2);

static void virtio_reset(VIRTIODevice *s)
{
    void virtio_reset__part3699(VIRTIODevice *s);

    int i;

    virtio_reset__part3699(s);

    void virtio_reset__part3700(QueueState *qs);
    virtio_reset__part3700(&s->queue[0]);

}

static uint8_t *virtio_pci_get_ram_ptr(VIRTIODevice *s, virtio_phys_addr_t paddr, BOOL is_rw)
{
    uint8_t *virtio_pci_get_ram_ptr__part3702(VIRTIODevice *s, virtio_phys_addr_t paddr, BOOL is_rw);
    return virtio_pci_get_ram_ptr__part3702(s, paddr, is_rw);
}

static uint8_t *virtio_mmio_get_ram_ptr(VIRTIODevice *s, virtio_phys_addr_t paddr, BOOL is_rw)
{
        uint8_t *virtio_mmio_get_ram_ptr__part3704(VIRTIODevice *s, virtio_phys_addr_t paddr, BOOL is_rw);
    return virtio_mmio_get_ram_ptr__part3704(s, paddr, is_rw);

}

static void virtio_add_pci_capability(VIRTIODevice *s, int cfg_type,
                                      int bar, uint32_t offset, uint32_t len,
                                      uint32_t mult)
{
    uint8_t cap[20];
    int cap_len;
        int virtio_add_pci_capability__part3706(int cfg_type);
    cap_len = virtio_add_pci_capability__part3706(cfg_type);

    void virtio_add_pci_capability__part3707(uint8_t *cap, int cap_len, int cfg_type, int bar, uint32_t offset, uint32_t len);
    virtio_add_pci_capability__part3707(cap, cap_len, cfg_type, bar, offset, len);

    void virtio_add_pci_capability__part3708(int cfg_type, uint8_t *cap, uint32_t mult);
    virtio_add_pci_capability__part3708(cfg_type, cap, mult);

        void virtio_add_pci_capability__part3709(VIRTIODevice *s, const uint8_t *cap, int cap_len);
    virtio_add_pci_capability__part3709(s, cap, cap_len);

}

static void virtio_pci_bar_set(void *opaque, int bar_num,
                               uint32_t addr, BOOL enabled)
{
    void virtio_pci_bar_set__part3711(void *opaque, int bar_num, uint32_t addr, int enabled);
    virtio_pci_bar_set__part3711(opaque, bar_num, addr, enabled);

}

static void virtio_init(VIRTIODevice *s, VIRTIOBusDef *bus,
                        uint32_t device_id, int config_space_size,
                        VIRTIODeviceRecvFunc *device_recv)
{
    void virtio_init__part3713(void *s, size_t size);
virtio_init__part3713(s, sizeof(*s));


    if (bus->pci_bus) {
        uint16_t pci_device_id, class_id;
        char name[32];
        int bar_num;

        switch(device_id) {
        case 1:
            pci_device_id = 0x1000;
            class_id = 0x0200;
            break;
        case 2:
            pci_device_id = 0x1001;
            class_id = 0x0100;
            break;
        case 3:
            pci_device_id = 0x1003;
            class_id = 0x0780;
            break;
        case 9:
            pci_device_id = 0x1040 + device_id;
            class_id = 0x2;
            break;
        case 18:
            pci_device_id = 0x1040 + device_id;
            class_id = 0x0980;
            break;
        default:
            abort();
        }
        snprintf(name, sizeof(name), "virtio_%04x", pci_device_id);
        s->pci_dev = pci_register_device(bus->pci_bus, name, -1,
                                         0x1af4, pci_device_id, 0x00,
                                         class_id);
        pci_device_set_config16(s->pci_dev, 0x2c, 0x1af4);
        pci_device_set_config16(s->pci_dev, 0x2e, device_id);
        pci_device_set_config8(s->pci_dev, 0x3d, 1);

        bar_num = 4;
        virtio_add_pci_capability(s, 1, bar_num,
                              0x0000, 0x1000, 0);
        virtio_add_pci_capability(s, 3, bar_num,
                              0x1000, 0x1000, 0);
        virtio_add_pci_capability(s, 4, bar_num,
                              0x2000, 0x1000, 0);
        virtio_add_pci_capability(s, 2, bar_num,
                              0x3000, 0x1000, 0);

        s->get_ram_ptr = virtio_pci_get_ram_ptr;
        s->irq = pci_device_get_irq(s->pci_dev, 0);
        s->mem_map = pci_device_get_mem_map(s->pci_dev);
        s->mem_range = cpu_register_device(s->mem_map, 0, 0x4000, s,
                                           virtio_pci_read, virtio_pci_write,
                                           (1 << 0) | (1 << 1) | (1 << 2) | (1 << 4));
        pci_register_bar(s->pci_dev, bar_num, 0x4000, 0x00,
                         s, virtio_pci_bar_set);
    } else {

        s->mem_map = bus->mem_map;
        s->irq = bus->irq;
        s->mem_range = cpu_register_device(s->mem_map, bus->addr, 4096,
                                           s, virtio_mmio_read, virtio_mmio_write,
                                           (1 << 0) | (1 << 1) | (1 << 2));
        s->get_ram_ptr = virtio_mmio_get_ram_ptr;
    }

        void virtio_init__part3715(VIRTIODevice *s, uint32_t device_id, int config_space_size, VIRTIODeviceRecvFunc *device_recv);
    virtio_init__part3715(s, device_id, config_space_size, device_recv);

}

static uint16_t virtio_read16(VIRTIODevice *s, virtio_phys_addr_t addr)
{
    uint8_t *ptr;
    extern int virtio_read16__part3717(uint64_t addr);
    if (virtio_read16__part3717(addr))
        return 0;

        uint8_t *virtio_read16__part3718(VIRTIODevice *s, virtio_phys_addr_t addr);
    ptr = virtio_read16__part3718(s, addr);
        int virtio_read16__part3719(const uint8_t *ptr);
    if (virtio_read16__part3719(ptr))
        return 0;

        {
        uint16_t virtio_read16__part3720(const uint8_t *ptr);
        return virtio_read16__part3720(ptr);
    }

}

static void virtio_write16(VIRTIODevice *s, virtio_phys_addr_t addr,
                           uint16_t val)
{
    uint8_t *ptr;
    extern int virtio_write16__part3722(uint64_t addr);
    if (virtio_write16__part3722(addr))
        return;
        uint8_t *virtio_write16__part3723(VIRTIODevice *s, virtio_phys_addr_t addr);
    ptr = virtio_write16__part3723(s, addr);

        int virtio_write16__part3724(uint8_t *ptr);
    if (virtio_write16__part3724(ptr))
        return;
    void virtio_write16__part3725(uint8_t *ptr, uint16_t val);
virtio_write16__part3725(ptr, val);

}

static void virtio_write32(VIRTIODevice *s, virtio_phys_addr_t addr,
                           uint32_t val)
{
    uint8_t *ptr;
        {
        int virtio_write32__part3727(uint64_t addr);
        if (virtio_write32__part3727(addr))
            return;
    }

    uint8_t *virtio_write32__part3728(VIRTIODevice *s, virtio_phys_addr_t addr);
    ptr = virtio_write32__part3728(s, addr);
    {
    int virtio_write32__part3729(uint8_t *ptr);
    if (virtio_write32__part3729(ptr))
        return;
}

    void virtio_write32__part3730(uint8_t *ptr, uint32_t val);
    virtio_write32__part3730(ptr, val);

}

static int virtio_memcpy_from_ram(VIRTIODevice *s, uint8_t *buf,
                                  virtio_phys_addr_t addr, int count)
{
    uint8_t *ptr;
    int l;

        {
        extern int virtio_memcpy_from_ram__part3732(VIRTIODevice *s, uint8_t *buf,
                                                   virtio_phys_addr_t addr, int count);
        int part_ret;
        (void)l;
        (void)ptr;
        part_ret = virtio_memcpy_from_ram__part3732(s, buf, addr, count);
        if (part_ret < 0)
            return -1;
    }

    int virtio_memcpy_from_ram__part3733(void);
return virtio_memcpy_from_ram__part3733();

}

static int virtio_memcpy_to_ram(VIRTIODevice *s, virtio_phys_addr_t addr,
                                const uint8_t *buf, int count)
{
    uint8_t *ptr;
    int l;

        {
        int virtio_memcpy_to_ram__part3735(VIRTIODevice *s, virtio_phys_addr_t addr,
                                           const uint8_t *buf, int count);
        if (virtio_memcpy_to_ram__part3735(s, addr, buf, count) < 0)
            return -1;
    }
    int virtio_memcpy_to_ram__part3736(void);
    return virtio_memcpy_to_ram__part3736();

}

static int get_desc(VIRTIODevice *s, VIRTIODesc *desc,
                    int queue_idx, int desc_idx)
{
    int get_desc__part3738(VIRTIODevice *s, VIRTIODesc *desc, int queue_idx, int desc_idx);
    return get_desc__part3738(s, desc, queue_idx, desc_idx);
}

static int memcpy_to_from_queue(VIRTIODevice *s, uint8_t *buf,
                                int queue_idx, int desc_idx,
                                int offset, int count, BOOL to_queue)
{
    VIRTIODesc desc;
    int l, f_write_flag;

    int memcpy_to_from_queue__part3740(int count);

    if (memcpy_to_from_queue__part3740(count))
        return 0;


        {
        int memcpy_to_from_queue__part3741(VIRTIODevice *s, VIRTIODesc *desc, int queue_idx, int desc_idx);
        (void)memcpy_to_from_queue__part3741(s, &desc, queue_idx, desc_idx);
    }


        {
        int memcpy_to_from_queue__part3742(VIRTIODevice *s, int queue_idx,
                                           int *desc_idx, VIRTIODesc *desc,
                                           BOOL to_queue, int *f_write_flag);
        int part_ret = memcpy_to_from_queue__part3742(s, queue_idx, &desc_idx,
                                                      &desc, to_queue,
                                                      &f_write_flag);
        if (part_ret != 0)
            return part_ret;
    }


        /* part3743: descriptor advance loop ported to Rust */
    {
        extern int memcpy_to_from_queue__part3743(VIRTIODevice *, VIRTIODesc *, int *, int *, int, int);
        int part3743_rc = memcpy_to_from_queue__part3743(s, &desc, &offset, &desc_idx, queue_idx, f_write_flag);
        if (part3743_rc != 0)
            return -1;
    }


    for(;;) {
        l = min_int(count, desc.len - offset);
        if (to_queue)
            virtio_memcpy_to_ram(s, desc.addr + offset, buf, l);
        else
            virtio_memcpy_from_ram(s, buf, desc.addr + offset, l);
        count -= l;
        if (count == 0)
            break;
        offset += l;
        buf += l;
        if (offset == desc.len) {
            if (!(desc.flags & 1))
                return -1;
            desc_idx = desc.next;
            get_desc(s, &desc, queue_idx, desc_idx);
            if ((desc.flags & 2) != f_write_flag)
                return -1;
            offset = 0;
        }
    }
        {
        int memcpy_to_from_queue__part3745(void);
        return memcpy_to_from_queue__part3745();
    }
}

static int memcpy_from_queue(VIRTIODevice *s, void *buf,
                             int queue_idx, int desc_idx,
                             int offset, int count)
{
        {
        int memcpy_from_queue__part3747(VIRTIODevice *s, void *buf,
                                        int queue_idx, int desc_idx,
                                        int offset, int count);
        return memcpy_from_queue__part3747(s, buf, queue_idx, desc_idx, offset,
                                           count);
    }

}

static int memcpy_to_queue(VIRTIODevice *s,
                           int queue_idx, int desc_idx,
                           int offset, const void *buf, int count)
{
        int memcpy_to_queue__part3749(VIRTIODevice *s, int queue_idx, int desc_idx, int offset, const void *buf, int count);
    return memcpy_to_queue__part3749(s, queue_idx, desc_idx, offset, buf, count);

}


static void virtio_consume_desc(VIRTIODevice *s,
                                int queue_idx, int desc_idx, int desc_len)
{
        void virtio_consume_desc__part3751(VIRTIODevice *s, int queue_idx, int desc_idx, int desc_len);
    virtio_consume_desc__part3751(s, queue_idx, desc_idx, desc_len);

}

static int get_desc_rw_size(VIRTIODevice *s,
                             int *pread_size, int *pwrite_size,
                             int queue_idx, int desc_idx)
{
    void get_desc_rw_size__part3753(VIRTIODevice *s, int queue_idx, int desc_idx,
                                VIRTIODesc *pdesc, int *pread_size, int *pwrite_size);
    VIRTIODesc desc;
    int read_size, write_size;

    get_desc_rw_size__part3753(s, queue_idx, desc_idx, &desc, &read_size, &write_size);


    int get_desc_rw_size__part3754(VIRTIODevice *s, VIRTIODesc *desc,
                               int *pread_size, int queue_idx, int *pdesc_idx);
    if (get_desc_rw_size__part3754(s, &desc, &read_size, queue_idx, &desc_idx))
        goto done;

        {
        extern int get_desc_rw_size__part3755(VIRTIODevice *s, VIRTIODesc *desc,
                                              int *write_size, int *desc_idx,
                                              int queue_idx);
        int part3755_ret;

        part3755_ret = get_desc_rw_size__part3755(s, &desc, &write_size,
                                                 &desc_idx, queue_idx);
        if (part3755_ret)
            return part3755_ret;
    }


  done:
    void get_desc_rw_size__part3756(int *pread_size, int read_size);
    get_desc_rw_size__part3756(pread_size, read_size);

        {
        int get_desc_rw_size__part3757(int *pwrite_size, int write_size);
        return get_desc_rw_size__part3757(pwrite_size, write_size);
    }

}


static void queue_notify(VIRTIODevice *s, int queue_idx)
{
    QueueState *qs = &s->queue[queue_idx];
    uint16_t avail_idx;
    int desc_idx, read_size, write_size;

    int queue_notify__part3759(QueueState *qs);

    if (queue_notify__part3759(qs))
        return;

        uint16_t queue_notify__part3760(VIRTIODevice *s, virtio_phys_addr_t addr);
    avail_idx = queue_notify__part3760(s, qs->avail_addr + 2);
    void queue_notify__part3761(VIRTIODevice *s, int queue_idx, uint16_t avail_idx);
    queue_notify__part3761(s, queue_idx, avail_idx);
}

static uint32_t virtio_config_read(VIRTIODevice *s, uint32_t offset,
                                   int size_log2)
{
    uint32_t val;
        uint32_t virtio_config_read__part3763(const uint8_t *config_space,
                                          uint32_t config_space_size,
                                          uint32_t offset, int size_log2);
    val = virtio_config_read__part3763(s->config_space, s->config_space_size,
                                       offset, size_log2);

        {
        uint32_t virtio_config_read__part3764(uint32_t val);
        return virtio_config_read__part3764(val);
    }

}

static void virtio_config_write(VIRTIODevice *s, uint32_t offset,
                                uint32_t val, int size_log2)
{
        void virtio_config_write__part3766(uint8_t *config_space,
                                       uint32_t config_space_size,
                                       void (*config_write)(VIRTIODevice *s),
                                       VIRTIODevice *s,
                                       uint32_t offset, uint32_t val,
                                       int size_log2);
    virtio_config_write__part3766(s->config_space, s->config_space_size,
                                  s->config_write, s, offset, val, size_log2);

}

static uint32_t virtio_mmio_read(void *opaque, uint32_t offset, int size_log2)
{
    VIRTIODevice *s = opaque;
    uint32_t val;

    uint32_t virtio_mmio_read__part3768(void *opaque, uint32_t offset, int size_log2);
    if (offset >= 0x100) {
        return virtio_mmio_read__part3768(opaque, offset, size_log2);
    }


        uint32_t virtio_mmio_read__part3769(VIRTIODevice *s, uint32_t offset, int size_log2);
    val = virtio_mmio_read__part3769(s, offset, size_log2);







        uint32_t virtio_mmio_read__part3770(uint32_t val);
    return virtio_mmio_read__part3770(val);

}


static void set_low32(virtio_phys_addr_t *paddr, uint32_t val)
{
        extern void set_low32__part3772(virtio_phys_addr_t *paddr, uint32_t val);
    set_low32__part3772(paddr, val);
}

static void set_high32(virtio_phys_addr_t *paddr, uint32_t val)
{
        void set_high32__part3774(uint64_t *paddr, uint32_t val);
    set_high32__part3774(paddr, val);
}







static void virtio_mmio_write(void *opaque, uint32_t offset,
                              uint32_t val, int size_log2)
{
    VIRTIODevice *s = opaque;
    int virtio_mmio_write__part3776(VIRTIODevice *s, uint32_t offset,
                               uint32_t val, int size_log2);
    if (virtio_mmio_write__part3776(s, offset, val, size_log2))
        return;


        void virtio_mmio_write__part3777(void *opaque, uint32_t offset, uint32_t val, int size_log2);
    virtio_mmio_write__part3777(opaque, offset, val, size_log2);
}

static uint32_t virtio_pci_read(void *opaque, uint32_t offset1, int size_log2)
{
    uint32_t virtio_pci_read__part3779(void *opaque, uint32_t offset1, uint32_t *val_out);
    VIRTIODevice *s = opaque;
    uint32_t offset;
    uint32_t val;
    offset = virtio_pci_read__part3779(opaque, offset1, &val);

    {
    uint32_t virtio_pci_read__part3780(void *opaque, uint32_t offset1, int size_log2);
    val = virtio_pci_read__part3780(opaque, offset1, size_log2);
}







    uint32_t virtio_pci_read__part3781(uint32_t val);
    return virtio_pci_read__part3781(val);
}

static void virtio_pci_write(void *opaque, uint32_t offset1,
                             uint32_t val, int size_log2)
{
        VIRTIODevice *s = opaque;
    uint32_t offset;
    uint32_t virtio_pci_write__part3783(uint32_t offset1);

    offset = virtio_pci_write__part3783(offset1);

    switch(offset1 >> 12) {
    case 0x0000 >> 12:
        if (size_log2 == 2) {
            switch(offset) {
            case 0x000:
                s->device_features_sel = val;
                break;
            case 0x020:
                set_low32(&s->queue[s->queue_sel].desc_addr, val);
                break;
            case 0x028:
                set_low32(&s->queue[s->queue_sel].avail_addr, val);
                break;
            case 0x030:
                set_low32(&s->queue[s->queue_sel].used_addr, val);
                break;

            case 0x024:
                set_high32(&s->queue[s->queue_sel].desc_addr, val);
                break;
            case 0x02c:
                set_high32(&s->queue[s->queue_sel].avail_addr, val);
                break;
            case 0x034:
                set_high32(&s->queue[s->queue_sel].used_addr, val);
                break;

            }
        } else if (size_log2 == 1) {
            switch(offset) {
            case 0x016:
                if (val < 8)
                    s->queue_sel = val;
                break;
            case 0x018:
                if ((val & (val - 1)) == 0 && val > 0) {
                    s->queue[s->queue_sel].num = val;
                }
                break;
            case 0x01c:
                s->queue[s->queue_sel].ready = val & 1;
                break;
            }
        } else if (size_log2 == 0) {
            switch(offset) {
            case 0x014:
                s->status = val;
                if (val == 0) {

                    set_irq(s->irq, 0);
                    virtio_reset(s);
                }
                break;
            }
        }
        break;
    case 0x2000 >> 12:
        virtio_config_write(s, offset, val, size_log2);
        break;
    case 0x3000 >> 12:
        if (val < 8)
            queue_notify(s, val);
        break;
    }
}

void virtio_set_debug(VIRTIODevice *s, int debug)
{
    void virtio_set_debug__part3786(VIRTIODevice *s, int debug);
virtio_set_debug__part3786(s, debug);

}

static void virtio_config_change_notify(VIRTIODevice *s)
{

    void virtio_config_change_notify__part3788(VIRTIODevice *s);
    virtio_config_change_notify__part3788(s);
}




typedef struct {
    uint32_t type;
    uint8_t *buf;
    int write_size;
    int queue_idx;
    int desc_idx;
} BlockRequest;

typedef struct VIRTIOBlockDevice {
    VIRTIODevice common;
    BlockDevice *bs;

    BOOL req_in_progress;
    BlockRequest req;
} VIRTIOBlockDevice;

typedef struct {
    uint32_t type;
    uint32_t ioprio;
    uint64_t sector_num;
} BlockRequestHeader;
static void virtio_block_req_end(VIRTIODevice *s, int ret)
{
    VIRTIOBlockDevice *s1 = (VIRTIOBlockDevice *)s;
    int write_size;
    int queue_idx = s1->req.queue_idx;
    int desc_idx = s1->req.desc_idx;
    uint8_t *buf, buf1[1];

    switch(s1->req.type) {
    case 0:
        write_size = s1->req.write_size;
        buf = s1->req.buf;
        if (ret < 0) {
            buf[write_size - 1] = 1;
        } else {
            buf[write_size - 1] = 0;
        }
        memcpy_to_queue(s, queue_idx, desc_idx, 0, buf, write_size);
        free(buf);
        virtio_consume_desc(s, queue_idx, desc_idx, write_size);
        break;
    case 1:
        if (ret < 0)
            buf1[0] = 1;
        else
            buf1[0] = 0;
        memcpy_to_queue(s, queue_idx, desc_idx, 0, buf1, sizeof(buf1));
        virtio_consume_desc(s, queue_idx, desc_idx, 1);
        break;
    default:
        abort();
    }
}

static void virtio_block_req_cb(void *opaque, int ret)
{
        void virtio_block_req_cb__part3798(void *opaque, int ret);

    virtio_block_req_cb__part3798(opaque, ret);

}


static int virtio_block_recv_request(VIRTIODevice *s, int queue_idx,
                                     int desc_idx, int read_size,
                                     int write_size)
{
    VIRTIOBlockDevice *s1 = (VIRTIOBlockDevice *)s;
    BlockDevice *bs = s1->bs;
    BlockRequestHeader h;
    uint8_t *buf;
    int len, ret;

    extern int virtio_block_recv_request__part3800(VIRTIOBlockDevice *s1);
    if (virtio_block_recv_request__part3800(s1))
        return -1;


        {
        int virtio_block_recv_request__part3801(void *s, void *h, int queue_idx, int desc_idx);
        if (virtio_block_recv_request__part3801(s, &h, queue_idx, desc_idx))
            return 0;
    }

    {
        void virtio_block_recv_request__part3802(BlockRequest *req,
                                                 const BlockRequestHeader *h,
                                                 int queue_idx, int desc_idx);
        virtio_block_recv_request__part3802(&s1->req, &h, queue_idx, desc_idx);
    }
    switch(h.type) {
    case 0:
        s1->req.buf = malloc(write_size);
        s1->req.write_size = write_size;
        ret = bs->read_async(bs, h.sector_num, s1->req.buf,
                             (write_size - 1) / 512,
                             virtio_block_req_cb, s);
        if (ret > 0) {

            s1->req_in_progress = TRUE;
        } else {
            virtio_block_req_end(s, ret);
        }
        break;
    case 1:
        
       ((void) sizeof (__assert_single_arg (
       write_size >= 1
       )), __extension__ ({ if (
       write_size >= 1
       ) ; else __assert_fail (
       "write_size >= 1"
       , "virtio.c", 1097, __extension__ __PRETTY_FUNCTION__); }))
                              ;
        len = read_size - sizeof(h);
        buf = malloc(len);
        memcpy_from_queue(s, buf, queue_idx, desc_idx, sizeof(h), len);
        ret = bs->write_async(bs, h.sector_num, buf, len / 512,
                              virtio_block_req_cb, s);
        free(buf);
        if (ret > 0) {

            s1->req_in_progress = TRUE;
        } else {
            virtio_block_req_end(s, ret);
        }
        break;
    default:
        break;
    }
    int virtio_block_recv_request__part3804(void);
    return virtio_block_recv_request__part3804();
}

VIRTIODevice *virtio_block_init(VIRTIOBusDef *bus, BlockDevice *bs)
{
    VIRTIOBlockDevice *s;
    uint64_t nb_sectors;

    s = mallocz(sizeof(*s));
    virtio_init(&s->common, bus,
                2, 8, virtio_block_recv_request);
    s->bs = bs;

    nb_sectors = bs->get_sector_count(bs);
    put_le32(s->common.config_space, nb_sectors);
    put_le32(s->common.config_space + 4, nb_sectors >> 32);

    return (VIRTIODevice *)s;
}




typedef struct VIRTIONetDevice {
    VIRTIODevice common;
    EthernetDevice *es;
    int header_size;
} VIRTIONetDevice;

typedef struct {
    uint8_t flags;
    uint8_t gso_type;
    uint16_t hdr_len;
    uint16_t gso_size;
    uint16_t csum_start;
    uint16_t csum_offset;
    uint16_t num_buffers;
} VIRTIONetHeader;

static int virtio_net_recv_request(VIRTIODevice *s, int queue_idx,
                                   int desc_idx, int read_size,
                                   int write_size)
{
    VIRTIONetDevice *s1 = (VIRTIONetDevice *)s;
    EthernetDevice *es = s1->es;
    VIRTIONetHeader h;
    uint8_t *buf;
    int len;

        if (queue_idx == 1) {
        extern int virtio_net_recv_request__part3812(VIRTIODevice *s, int queue_idx,
                                                     int desc_idx, int read_size,
                                                     int write_size);
        if (virtio_net_recv_request__part3812(s, queue_idx, desc_idx,
                                              read_size, write_size))
            return 0;
    }

        int virtio_net_recv_request__part3813(void);
    return virtio_net_recv_request__part3813();

}

static BOOL virtio_net_can_write_packet(EthernetDevice *es)
{
    VIRTIODevice *s = es->device_opaque;
    QueueState *qs = &s->queue[0];
    uint16_t avail_idx;

        int virtio_net_can_write_packet__part3815(const QueueState *qs);
    if (!virtio_net_can_write_packet__part3815(qs))
        return FALSE;

    BOOL virtio_net_can_write_packet__part3816(VIRTIODevice *s, QueueState *qs);
    return virtio_net_can_write_packet__part3816(s, qs);

}

static void virtio_net_write_packet(EthernetDevice *es, const uint8_t *buf, int buf_len)
{
    VIRTIODevice *s = es->device_opaque;
    VIRTIONetDevice *s1 = (VIRTIONetDevice *)s;
    int queue_idx = 0;
    QueueState *qs = &s->queue[queue_idx];
    int desc_idx;
    VIRTIONetHeader h;
    int len, read_size, write_size;
    uint16_t avail_idx;

    int virtio_net_write_packet__part3818(const QueueState *qs);
    if (virtio_net_write_packet__part3818(qs))
        return;
    uint16_t virtio_net_write_packet__part3819(VIRTIODevice *s, virtio_phys_addr_t addr);
avail_idx = virtio_net_write_packet__part3819(s, qs->avail_addr + 2);
    int virtio_net_write_packet__part3820(const QueueState *qs, uint16_t avail_idx);
    if (virtio_net_write_packet__part3820(qs, avail_idx))
        return;

    uint16_t virtio_net_write_packet__part3821(VIRTIODevice *s, QueueState *qs);
desc_idx = virtio_net_write_packet__part3821(s, qs);
        int virtio_net_write_packet__part3822(VIRTIODevice *s, int *pread_size, int *pwrite_size, int queue_idx, int desc_idx);
    if (virtio_net_write_packet__part3822(s, &read_size, &write_size, queue_idx, desc_idx))
        return;
    int virtio_net_write_packet__part3823(int header_size, int buf_len);
    len = virtio_net_write_packet__part3823(s1->header_size, buf_len);

    int virtio_net_write_packet__part3824(int len, int write_size);
    if (virtio_net_write_packet__part3824(len, write_size))
        return;

    memset(&h, 0, s1->header_size);
    memcpy_to_queue(s, queue_idx, desc_idx, 0, &h, s1->header_size);
    memcpy_to_queue(s, queue_idx, desc_idx, s1->header_size, buf, buf_len);
    virtio_consume_desc(s, queue_idx, desc_idx, len);
    qs->last_avail_idx++;
}

static void virtio_net_set_carrier(EthernetDevice *es, BOOL carrier_state)
{
}

VIRTIODevice *virtio_net_init(VIRTIOBusDef *bus, EthernetDevice *es)
{
    VIRTIONetDevice *s;

    s = mallocz(sizeof(*s));
    virtio_init(&s->common, bus,
                1, 6 + 2, virtio_net_recv_request);

    s->common.device_features = (1 << 5) ;
    s->common.queue[0].manual_recv = TRUE;
    s->es = es;
    memcpy(s->common.config_space, es->mac_addr, 6);

    s->common.config_space[6] = 0;
    s->common.config_space[7] = 0;

    s->header_size = sizeof(VIRTIONetHeader);

    es->device_opaque = s;
    es->device_can_write_packet = virtio_net_can_write_packet;
    es->device_write_packet = virtio_net_write_packet;
    es->device_set_carrier = virtio_net_set_carrier;
    return (VIRTIODevice *)s;
}




typedef struct VIRTIOConsoleDevice {
    VIRTIODevice common;
    CharacterDevice *cs;
} VIRTIOConsoleDevice;

static int virtio_console_recv_request(VIRTIODevice *s, int queue_idx,
                                       int desc_idx, int read_size,
                                       int write_size)
{
    VIRTIOConsoleDevice *s1 = (VIRTIOConsoleDevice *)s;
    CharacterDevice *cs = s1->cs;
    uint8_t *buf;

    void virtio_console_recv_request__part3832(VIRTIODevice *s, CharacterDevice *cs,
                                           int queue_idx, int desc_idx,
                                           int read_size);
    virtio_console_recv_request__part3832(s, cs, queue_idx, desc_idx, read_size);

    int virtio_console_recv_request__part3833(void);
    return virtio_console_recv_request__part3833();
}

BOOL virtio_console_can_write_data(VIRTIODevice *s)
{
    QueueState *qs = &s->queue[0];
    uint16_t avail_idx;

    {
    int virtio_console_can_write_data__part3835(const QueueState *qs);
    if (virtio_console_can_write_data__part3835(qs))
        return FALSE;
}
    BOOL virtio_console_can_write_data__part3836(VIRTIODevice *s, QueueState *qs);
    return virtio_console_can_write_data__part3836(s, qs);

}

int virtio_console_get_write_len(VIRTIODevice *s)
{
    int queue_idx = 0;
    QueueState *qs = &s->queue[queue_idx];
    int desc_idx;
    int read_size, write_size;
    uint16_t avail_idx;

    extern int virtio_console_get_write_len__part3838(uint32_t ready);
    if (virtio_console_get_write_len__part3838(qs->ready))
        return 0;

        uint16_t virtio_console_get_write_len__part3839(VIRTIODevice *s, QueueState *qs);
    avail_idx = virtio_console_get_write_len__part3839(s, qs);
    int virtio_console_get_write_len__part3840(const QueueState *qs, uint16_t avail_idx);
    if (virtio_console_get_write_len__part3840(qs, avail_idx))
        return 0;

    uint16_t virtio_console_get_write_len__part3841(VIRTIODevice *s, QueueState *qs);
    desc_idx = virtio_console_get_write_len__part3841(s, qs);

        int virtio_console_get_write_len__part3842(VIRTIODevice *s, int *pread_size, int *pwrite_size, int queue_idx, int desc_idx);
    if (virtio_console_get_write_len__part3842(s, &read_size, &write_size, queue_idx, desc_idx))
        return 0;
        int virtio_console_get_write_len__part3843(int write_size);
    return virtio_console_get_write_len__part3843(write_size);

}

int virtio_console_write_data(VIRTIODevice *s, const uint8_t *buf, int buf_len)
{
    int queue_idx = 0;
    QueueState *qs = &s->queue[queue_idx];
    int desc_idx;
    uint16_t avail_idx;

    int virtio_console_write_data__part3845(const QueueState *qs);
    if (virtio_console_write_data__part3845(qs))
        return 0;

    uint16_t virtio_console_write_data__part3846(VIRTIODevice *s, QueueState *qs);
    avail_idx = virtio_console_write_data__part3846(s, qs);
    int virtio_console_write_data__part3847(uint16_t last_avail_idx, uint16_t avail_idx);
    if (virtio_console_write_data__part3847(qs->last_avail_idx, avail_idx))
        return 0;

    desc_idx = virtio_read16(s, qs->avail_addr + 4 +
                             (qs->last_avail_idx & (qs->num - 1)) * 2);
    memcpy_to_queue(s, queue_idx, desc_idx, 0, buf, buf_len);
    virtio_consume_desc(s, queue_idx, desc_idx, buf_len);
    qs->last_avail_idx++;
    return buf_len;
}


void virtio_console_resize_event(VIRTIODevice *s, int width, int height)
{

    void virtio_console_resize_event__part3850(VIRTIODevice *s, int width, int height);
    virtio_console_resize_event__part3850(s, width, height);

}

VIRTIODevice *virtio_console_init(VIRTIOBusDef *bus, CharacterDevice *cs)
{
    VIRTIOConsoleDevice *s;

    s = mallocz(sizeof(*s));
    virtio_init(&s->common, bus,
                3, 4, virtio_console_recv_request);
    s->common.device_features = (1 << 0);
    s->common.queue[0].manual_recv = TRUE;

    s->cs = cs;
    return (VIRTIODevice *)s;
}




enum {
    VIRTIO_INPUT_CFG_UNSET = 0x00,
    VIRTIO_INPUT_CFG_ID_NAME = 0x01,
    VIRTIO_INPUT_CFG_ID_SERIAL = 0x02,
    VIRTIO_INPUT_CFG_ID_DEVIDS = 0x03,
    VIRTIO_INPUT_CFG_PROP_BITS = 0x10,
    VIRTIO_INPUT_CFG_EV_BITS = 0x11,
    VIRTIO_INPUT_CFG_ABS_INFO = 0x12,
};
typedef struct VIRTIOInputDevice {
    VIRTIODevice common;
    VirtioInputTypeEnum type;
    uint32_t buttons_state;
} VIRTIOInputDevice;

static const uint16_t buttons_list[] = {
    0x110, 0x111, 0x112
};

static int virtio_input_recv_request(VIRTIODevice *s, int queue_idx,
                                      int desc_idx, int read_size,
                                      int write_size)
{
    void virtio_input_recv_request__part3858(VIRTIODevice *s, int queue_idx, int desc_idx);
    virtio_input_recv_request__part3858(s, queue_idx, desc_idx);

        int virtio_input_recv_request__part3859(void);
    return virtio_input_recv_request__part3859();

}


static int virtio_input_queue_event(VIRTIODevice *s,
                                    uint16_t type, uint16_t code,
                                    uint32_t value)
{
    int queue_idx = 0;
    QueueState *qs = &s->queue[queue_idx];
    int desc_idx, buf_len;
    uint16_t avail_idx;
    uint8_t buf[8];

    extern int virtio_input_queue_event__part3861(const uint32_t *ready);
    if (virtio_input_queue_event__part3861(&qs->ready))
        return -1;


        void virtio_input_queue_event__part3862(VIRTIODevice *s, QueueState *qs,
                                            uint16_t type, uint16_t code,
                                            uint32_t value, uint8_t *buf,
                                            int *buf_len, uint16_t *avail_idx);
    virtio_input_queue_event__part3862(s, qs, type, code, value, buf,
                                       &buf_len, &avail_idx);

    int virtio_input_queue_event__part3863(uint16_t last_avail_idx, uint16_t avail_idx);
    if (virtio_input_queue_event__part3863(qs->last_avail_idx, avail_idx))
        return -1;

        int virtio_input_queue_event__part3864(VIRTIODevice *s, int queue_idx,
                                           const uint8_t *buf, int buf_len);
    return virtio_input_queue_event__part3864(s, queue_idx, buf, buf_len);

}

int virtio_input_send_key_event(VIRTIODevice *s, BOOL is_down,
                                uint16_t key_code)
{
    VIRTIOInputDevice *s1 = (VIRTIOInputDevice *)s;
    int ret;

    int virtio_input_send_key_event__part3866(int type);
    if (virtio_input_send_key_event__part3866(s1->type))
        return -1;

    ret = virtio_input_queue_event(s, 0x01, key_code, is_down);
    int virtio_input_send_key_event__part3868(int ret);
    if (virtio_input_send_key_event__part3868(ret))
        return ret;

    int virtio_input_send_key_event__part3869(VIRTIODevice *s);
    return virtio_input_send_key_event__part3869(s);
}


int virtio_input_send_mouse_event(VIRTIODevice *s, int dx, int dy, int dz,
                                  unsigned int buttons)
{
    VIRTIOInputDevice *s1 = (VIRTIOInputDevice *)s;
    int ret, i, b, last_b;

    {
    int virtio_input_send_mouse_event__part3871(int input_type);
    if (virtio_input_send_mouse_event__part3871((int)s1->type) != 0)
        return -1;
}

    {
        int virtio_input_send_mouse_event__part3872(VIRTIODevice *s, int dx, int dy);
        ret = virtio_input_send_mouse_event__part3872(s, dx, dy);
        if (ret != 0)
            return ret;
    }

    {
    int virtio_input_send_mouse_event__part3873(VIRTIODevice *s, int dz);
    ret = virtio_input_send_mouse_event__part3873(s, dz);
    if (ret != 0)
        return ret;
}


    if (buttons != s1->buttons_state) {
        for(i = 0; i < (sizeof(buttons_list) / sizeof(buttons_list[0])); i++) {
            b = (buttons >> i) & 1;
            last_b = (s1->buttons_state >> i) & 1;
            if (b != last_b) {
                ret = virtio_input_queue_event(s, 0x01,
                                               buttons_list[i], b);
                if (ret != 0)
                    return ret;
            }
        }
        s1->buttons_state = buttons;
    }

    return virtio_input_queue_event(s, 0x00, 0, 0);
}

static void set_bit(uint8_t *tab, int k)
{
    void set_bit__part3877(uint8_t *tab, int k);
    set_bit__part3877(tab, k);

}

static void virtio_input_config_write(VIRTIODevice *s)
{
    VIRTIOInputDevice *s1 = (VIRTIOInputDevice *)s;
    uint8_t *config = s->config_space;
    int i;


        void virtio_input_config_write__part3879(uint8_t *config, int s1_type);
    virtio_input_config_write__part3879(config, (int)s1->type);

}

VIRTIODevice *virtio_input_init(VIRTIOBusDef *bus, VirtioInputTypeEnum type)
{
    VIRTIOInputDevice *s;

    s = mallocz(sizeof(*s));
    virtio_init(&s->common, bus,
                18, 256, virtio_input_recv_request);
    s->common.queue[0].manual_recv = TRUE;
    s->common.device_features = 0;
    s->common.config_write = virtio_input_config_write;
    s->type = type;
    return (VIRTIODevice *)s;
}




typedef struct {
    struct list_head link;
    uint32_t fid;
    FSFile *fd;
} FIDDesc;

typedef struct VIRTIO9PDevice {
    VIRTIODevice common;
    FSDevice *fs;
    int msize;
    struct list_head fid_list;
    BOOL req_in_progress;
} VIRTIO9PDevice;

static FIDDesc *fid_find1(VIRTIO9PDevice *s, uint32_t fid)
{
    struct list_head *el;
    FIDDesc *f;

        FIDDesc *fid_find1__part3887(struct list_head *head, struct list_head **el_out, uint32_t fid);
    f = fid_find1__part3887(&s->fid_list, &el, fid);
    if (f)
        return f;

        {
        FIDDesc *(*fid_find1__part3888)(void);
        return fid_find1__part3888();
    }
}

static FSFile *fid_find(VIRTIO9PDevice *s, uint32_t fid)
{
    FIDDesc *f;
    FIDDesc *fid_find__part3890(VIRTIO9PDevice *s, uint32_t fid);

    f = fid_find__part3890(s, fid);
    int fid_find__part3891(const void *f);
    if (fid_find__part3891(f))
        return ((void *)0);

    void *fid_find__part3892(FIDDesc *f);
return fid_find__part3892(f);

}

static void fid_delete(VIRTIO9PDevice *s, uint32_t fid)
{
    FIDDesc *f;

    FIDDesc *fid_delete__part3894(VIRTIO9PDevice *s, uint32_t fid);
    f = fid_delete__part3894(s, fid);
    void fid_delete__part3895(VIRTIO9PDevice *s, FIDDesc *f);
    fid_delete__part3895(s, f);
}

static void fid_set(VIRTIO9PDevice *s, uint32_t fid, FSFile *fd)
{
    FIDDesc *fid_set__part3897(VIRTIO9PDevice *s, uint32_t fid);

    FIDDesc *f;

    f = fid_set__part3897(s, fid);

    FIDDesc *fid_set__part3898(FSDevice *fs, struct list_head *fid_list,
                           uint32_t fid, FSFile *fd, FIDDesc *f);

    f = fid_set__part3898(s->fs, &s->fid_list, fid, fd, f);

}
static int marshall(VIRTIO9PDevice *s,
                    uint8_t *buf1, int max_len, const char *fmt, ...)
{
    va_list ap;
    int c;
    uint32_t val;
    uint64_t val64;
    uint8_t *buf, *buf_end;





    
   __builtin_c23_va_start(
   ap, fmt
   )
                    ;
    buf = buf1;
    buf_end = buf1 + max_len;
    for(;;) {
        c = *fmt++;
        if (c == '\0')
            break;
        switch(c) {
        case 'b':
            
           ((void) sizeof (__assert_single_arg (
           buf + 1 <= buf_end
           )), __extension__ ({ if (
           buf + 1 <= buf_end
           ) ; else __assert_fail (
           "buf + 1 <= buf_end"
           , "virtio.c", 1783, __extension__ __PRETTY_FUNCTION__); }))
                                     ;
            val = 
                 __builtin_va_arg(
                 ap
                 ,
                 int
                 )
                                ;




            buf[0] = val;
            buf += 1;
            break;
        case 'h':
            
           ((void) sizeof (__assert_single_arg (
           buf + 2 <= buf_end
           )), __extension__ ({ if (
           buf + 2 <= buf_end
           ) ; else __assert_fail (
           "buf + 2 <= buf_end"
           , "virtio.c", 1793, __extension__ __PRETTY_FUNCTION__); }))
                                     ;
            val = 
                 __builtin_va_arg(
                 ap
                 ,
                 int
                 )
                                ;




            put_le16(buf, val);
            buf += 2;
            break;
        case 'w':
            
           ((void) sizeof (__assert_single_arg (
           buf + 4 <= buf_end
           )), __extension__ ({ if (
           buf + 4 <= buf_end
           ) ; else __assert_fail (
           "buf + 4 <= buf_end"
           , "virtio.c", 1803, __extension__ __PRETTY_FUNCTION__); }))
                                     ;
            val = 
                 __builtin_va_arg(
                 ap
                 ,
                 int
                 )
                                ;




            put_le32(buf, val);
            buf += 4;
            break;
        case 'd':
            
           ((void) sizeof (__assert_single_arg (
           buf + 8 <= buf_end
           )), __extension__ ({ if (
           buf + 8 <= buf_end
           ) ; else __assert_fail (
           "buf + 8 <= buf_end"
           , "virtio.c", 1813, __extension__ __PRETTY_FUNCTION__); }))
                                     ;
            val64 = 
                   __builtin_va_arg(
                   ap
                   ,
                   uint64_t
                   )
                                       ;




            put_le64(buf, val64);
            buf += 8;
            break;
        case 's':
            {
                char *str;
                int len;
                str = 
                     __builtin_va_arg(
                     ap
                     ,
                     char *
                     )
                                       ;




                len = strlen(str);
                
               ((void) sizeof (__assert_single_arg (
               len <= 65535
               )), __extension__ ({ if (
               len <= 65535
               ) ; else __assert_fail (
               "len <= 65535"
               , "virtio.c", 1832, __extension__ __PRETTY_FUNCTION__); }))
                                   ;
                
               ((void) sizeof (__assert_single_arg (
               buf + 2 + len <= buf_end
               )), __extension__ ({ if (
               buf + 2 + len <= buf_end
               ) ; else __assert_fail (
               "buf + 2 + len <= buf_end"
               , "virtio.c", 1833, __extension__ __PRETTY_FUNCTION__); }))
                                               ;
                put_le16(buf, len);
                buf += 2;
                memcpy(buf, str, len);
                buf += len;
            }
            break;
        case 'Q':
            {
                FSQID *qid;
                
               ((void) sizeof (__assert_single_arg (
               buf + 13 <= buf_end
               )), __extension__ ({ if (
               buf + 13 <= buf_end
               ) ; else __assert_fail (
               "buf + 13 <= buf_end"
               , "virtio.c", 1843, __extension__ __PRETTY_FUNCTION__); }))
                                          ;
                qid = 
                     __builtin_va_arg(
                     ap
                     ,
                     FSQID *
                     )
                                        ;




                buf[0] = qid->type;
                put_le32(buf + 1, qid->version);
                put_le64(buf + 5, qid->path);
                buf += 13;
            }
            break;
        default:
            abort();
        }
    }
    
   __builtin_va_end(
   ap
   )
             ;
    return buf - buf1;
}



static int unmarshall(VIRTIO9PDevice *s, int queue_idx,
                      int desc_idx, int *poffset, const char *fmt, ...)
{
    VIRTIODevice *s1 = (VIRTIODevice *)s;
    va_list ap;
    int offset, c;
    uint8_t buf[16];

    offset = *poffset;
    
   __builtin_c23_va_start(
   ap, fmt
   )
                    ;
    for(;;) {
        c = *fmt++;
        if (c == '\0')
            break;
        switch(c) {
        case 'b':
            {
                uint8_t *ptr;
                if (memcpy_from_queue(s1, buf, queue_idx, desc_idx, offset, 1))
                    return -1;
                ptr = 
                     __builtin_va_arg(
                     ap
                     ,
                     uint8_t *
                     )
                                          ;
                *ptr = buf[0];
                offset += 1;




            }
            break;
        case 'h':
            {
                uint16_t *ptr;
                if (memcpy_from_queue(s1, buf, queue_idx, desc_idx, offset, 2))
                    return -1;
                ptr = 
                     __builtin_va_arg(
                     ap
                     ,
                     uint16_t *
                     )
                                           ;
                *ptr = get_le16(buf);
                offset += 2;




            }
            break;
        case 'w':
            {
                uint32_t *ptr;
                if (memcpy_from_queue(s1, buf, queue_idx, desc_idx, offset, 4))
                    return -1;
                ptr = 
                     __builtin_va_arg(
                     ap
                     ,
                     uint32_t *
                     )
                                           ;
                *ptr = get_le32(buf);
                offset += 4;




            }
            break;
        case 'd':
            {
                uint64_t *ptr;
                if (memcpy_from_queue(s1, buf, queue_idx, desc_idx, offset, 8))
                    return -1;
                ptr = 
                     __builtin_va_arg(
                     ap
                     ,
                     uint64_t *
                     )
                                           ;
                *ptr = get_le64(buf);
                offset += 8;




            }
            break;
        case 's':
            {
                char *str, **ptr;
                int len;

                if (memcpy_from_queue(s1, buf, queue_idx, desc_idx, offset, 2))
                    return -1;
                len = get_le16(buf);
                offset += 2;
                str = malloc(len + 1);
                if (memcpy_from_queue(s1, str, queue_idx, desc_idx, offset, len))
                    return -1;
                str[len] = '\0';
                offset += len;
                ptr = 
                     __builtin_va_arg(
                     ap
                     ,
                     char **
                     )
                                        ;
                *ptr = str;




            }
            break;
        default:
            abort();
        }
    }
    
   __builtin_va_end(
   ap
   )
             ;
    *poffset = offset;
    return 0;
}

static void virtio_9p_send_reply(VIRTIO9PDevice *s, int queue_idx,
                                 int desc_idx, uint8_t id, uint16_t tag,
                                 uint8_t *buf, int buf_len)
{
    uint8_t *buf1;
    int len;
    len = buf_len + 7;
    buf1 = malloc(len);
    put_le32(buf1, len);
    buf1[4] = id + 1;
    put_le16(buf1 + 5, tag);
    memcpy(buf1 + 7, buf, buf_len);
    memcpy_to_queue((VIRTIODevice *)s, queue_idx, desc_idx, 0, buf1, len);
    virtio_consume_desc((VIRTIODevice *)s, queue_idx, desc_idx, len);
    free(buf1);
}

static void virtio_9p_send_error(VIRTIO9PDevice *s, int queue_idx,
                                 int desc_idx, uint16_t tag, uint32_t error)
{
    uint8_t buf[4];
    int buf_len;

    buf_len = marshall(s, buf, sizeof(buf), "w", -error);
    virtio_9p_send_reply(s, queue_idx, desc_idx, 6, tag, buf, buf_len);
}

typedef struct {
    VIRTIO9PDevice *dev;
    int queue_idx;
    int desc_idx;
    uint16_t tag;
} P9OpenInfo;

static void virtio_9p_open_reply(FSDevice *fs, FSQID *qid, int err,
                                 P9OpenInfo *oi)
{
    VIRTIO9PDevice *s = oi->dev;
    uint8_t buf[32];
    int buf_len;

    if (err < 0) {
        virtio_9p_send_error(s, oi->queue_idx, oi->desc_idx, oi->tag, err);
    } else {
        buf_len = marshall(s, buf, sizeof(buf),
                           "Qw", qid, s->msize - 24);
        virtio_9p_send_reply(s, oi->queue_idx, oi->desc_idx, 12, oi->tag,
                             buf, buf_len);
    }
    void virtio_9p_open_reply__part3909(void *oi);
    virtio_9p_open_reply__part3909(oi);

}

static void virtio_9p_open_cb(FSDevice *fs, FSQID *qid, int err,
                              void *opaque)
{
    P9OpenInfo *oi = opaque;
    VIRTIO9PDevice *s = oi->dev;
    int queue_idx = oi->queue_idx;

    virtio_9p_open_reply(fs, qid, err, oi);

    s->req_in_progress = FALSE;


    queue_notify((VIRTIODevice *)s, queue_idx);
}

static int virtio_9p_recv_request(VIRTIODevice *s1, int queue_idx,
                                   int desc_idx, int read_size,
                                   int write_size)
{
    VIRTIO9PDevice *s = (VIRTIO9PDevice *)s1;
    int offset, header_len;
    uint8_t id;
    uint16_t tag;
    uint8_t buf[1024];
    int buf_len, err;
    FSDevice *fs = s->fs;

    int virtio_9p_recv_request__part3913(int queue_idx);
    if (virtio_9p_recv_request__part3913(queue_idx))
        return 0;


        int virtio_9p_recv_request__part3914(int req_in_progress);
    if (virtio_9p_recv_request__part3914(s->req_in_progress))
        return -1;

    void virtio_9p_recv_request__part3915(int *offset, int *header_len);
    virtio_9p_recv_request__part3915(&offset, &header_len);

    if (memcpy_from_queue(s1, buf, queue_idx, desc_idx, offset, header_len)) {
        tag = 0;
        goto protocol_error;
    }

    void virtio_9p_recv_request__part3917(const uint8_t *buf, int header_len, uint8_t *id, uint16_t *tag, int *offset);
    virtio_9p_recv_request__part3917(buf, header_len, &id, &tag, &offset);

    switch(id) {
    case 8:
        {
            FSStatFS st;

            fs->fs_statfs(fs, &st);
            buf_len = marshall(s, buf, sizeof(buf),
                               "wwddddddw",
                               0,
                               st.f_bsize,
                               st.f_blocks,
                               st.f_bfree,
                               st.f_bavail,
                               st.f_files,
                               st.f_ffree,
                               0,
                               256
                               );
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, buf, buf_len);
        }
        break;
    case 12:
        {
            uint32_t fid, flags;
            FSFile *f;
            FSQID qid;
            P9OpenInfo *oi;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "ww", &fid, &flags))
                goto protocol_error;
            f = fid_find(s, fid);
            if (!f)
                goto fid_not_found;
            oi = malloc(sizeof(*oi));
            oi->dev = s;
            oi->queue_idx = queue_idx;
            oi->desc_idx = desc_idx;
            oi->tag = tag;
            err = fs->fs_open(fs, &qid, f, flags, virtio_9p_open_cb, oi);
            if (err <= 0) {
                virtio_9p_open_reply(fs, &qid, err, oi);
            } else {
                s->req_in_progress = TRUE;
            }
        }
        break;
    case 14:
        {
            uint32_t fid, flags, mode, gid;
            char *name;
            FSFile *f;
            FSQID qid;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "wswww", &fid, &name, &flags, &mode, &gid))
                goto protocol_error;
            f = fid_find(s, fid);
            if (!f) {
                err = -71;
            } else {
                err = fs->fs_create(fs, &qid, f, name, flags, mode, gid);
            }
            free(name);
            if (err)
                goto error;
            buf_len = marshall(s, buf, sizeof(buf),
                               "Qw", &qid, s->msize - 24);
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, buf, buf_len);
        }
        break;
    case 16:
        {
            uint32_t fid, gid;
            char *name, *symgt;
            FSFile *f;
            FSQID qid;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "wssw", &fid, &name, &symgt, &gid))
                goto protocol_error;
            f = fid_find(s, fid);
            if (!f) {
                err = -71;
            } else {
                err = fs->fs_symlink(fs, &qid, f, name, symgt, gid);
            }
            free(name);
            free(symgt);
            if (err)
                goto error;
            buf_len = marshall(s, buf, sizeof(buf),
                               "Q", &qid);
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, buf, buf_len);
        }
        break;
    case 18:
        {
            uint32_t fid, mode, major, minor, gid;
            char *name;
            FSFile *f;
            FSQID qid;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "wswwww", &fid, &name, &mode, &major, &minor, &gid))
                goto protocol_error;
            f = fid_find(s, fid);
            if (!f) {
                err = -71;
            } else {
                err = fs->fs_mknod(fs, &qid, f, name, mode, major, minor, gid);
            }
            free(name);
            if (err)
                goto error;
            buf_len = marshall(s, buf, sizeof(buf),
                               "Q", &qid);
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, buf, buf_len);
        }
        break;
    case 22:
        {
            uint32_t fid;
            char buf1[1024];
            FSFile *f;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "w", &fid))
                goto protocol_error;
            f = fid_find(s, fid);
            if (!f) {
                err = -71;
            } else {
                err = fs->fs_readlink(fs, buf1, sizeof(buf1), f);
            }
            if (err)
                goto error;
            buf_len = marshall(s, buf, sizeof(buf), "s", buf1);
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, buf, buf_len);
        }
        break;
    case 24:
        {
            uint32_t fid;
            uint64_t mask;
            FSFile *f;
            FSStat st;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "wd", &fid, &mask))
                goto protocol_error;
            f = fid_find(s, fid);
            if (!f)
                goto fid_not_found;
            err = fs->fs_stat(fs, f, &st);
            if (err)
                goto error;

            buf_len = marshall(s, buf, sizeof(buf),
                               "dQwwwddddddddddddddd",
                               mask, &st.qid,
                               st.st_mode, st.st_uid, st.st_gid,
                               st.st_nlink, st.st_rdev, st.st_size,
                               st.st_blksize, st.st_blocks,
                               st.st_atime_sec, (uint64_t)st.st_atime_nsec,
                               st.st_mtime_sec, (uint64_t)st.st_mtime_nsec,
                               st.st_ctime_sec, (uint64_t)st.st_ctime_nsec,
                               (uint64_t)0, (uint64_t)0,
                               (uint64_t)0, (uint64_t)0);
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, buf, buf_len);
        }
        break;
    case 26:
        {
            uint32_t fid, mask, mode, uid, gid;
            uint64_t size, atime_sec, atime_nsec, mtime_sec, mtime_nsec;
            FSFile *f;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "wwwwwddddd", &fid, &mask, &mode, &uid, &gid,
                           &size, &atime_sec, &atime_nsec,
                           &mtime_sec, &mtime_nsec))
                goto protocol_error;
            f = fid_find(s, fid);
            if (!f)
                goto fid_not_found;
            err = fs->fs_setattr(fs, f, mask, mode, uid, gid, size, atime_sec,
                                 atime_nsec, mtime_sec, mtime_nsec);
            if (err)
                goto error;
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, 
                                                                 ((void *)0)
                                                                     , 0);
        }
        break;
    case 30:
        {

            err = -524;
            goto error;
        }
        break;
    case 40:
        {
            uint32_t fid, count;
            uint64_t offs;
            uint8_t *buf;
            int n;
            FSFile *f;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "wdw", &fid, &offs, &count))
                goto protocol_error;
            f = fid_find(s, fid);
            if (!f)
                goto fid_not_found;
            buf = malloc(count + 4);
            n = fs->fs_readdir(fs, f, offs, buf + 4, count);
            if (n < 0) {
                err = n;
                goto error;
            }
            put_le32(buf, n);
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, buf, n + 4);
            free(buf);
        }
        break;
    case 50:
        {
            uint32_t fid;
            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "w", &fid))
                goto protocol_error;

            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, 
                                                                 ((void *)0)
                                                                     , 0);
        }
        break;
    case 52:
        {
            uint32_t fid;
            FSFile *f;
            FSLock lock;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "wbwddws", &fid, &lock.type, &lock.flags,
                           &lock.start, &lock.length,
                           &lock.proc_id, &lock.client_id))
                goto protocol_error;
            f = fid_find(s, fid);
            if (!f)
                err = -71;
            else
                err = fs->fs_lock(fs, f, &lock);
            free(lock.client_id);
            if (err < 0)
                goto error;
            buf_len = marshall(s, buf, sizeof(buf), "b", err);
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, buf, buf_len);
        }
        break;
    case 54:
        {
            uint32_t fid;
            FSFile *f;
            FSLock lock;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "wbddws", &fid, &lock.type,
                           &lock.start, &lock.length,
                           &lock.proc_id, &lock.client_id))
                goto protocol_error;
            f = fid_find(s, fid);
            if (!f)
                err = -71;
            else
                err = fs->fs_getlock(fs, f, &lock);
            if (err < 0) {
                free(lock.client_id);
                goto error;
            }
            buf_len = marshall(s, buf, sizeof(buf), "bddws",
                               &lock.type,
                               &lock.start, &lock.length,
                               &lock.proc_id, &lock.client_id);
            free(lock.client_id);
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, buf, buf_len);
        }
        break;
    case 70:
        {
            uint32_t dfid, fid;
            char *name;
            FSFile *f, *df;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "wws", &dfid, &fid, &name))
                goto protocol_error;
            df = fid_find(s, dfid);
            f = fid_find(s, fid);
            if (!df || !f) {
                err = -71;
            } else {
                err = fs->fs_link(fs, df, f, name);
            }
            free(name);
            if (err)
                goto error;
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, 
                                                                 ((void *)0)
                                                                     , 0);
        }
        break;
    case 72:
        {
            uint32_t fid, mode, gid;
            char *name;
            FSFile *f;
            FSQID qid;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "wsww", &fid, &name, &mode, &gid))
                goto protocol_error;
            f = fid_find(s, fid);
            if (!f)
                goto fid_not_found;
            err = fs->fs_mkdir(fs, &qid, f, name, mode, gid);
            if (err != 0)
                goto error;
            buf_len = marshall(s, buf, sizeof(buf), "Q", &qid);
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, buf, buf_len);
        }
        break;
    case 74:
        {
            uint32_t fid, new_fid;
            char *name, *new_name;
            FSFile *f, *new_f;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "wsws", &fid, &name, &new_fid, &new_name))
                goto protocol_error;
            f = fid_find(s, fid);
            new_f = fid_find(s, new_fid);
            if (!f || !new_f) {
                err = -71;
            } else {
                err = fs->fs_renameat(fs, f, name, new_f, new_name);
            }
            free(name);
            free(new_name);
            if (err != 0)
                goto error;
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, 
                                                                 ((void *)0)
                                                                     , 0);
        }
        break;
    case 76:
        {
            uint32_t fid, flags;
            char *name;
            FSFile *f;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "wsw", &fid, &name, &flags))
                goto protocol_error;
            f = fid_find(s, fid);
            if (!f) {
                err = -71;
            } else {
                err = fs->fs_unlinkat(fs, f, name);
            }
            free(name);
            if (err != 0)
                goto error;
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, 
                                                                 ((void *)0)
                                                                     , 0);
        }
        break;
    case 100:
        {
            uint32_t msize;
            char *version;
            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "ws", &msize, &version))
                goto protocol_error;
            s->msize = msize;

            free(version);
            buf_len = marshall(s, buf, sizeof(buf), "ws", s->msize, "9P2000.L");
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, buf, buf_len);
        }
        break;
    case 104:
        {
            uint32_t fid, afid, uid;
            char *uname, *aname;
            FSQID qid;
            FSFile *f;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "wwssw", &fid, &afid, &uname, &aname, &uid))
                goto protocol_error;
            err = fs->fs_attach(fs, &f, &qid, uid, uname, aname);
            if (err != 0)
                goto error;
            fid_set(s, fid, f);
            free(uname);
            free(aname);
            buf_len = marshall(s, buf, sizeof(buf), "Q", &qid);
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, buf, buf_len);
        }
        break;
    case 108:
        {
            uint16_t oldtag;
            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "h", &oldtag))
                goto protocol_error;

            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, 
                                                                 ((void *)0)
                                                                     , 0);
        }
        break;
    case 110:
        {
            uint32_t fid, newfid;
            uint16_t nwname;
            FSQID *qids;
            char **names;
            FSFile *f;
            int i;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "wwh", &fid, &newfid, &nwname))
                goto protocol_error;
            f = fid_find(s, fid);
            if (!f)
                goto fid_not_found;
            names = mallocz(sizeof(names[0]) * nwname);
            qids = malloc(sizeof(qids[0]) * nwname);
            for(i = 0; i < nwname; i++) {
                if (unmarshall(s, queue_idx, desc_idx, &offset,
                               "s", &names[i])) {
                    err = -71;
                    goto walk_done;
                }
            }
            err = fs->fs_walk(fs, &f, qids, f, nwname, names);
        walk_done:
            for(i = 0; i < nwname; i++) {
                free(names[i]);
            }
            free(names);
            if (err < 0) {
                free(qids);
                goto error;
            }
            buf_len = marshall(s, buf, sizeof(buf), "h", err);
            for(i = 0; i < err; i++) {
                buf_len += marshall(s, buf + buf_len, sizeof(buf) - buf_len,
                                    "Q", &qids[i]);
            }
            free(qids);
            fid_set(s, newfid, f);
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, buf, buf_len);
        }
        break;
    case 116:
        {
            uint32_t fid, count;
            uint64_t offs;
            uint8_t *buf;
            int n;
            FSFile *f;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "wdw", &fid, &offs, &count))
                goto protocol_error;
            f = fid_find(s, fid);
            if (!f)
                goto fid_not_found;
            buf = malloc(count + 4);
            n = fs->fs_read(fs, f, offs, buf + 4, count);
            if (n < 0) {
                err = n;
                free(buf);
                goto error;
            }
            put_le32(buf, n);
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, buf, n + 4);
            free(buf);
        }
        break;
    case 118:
        {
            uint32_t fid, count;
            uint64_t offs;
            uint8_t *buf1;
            int n;
            FSFile *f;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "wdw", &fid, &offs, &count))
                goto protocol_error;
            f = fid_find(s, fid);
            if (!f)
                goto fid_not_found;
            buf1 = malloc(count);
            if (memcpy_from_queue(s1, buf1, queue_idx, desc_idx, offset,
                                  count)) {
                free(buf1);
                goto protocol_error;
            }
            n = fs->fs_write(fs, f, offs, buf1, count);
            free(buf1);
            if (n < 0) {
                err = n;
                goto error;
            }
            buf_len = marshall(s, buf, sizeof(buf), "w", n);
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, buf, buf_len);
        }
        break;
    case 120:
        {
            uint32_t fid;

            if (unmarshall(s, queue_idx, desc_idx, &offset,
                           "w", &fid))
                goto protocol_error;
            fid_delete(s, fid);
            virtio_9p_send_reply(s, queue_idx, desc_idx, id, tag, 
                                                                 ((void *)0)
                                                                     , 0);
        }
        break;
    default:
        printf("9p: unsupported operation id=%d\n", id);
        goto protocol_error;
    }
        {
        int virtio_9p_recv_request__part3919(void);
        return virtio_9p_recv_request__part3919();
    }

 error:
    virtio_9p_send_error(s, queue_idx, desc_idx, tag, err);
    int virtio_9p_recv_request__part3921(void);
    return virtio_9p_recv_request__part3921();
  protocol_error:
 fid_not_found:
    {
        void virtio_9p_recv_request__part3922(int *err);
        virtio_9p_recv_request__part3922(&err);
    }
    goto error;

        {
        int virtio_9p_recv_request__part3923(void);
        if (virtio_9p_recv_request__part3923())
            goto error;
    }

}

VIRTIODevice *virtio_9p_init(VIRTIOBusDef *bus, FSDevice *fs,
                             const char *mount_tag)

{
    VIRTIO9PDevice *s;
    int len;
    uint8_t *cfg;

    len = strlen(mount_tag);
    s = mallocz(sizeof(*s));
    virtio_init(&s->common, bus,
                9, 2 + len, virtio_9p_recv_request);
    s->common.device_features = 1 << 0;


    cfg = s->common.config_space;
    cfg[0] = len;
    cfg[1] = len >> 8;
    memcpy(cfg + 2, mount_tag, len);

    s->fs = fs;
    s->msize = 8192;
    init_list_head(&s->fid_list);

    return (VIRTIODevice *)s;
}
