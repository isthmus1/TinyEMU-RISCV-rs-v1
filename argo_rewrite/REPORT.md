# Argo rewrite of https://github.com/fernandotcl/TinyEMU.git

2424 of 3751 functions exported into 52 modules; 1327 left out.

Every exported function compiled on its own with rustc when its worker checked it. `cargo build` here checks that they also compile together as one crate.

| Function | File | Status | Detail |
|---|---|---|---|
| `AES_set_encrypt_key` | aes.c | left out | not translated yet |
| `if (!userKey || !key)` | aes.c | exported | Unvalidated |
| `if (bits != 128 && bits != 192 && bits != 256)` | aes.c | exported | Unvalidated |
| `rk = key->rd_key;` | aes.c | exported | Unvalidated |
| `if (bits==128) … else` | aes.c | exported | Unvalidated |
| `rk[0] = (((u32)(userKey)[0] << 24) ^ ((u32)(userKey)[1] << 16) ^ ((u32)(userKey)[2] << 8) ^ ((u…` | aes.c | exported | Unvalidated |
| `if (bits == 128)` | aes.c | exported | Unvalidated |
| `rk[4] = (((u32)(userKey + 16)[0] << 24) ^ ((u32)(userKey + 16)[1] << 16) ^ ((u32)(userKey + 16)…` | aes.c | exported | Unvalidated |
| `if (bits == 192)` | aes.c | exported | Unvalidated |
| `rk[6] = (((u32)(userKey + 24)[0] << 24) ^ ((u32)(userKey + 24)[1] << 16) ^ ((u32)(userKey + 24)…` | aes.c | exported | Unvalidated |
| `if (bits == 256)` | aes.c | exported | Unvalidated |
| `return 0;` | aes.c | exported | Unvalidated |
| `AES_set_decrypt_key` | aes.c | left out | not translated yet |
| `u32 *rk; int i, j, status; u32 temp; status = AES_set_encrypt_key(userKey, bits, key);` | aes.c | exported | Unvalidated |
| `if (status < 0)` | aes.c | exported | Unvalidated |
| `rk = key->rd_key;` | aes.c | exported | Unvalidated |
| `for (i = 0, j = 4*(key->rounds; i < j; i += 4, j -= 4)` | aes.c | exported | Unvalidated |
| `for (i = 1; i < (key->rounds; i++)` | aes.c | exported | Unvalidated |
| `return 0;` | aes.c | exported | Unvalidated |
| `AES_encrypt` | aes.c | left out | not translated yet |
| `const u32 *rk; u32 s0, s1, s2, s3, t0, t1, t2, t3; int r; ((void) (0)) ; rk = key->rd_key; s0 =…` | aes.c | exported | Unvalidated |
| `for (; ; )` | aes.c | exported | Unvalidated |
| `s0 = (Te4[(t0 >> 24) ] & 0xff000000) ^ (Te4[(t1 >> 16) & 0xff] & 0x00ff0000) ^ (Te4[(t2 >> 8) &…` | aes.c | exported | Unvalidated |
| `{ … }` | aes.c | exported | Unvalidated |
| `; s1 = (Te4[(t1 >> 24) ] & 0xff000000) ^ (Te4[(t2 >> 16) & 0xff] & 0x00ff0000) ^ (Te4[(t3 >> 8)…` | aes.c | exported | Unvalidated |
| `{ … }` | aes.c | exported | Unvalidated |
| `; s2 = (Te4[(t2 >> 24) ] & 0xff000000) ^ (Te4[(t3 >> 16) & 0xff] & 0x00ff0000) ^ (Te4[(t0 >> 8)…` | aes.c | exported | Unvalidated |
| `{ … }` | aes.c | exported | Unvalidated |
| `; s3 = (Te4[(t3 >> 24) ] & 0xff000000) ^ (Te4[(t0 >> 16) & 0xff] & 0x00ff0000) ^ (Te4[(t1 >> 8)…` | aes.c | exported | Unvalidated |
| `{ … }` | aes.c | exported | Unvalidated |
| `;` | aes.c | exported | Unvalidated |
| `AES_decrypt` | aes.c | left out | not translated yet |
| `const u32 *rk; u32 s0, s1, s2, s3, t0, t1, t2, t3; int r; ((void) (0)) ; rk = key->rd_key; s0 =…` | aes.c | exported | Unvalidated |
| `for (; ; )` | aes.c | exported | Unvalidated |
| `s0 = (Td4[(t0 >> 24) ] & 0xff000000) ^ (Td4[(t3 >> 16) & 0xff] & 0x00ff0000) ^ (Td4[(t2 >> 8) &…` | aes.c | exported | Unvalidated |
| `{ … }` | aes.c | exported | Unvalidated |
| `; s1 = (Td4[(t1 >> 24) ] & 0xff000000) ^ (Td4[(t0 >> 16) & 0xff] & 0x00ff0000) ^ (Td4[(t3 >> 8)…` | aes.c | exported | Unvalidated |
| `{ … }` | aes.c | exported | Unvalidated |
| `; s2 = (Td4[(t2 >> 24) ] & 0xff000000) ^ (Td4[(t1 >> 16) & 0xff] & 0x00ff0000) ^ (Td4[(t0 >> 8)…` | aes.c | exported | Unvalidated |
| `{ … }` | aes.c | exported | Unvalidated |
| `; s3 = (Td4[(t3 >> 24) ] & 0xff000000) ^ (Td4[(t2 >> 16) & 0xff] & 0x00ff0000) ^ (Td4[(t1 >> 8)…` | aes.c | exported | Unvalidated |
| `{ … }` | aes.c | exported | Unvalidated |
| `;` | aes.c | exported | Unvalidated |
| `AES_cbc_encrypt` | aes.c | left out | not translated yet |
| `unsigned long n; unsigned long len = length; unsigned char tmp[16]; ((void) (0)) ;` | aes.c | exported | Unvalidated |
| `if (enc) … else` | aes.c | exported | Unvalidated |
| `bf_find_block` | block_net.c | left out | not translated yet |
| `for (el = (&bf->cached_blocks)->next; el != (&bf->cached_blocks; el = el->next)` | block_net.c | exported | Unvalidated |
| `return ((void *)0) ;` | block_net.c | exported | Unvalidated |
| `bf_free_block` | block_net.c | left out | not translated yet |
| `bf->n_cached_blocks--; file_buffer_reset(&b->fbuf); list_del(&b->link); free(b);` | block_net.c | exported | Unvalidated |
| `bf_add_block` | block_net.c | left out | not translated yet |
| `if (bf->n_cached_blocks >= bf->n_cached_blocks_max)` | block_net.c | exported | Unvalidated |
| `b = mallocz(sizeof(CachedBlock)); b->bf = bf; b->block_num = block_num; b->state = CBLOCK_LOADI…` | block_net.c | exported | Unvalidated |
| `bf_get_sector_count` | block_net.c | left out | not translated yet |
| `BlockDeviceHTTP *bf = bs->opaque; return bf->nb_sectors;` | block_net.c | exported | Unvalidated |
| `bf_start_load_block` | block_net.c | left out | not translated yet |
| `BlockDeviceHTTP *bf = bs->opaque; char filename[1024]; CachedBlock *b; b = bf_add_block(bf, blo…` | block_net.c | exported | Unvalidated |
| `bf_start_load_prefetch_group` | block_net.c | left out | not translated yet |
| `BlockDeviceHTTP *bf = bs->opaque; CachedBlock *b; PrefetchGroupRequest *req; char filename[1024…` | block_net.c | exported | Unvalidated |
| `for (i = 0; i < n_block_num; i++)` | block_net.c | exported | Unvalidated |
| `if (req_flag) … else` | block_net.c | exported | Unvalidated |
| `bf_prefetch_group_onload` | block_net.c | left out | not translated yet |
| `if (err < 0)` | block_net.c | exported | Unvalidated |
| `block_bytes = bf->block_size * 512; ((void) sizeof (__assert_single_arg ( size == block_bytes *…` | block_net.c | exported | Unvalidated |
| `for (i = 0; i < req->n_block_num; i++)` | block_net.c | exported | Unvalidated |
| `free(req);` | block_net.c | exported | Unvalidated |
| `bf_rw_async1` | block_net.c | left out | not translated yet |
| `for (; ; )` | block_net.c | exported | Unvalidated |
| `if (!is_sync)` | block_net.c | exported | Unvalidated |
| `return 0;` | block_net.c | exported | Unvalidated |
| `bf_update_block` | block_net.c | left out | not translated yet |
| `BlockDeviceHTTP *bf = b->bf; BlockDevice *bs = bf->bs; ((void) sizeof (__assert_single_arg ( b-…` | block_net.c | exported | Unvalidated |
| `if (b->block_num == bf->cur_block_num)` | block_net.c | exported | Unvalidated |
| `bf_read_onload` | block_net.c | left out | not translated yet |
| `if (err < 0)` | block_net.c | exported | Unvalidated |
| `((void) sizeof (__assert_single_arg ( size == bf->block_size * 512 )), __extension__ ({ if ( si…` | block_net.c | exported | Unvalidated |
| `bf_read_async` | block_net.c | left out | not translated yet |
| `BlockDeviceHTTP *bf = bs->opaque; bf->is_write = FALSE; bf->sector_num = sector_num; bf->io_buf…` | block_net.c | exported | Unvalidated |
| `bf_write_async` | block_net.c | left out | not translated yet |
| `BlockDeviceHTTP *bf = bs->opaque; bf->is_write = TRUE; bf->sector_num = sector_num; bf->io_buf…` | block_net.c | exported | Unvalidated |
| `block_device_init_http` | block_net.c | left out | not translated yet |
| `BlockDevice *bs; BlockDeviceHTTP *bf; char *p; bs = mallocz(sizeof(*bs)); bf = mallocz(sizeof(*…` | block_net.c | exported | Unvalidated |
| `if (!p) … else` | block_net.c | exported | Unvalidated |
| `*p = '\0'; init_list_head(&bf->cached_blocks); bf->max_cache_size_kb = max_cache_size_kb; bf->s…` | block_net.c | exported | Unvalidated |
| `bf_init_onload` | block_net.c | left out | not translated yet |
| `if (err < 0)` | block_net.c | exported | Unvalidated |
| `cfg = json_parse_value_len(data, size);` | block_net.c | exported | Unvalidated |
| `if (json_is_error(cfg))` | block_net.c | exported | Unvalidated |
| `if (vm_get_int(cfg, "block_size", &block_size_kb) < 0)` | block_net.c | exported | Unvalidated |
| `bf->block_size = block_size_kb * 2;` | block_net.c | exported | Unvalidated |
| `if (bf->block_size <= 0 || (bf->block_size & (bf->block_size - 1)) != 0)` | block_net.c | exported | Unvalidated |
| `if (vm_get_int(cfg, "n_block", &bf->nb_blocks) < 0)` | block_net.c | exported | Unvalidated |
| `if (bf->nb_blocks <= 0)` | block_net.c | exported | Unvalidated |
| `bf->nb_sectors = bf->block_size * (uint64_t)bf->nb_blocks; bf->n_cached_blocks = 0; bf->n_cache…` | block_net.c | exported | Unvalidated |
| `if (vm_get_int_opt(cfg, "prefetch_group_len", &bf->prefetch_group_len, 1) < 0)` | block_net.c | exported | Unvalidated |
| `if (bf->prefetch_group_len > 32)` | block_net.c | exported | Unvalidated |
| `array = json_object_get(cfg, "prefetch");` | block_net.c | exported | Unvalidated |
| `if (!json_is_undefined(array))` | block_net.c | exported | Unvalidated |
| `json_free(cfg);` | block_net.c | exported | Unvalidated |
| `if (bf->start_cb)` | block_net.c | exported | Unvalidated |
| `print_str` | build_filelist.c | left out | not translated yet |
| `const char *s; int c; s = str;` | build_filelist.c | exported | Unvalidated |
| `while (*s != '\0')` | build_filelist.c | exported | Unvalidated |
| `fputs(str, f); return;` | build_filelist.c | exported | Unvalidated |
| `use_quote: s = str;` | build_filelist.c | exported | Unvalidated |
| `fputc('"', f);` | build_filelist.c | exported | Unvalidated |
| `while (*s != '\0')` | build_filelist.c | exported | Unvalidated |
| `fputc('"', f);` | build_filelist.c | exported | Unvalidated |
| `copy_file` | build_filelist.c | left out | not translated yet |
| `uint8_t *buf; FILE *fi, *fo; int len; buf = malloc((1024 * 1024)); fi = fopen(src_filename, "rb…` | build_filelist.c | exported | Unvalidated |
| `if (!fi)` | build_filelist.c | exported | Unvalidated |
| `fo = fopen(dst_filename, "wb");` | build_filelist.c | exported | Unvalidated |
| `if (!fo)` | build_filelist.c | exported | Unvalidated |
| `for (; ; )` | build_filelist.c | exported | Unvalidated |
| `fclose(fo); fclose(fi);` | build_filelist.c | exported | Unvalidated |
| `add_file_size` | build_filelist.c | left out | not translated yet |
| `s->fs_size += block_align(size, (1 << 12));` | build_filelist.c | exported | Unvalidated |
| `if (s->fs_size > s->fs_max_size)` | build_filelist.c | exported | Unvalidated |
| `scan_dir` | build_filelist.c | left out | not translated yet |
| `FILE *f = s->f; DIR *dirp; struct dirent *de; const char *name; struct stat st; char *path1; ui…` | build_filelist.c | exported | Unvalidated |
| `if (!dirp)` | build_filelist.c | exported | Unvalidated |
| `for (; ; )` | build_filelist.c | exported | Unvalidated |
| `closedir(dirp); fprintf(f, ".\n");` | build_filelist.c | exported | Unvalidated |
| `help` | build_filelist.c | left out | not translated yet |
| `printf("usage: build_filelist [options] source_path dest_path\n" "\n" "Options:\n" "-m size_mb…` | build_filelist.c | exported | Unvalidated |
| `main` | build_filelist.c | left out | not translated yet |
| `const char *dst_path, *src_path; ScanState s_s, *s = &s_s; FILE *f; char *filename; FSFileID ro…` | build_filelist.c | exported | Unvalidated |
| `for (; ; )` | build_filelist.c | exported | Unvalidated |
| `if (optind + 1 >= argc)` | build_filelist.c | exported | Unvalidated |
| `src_path = argv[optind]; dst_path = argv[optind + 1]; mkdir(dst_path, 0755); s->files_path = co…` | build_filelist.c | exported | Unvalidated |
| `if (!f)` | build_filelist.c | exported | Unvalidated |
| `fprintf(f, "Version: 1\n"); fprintf(f, "Revision: 1\n"); fprintf(f, "\n"); s->f = f; scan_dir(s…` | build_filelist.c | exported | Unvalidated |
| `if (stat(filename, &st) < 0)` | build_filelist.c | exported | Unvalidated |
| `add_file_size(s, st.st_size); free(filename); filename = compose_path(dst_path, "head"); f = fo…` | build_filelist.c | exported | Unvalidated |
| `if (!f)` | build_filelist.c | exported | Unvalidated |
| `fprintf(f, "Version: 1\n"); fprintf(f, "Revision: 1\n"); fprintf(f, "NextFileID: %" "l" "x" "\n…` | build_filelist.c | exported | Unvalidated |
| `if (!f)` | build_filelist.c | exported | Unvalidated |
| `fclose(f); free(filename); return 0;` | build_filelist.c | exported | Unvalidated |
| `detect_algorithm` | compress.c | left out | not translated yet |
| `if (len < 2 || buf[0] != 0x1f)` | compress.c | exported | Unvalidated |
| `switch (buf[1])` | compress.c | exported | Unvalidated |
| `compress_detect_magic` | compress.c | left out | not translated yet |
| `return detect_algorithm(buf, len) != ALGORITHM_UNKNOWN;` | compress.c | exported | Unvalidated |
| `decompress` | compress.c | left out | not translated yet |
| `switch (detect_algorithm(in_buf, in_len))` | compress.c | exported | Unvalidated |
| `z_stream strm; strm.zalloc = 0 ; strm.zfree = 0 ; strm.opaque = 0 ; strm.avail_in = 0; strm.nex…` | compress.c | exported | Unvalidated |
| `if (inflateInit2_(( &strm ), ( windowBits ), "1.3.2", (int)sizeof(z_stream)) != 0)` | compress.c | exported | Unvalidated |
| `strm.avail_in = in_len; strm.next_in = (Bytef *)in_buf; strm.avail_out = out_len; strm.next_out…` | compress.c | exported | Unvalidated |
| `if (inflate(&strm, 4 ) != 1)` | compress.c | exported | Unvalidated |
| `return strm.total_out;` | compress.c | exported | Unvalidated |
| `mallocz` | cutils.c | left out | not translated yet |
| `void *ptr; ptr = malloc(size);` | cutils.c | exported | Unvalidated |
| `if (!ptr)` | cutils.c | exported | Unvalidated |
| `memset(ptr, 0, size); return ptr;` | cutils.c | exported | Unvalidated |
| `pstrcpy` | cutils.c | left out | not translated yet |
| `if (buf_size <= 0)` | cutils.c | exported | Unvalidated |
| `for (; ; )` | cutils.c | exported | Unvalidated |
| `*q = '\0';` | cutils.c | exported | Unvalidated |
| `pstrcat` | cutils.c | left out | not translated yet |
| `int len; len = strlen(buf);` | cutils.c | exported | Unvalidated |
| `if (len < buf_size)` | cutils.c | exported | Unvalidated |
| `return buf;` | cutils.c | exported | Unvalidated |
| `strstart` | cutils.c | left out | not translated yet |
| `const char *p, *q; p = str; q = val;` | cutils.c | exported | Unvalidated |
| `while (*q != '\0')` | cutils.c | exported | Unvalidated |
| `if (ptr)` | cutils.c | exported | Unvalidated |
| `return 1;` | cutils.c | exported | Unvalidated |
| `dbuf_init` | cutils.c | left out | not translated yet |
| `memset(s, 0, sizeof(*s));` | cutils.c | exported | Unvalidated |
| `dbuf_write` | cutils.c | left out | not translated yet |
| `size_t end, new_size; new_size = end = offset + len;` | cutils.c | exported | Unvalidated |
| `if (new_size > s->allocated_size)` | cutils.c | exported | Unvalidated |
| `memcpy(s->buf + offset, data, len);` | cutils.c | exported | Unvalidated |
| `if (end > s->size)` | cutils.c | exported | Unvalidated |
| `dbuf_putc` | cutils.c | left out | not translated yet |
| `dbuf_write(s, s->size, &c, 1);` | cutils.c | exported | Unvalidated |
| `dbuf_putstr` | cutils.c | left out | not translated yet |
| `dbuf_write(s, s->size, (const uint8_t *)str, strlen(str));` | cutils.c | exported | Unvalidated |
| `dbuf_free` | cutils.c | left out | not translated yet |
| `free(s->buf); memset(s, 0, sizeof(*s));` | cutils.c | exported | Unvalidated |
| `max_int` | cutils.h | left out | not translated yet |
| `if (a > b) … else` | cutils.h | exported | Unvalidated |
| `min_int` | cutils.h | left out | not translated yet |
| `if (a < b) … else` | cutils.h | exported | Unvalidated |
| `get_le16` | cutils.h | left out | not translated yet |
| `return ptr[0] | (ptr[1] << 8);` | cutils.h | exported | Unvalidated |
| `get_le32` | cutils.h | left out | not translated yet |
| `return ptr[0] | (ptr[1] << 8) | (ptr[2] << 16) | (ptr[3] << 24);` | cutils.h | exported | Unvalidated |
| `get_le64` | cutils.h | left out | not translated yet |
| `return get_le32(ptr) | ((uint64_t)get_le32(ptr + 4) << 32);` | cutils.h | exported | Unvalidated |
| `put_le16` | cutils.h | left out | not translated yet |
| `ptr[0] = v; ptr[1] = v >> 8;` | cutils.h | exported | Unvalidated |
| `put_le32` | cutils.h | left out | not translated yet |
| `ptr[0] = v; ptr[1] = v >> 8; ptr[2] = v >> 16; ptr[3] = v >> 24;` | cutils.h | exported | Unvalidated |
| `put_le64` | cutils.h | left out | not translated yet |
| `put_le32(ptr, v); put_le32(ptr + 4, v >> 32);` | cutils.h | exported | Unvalidated |
| `get_be32` | cutils.h | left out | not translated yet |
| `return (d[0] << 24) | (d[1] << 16) | (d[2] << 8) | d[3];` | cutils.h | exported | Unvalidated |
| `put_be32` | cutils.h | left out | not translated yet |
| `d[0] = v >> 24; d[1] = v >> 16; d[2] = v >> 8; d[3] = v >> 0;` | cutils.h | exported | Unvalidated |
| `put_be64` | cutils.h | left out | not translated yet |
| `put_be32(d, v >> 32); put_be32(d + 4, v);` | cutils.h | exported | Unvalidated |
| `cpu_to_be32` | cutils.h | left out | not translated yet |
| `return __bswap_32 ( v ) ;` | cutils.h | exported | Unvalidated |
| `ctz32` | cutils.h | left out | not translated yet |
| `return val ? __builtin_ctz(val) : 32;` | cutils.h | exported | Unvalidated |
| `elf_detect_magic` | elf.c | left out | not translated yet |
| `if (buf_len < sizeof(elf_ident_t))` | elf.c | exported | Unvalidated |
| `elf_ident_t *ident = (void *)buf; return (ident->magic[0] == 0x7f && ident->magic[1] == 'E' &&…` | elf.c | exported | Unvalidated |
| `elf_load` | elf.c | left out | not translated yet |
| `for (i = 0; i < phnum; i++)` | elf.c | exported | Unvalidated |
| `for (i = 0; i < phnum; i++)` | elf.c | exported | Unvalidated |
| `if (start != ((void *)0)` | elf.c | exported | Unvalidated |
| `if (len != ((void *)0)` | elf.c | exported | Unvalidated |
| `return 0;` | elf.c | exported | Unvalidated |
| `elf_find_section` | elf.c | left out | not translated yet |
| `if (shstrndx == 0)` | elf.c | exported | Unvalidated |
| `if (table_len == 0)` | elf.c | exported | Unvalidated |
| `for (i = 0; i < shnum; i++)` | elf.c | exported | Unvalidated |
| `return 0;` | elf.c | exported | Unvalidated |
| `fs_dup` | fs.c | left out | not translated yet |
| `FSQID qid; fs->fs_walk(fs, &f, &qid, f, 0, ((void *)0) ); return f;` | fs.c | exported | Unvalidated |
| `fs_walk_path1` | fs.c | left out | not translated yet |
| `if (path[0] == '/')` | fs.c | exported | Unvalidated |
| `is_first = TRUE;` | fs.c | exported | Unvalidated |
| `for (; ; )` | fs.c | exported | Unvalidated |
| `return f;` | fs.c | exported | Unvalidated |
| `fs_walk_path` | fs.c | left out | not translated yet |
| `return fs_walk_path1(fs, f, path, ((void *)0) );` | fs.c | exported | Unvalidated |
| `fs_end` | fs.c | left out | not translated yet |
| `fs->fs_end(fs); free(fs);` | fs.c | exported | Unvalidated |
| `fs_delete` | fs_disk.c | left out | not translated yet |
| `if (f->is_opened)` | fs_disk.c | exported | Unvalidated |
| `free(f->path); free(f);` | fs_disk.c | exported | Unvalidated |
| `fid_create` | fs_disk.c | left out | not translated yet |
| `FSFile *f; f = mallocz(sizeof(*f)); f->path = path; f->uid = uid; return f;` | fs_disk.c | exported | Unvalidated |
| `errno_to_p9` | fs_disk.c | left out | not translated yet |
| `if (err == 0)` | fs_disk.c | exported | Unvalidated |
| `for (i = 0; i < (sizeof(errno_table) / sizeof(errno_table[0]; i++)` | fs_disk.c | exported | Unvalidated |
| `return 22;` | fs_disk.c | exported | Unvalidated |
| `p9_flags_to_host` | fs_disk.c | left out | not translated yet |
| `int ret, i; ret = (flags & 0x00000003);` | fs_disk.c | exported | Unvalidated |
| `for (i = 0; i < (sizeof(open_flags) / sizeof(open_flags[0]; i++)` | fs_disk.c | exported | Unvalidated |
| `return ret;` | fs_disk.c | exported | Unvalidated |
| `stat_to_qid` | fs_disk.c | left out | not translated yet |
| `if (st->st_mode )) & 0170000) == (0040000) … else` | fs_disk.c | exported | Unvalidated |
| `qid->version = 0; qid->path = st->st_ino;` | fs_disk.c | exported | Unvalidated |
| `fs_statfs` | fs_disk.c | left out | not translated yet |
| `FSDeviceDisk *fs = (FSDeviceDisk *)fs1; struct statfs st1; statfs(fs->root_path, &st1); st->f_b…` | fs_disk.c | exported | Unvalidated |
| `compose_path` | fs_disk.c | left out | not translated yet |
| `int path_len, name_len; char *d; path_len = strlen(path); name_len = strlen(name); d = malloc(p…` | fs_disk.c | exported | Unvalidated |
| `fs_attach` | fs_disk.c | left out | not translated yet |
| `if (lstat(fs->root_path, &st) != 0)` | fs_disk.c | exported | Unvalidated |
| `f = fid_create(fs1, strdup(fs->root_path), uid); stat_to_qid(qid, &st); *pf = f; return 0;` | fs_disk.c | exported | Unvalidated |
| `fs_walk` | fs_disk.c | left out | not translated yet |
| `char *path, *path1; struct stat st; int i; path = strdup(f->path);` | fs_disk.c | exported | Unvalidated |
| `for (i = 0; i < n; i++)` | fs_disk.c | exported | Unvalidated |
| `*pf = fid_create(fs, path, f->uid); return i;` | fs_disk.c | exported | Unvalidated |
| `fs_mkdir` | fs_disk.c | left out | not translated yet |
| `char *path; struct stat st; path = compose_path(f->path, name);` | fs_disk.c | exported | Unvalidated |
| `if (mkdir(path, mode) < 0)` | fs_disk.c | exported | Unvalidated |
| `if (lstat(path, &st) != 0)` | fs_disk.c | exported | Unvalidated |
| `free(path); stat_to_qid(qid, &st); return 0;` | fs_disk.c | exported | Unvalidated |
| `fs_open` | fs_disk.c | left out | not translated yet |
| `struct stat st; fs_close(fs, f);` | fs_disk.c | exported | Unvalidated |
| `if (stat(f->path, &st) != 0)` | fs_disk.c | exported | Unvalidated |
| `stat_to_qid(qid, &st);` | fs_disk.c | exported | Unvalidated |
| `if (flags & 0x00010000) … else` | fs_disk.c | exported | Unvalidated |
| `return 0;` | fs_disk.c | exported | Unvalidated |
| `fs_create` | fs_disk.c | left out | not translated yet |
| `struct stat st; char *path; int ret, fd; fs_close(fs, f); path = compose_path(f->path, name); f…` | fs_disk.c | exported | Unvalidated |
| `if (fd < 0)` | fs_disk.c | exported | Unvalidated |
| `ret = lstat(path, &st);` | fs_disk.c | exported | Unvalidated |
| `if (ret != 0)` | fs_disk.c | exported | Unvalidated |
| `free(f->path); f->path = path; f->is_opened = TRUE; f->is_dir = FALSE; f->u.fd = fd; stat_to_qi…` | fs_disk.c | exported | Unvalidated |
| `fs_readdir` | fs_disk.c | left out | not translated yet |
| `if (!f->is_opened || !f->is_dir)` | fs_disk.c | exported | Unvalidated |
| `if (offset == 0) … else` | fs_disk.c | exported | Unvalidated |
| `pos = 0;` | fs_disk.c | exported | Unvalidated |
| `for (; ; )` | fs_disk.c | exported | Unvalidated |
| `return pos;` | fs_disk.c | exported | Unvalidated |
| `fs_read` | fs_disk.c | left out | not translated yet |
| `if (!f->is_opened || f->is_dir)` | fs_disk.c | exported | Unvalidated |
| `ret = pread(f->u.fd, buf, count, offset);` | fs_disk.c | exported | Unvalidated |
| `if (ret < 0) … else` | fs_disk.c | exported | Unvalidated |
| `fs_write` | fs_disk.c | left out | not translated yet |
| `if (!f->is_opened || f->is_dir)` | fs_disk.c | exported | Unvalidated |
| `ret = pwrite(f->u.fd, buf, count, offset);` | fs_disk.c | exported | Unvalidated |
| `if (ret < 0) … else` | fs_disk.c | exported | Unvalidated |
| `fs_close` | fs_disk.c | left out | not translated yet |
| `if (!f->is_opened)` | fs_disk.c | exported | Unvalidated |
| `if (f->is_dir) … else` | fs_disk.c | exported | Unvalidated |
| `f->is_opened = FALSE;` | fs_disk.c | exported | Unvalidated |
| `fs_stat` | fs_disk.c | left out | not translated yet |
| `if (lstat(f->path, &st1) != 0)` | fs_disk.c | exported | Unvalidated |
| `stat_to_qid(&st->qid, &st1); st->st_mode = st1.st_mode; st->st_uid = st1.st_uid; st->st_gid = s…` | fs_disk.c | exported | Unvalidated |
| `fs_setattr` | fs_disk.c | left out | not translated yet |
| `if (mask & (0x00000002 | 0x00000004)` | fs_disk.c | exported | Unvalidated |
| `if (mask & 0x00000001)` | fs_disk.c | exported | Unvalidated |
| `if (mask & 0x00000008)` | fs_disk.c | exported | Unvalidated |
| `if (mask & (0x00000010 | 0x00000020)` | fs_disk.c | exported | Unvalidated |
| `if (mask & 0x00000040) && !ctime_updated)` | fs_disk.c | exported | Unvalidated |
| `return 0;` | fs_disk.c | exported | Unvalidated |
| `fs_link` | fs_disk.c | left out | not translated yet |
| `char *path; path = compose_path(df->path, name);` | fs_disk.c | exported | Unvalidated |
| `if (link(f->path, path) < 0)` | fs_disk.c | exported | Unvalidated |
| `free(path); return 0;` | fs_disk.c | exported | Unvalidated |
| `fs_symlink` | fs_disk.c | left out | not translated yet |
| `char *path; struct stat st; path = compose_path(f->path, name);` | fs_disk.c | exported | Unvalidated |
| `if (symlink(symgt, path) < 0)` | fs_disk.c | exported | Unvalidated |
| `if (lstat(path, &st) != 0)` | fs_disk.c | exported | Unvalidated |
| `free(path); stat_to_qid(qid, &st); return 0;` | fs_disk.c | exported | Unvalidated |
| `fs_mknod` | fs_disk.c | left out | not translated yet |
| `char *path; struct stat st; path = compose_path(f->path, name);` | fs_disk.c | exported | Unvalidated |
| `if (mknod(path, mode, gnu_dev_makedev ( major , minor ) ) < 0)` | fs_disk.c | exported | Unvalidated |
| `if (lstat(path, &st) != 0)` | fs_disk.c | exported | Unvalidated |
| `free(path); stat_to_qid(qid, &st); return 0;` | fs_disk.c | exported | Unvalidated |
| `fs_readlink` | fs_disk.c | left out | not translated yet |
| `int ret; ret = readlink(f->path, buf, buf_size - 1);` | fs_disk.c | exported | Unvalidated |
| `if (ret < 0)` | fs_disk.c | exported | Unvalidated |
| `buf[ret] = '\0'; return 0;` | fs_disk.c | exported | Unvalidated |
| `fs_renameat` | fs_disk.c | left out | not translated yet |
| `char *path, *new_path; int ret; path = compose_path(f->path, name); new_path = compose_path(new…` | fs_disk.c | exported | Unvalidated |
| `if (ret < 0)` | fs_disk.c | exported | Unvalidated |
| `return 0;` | fs_disk.c | exported | Unvalidated |
| `fs_unlinkat` | fs_disk.c | left out | not translated yet |
| `char *path; int ret; path = compose_path(f->path, name); ret = remove(path); free(path);` | fs_disk.c | exported | Unvalidated |
| `if (ret < 0)` | fs_disk.c | exported | Unvalidated |
| `return 0;` | fs_disk.c | exported | Unvalidated |
| `fs_lock` | fs_disk.c | left out | not translated yet |
| `if (!f->is_opened || f->is_dir)` | fs_disk.c | exported | Unvalidated |
| `fl.l_type = lock->type; fl.l_whence = 0 ; fl.l_start = lock->start; fl.l_len = lock->length; re…` | fs_disk.c | exported | Unvalidated |
| `if (ret == 0) … else` | fs_disk.c | exported | Unvalidated |
| `return ret;` | fs_disk.c | exported | Unvalidated |
| `fs_getlock` | fs_disk.c | left out | not translated yet |
| `if (!f->is_opened || f->is_dir)` | fs_disk.c | exported | Unvalidated |
| `fl.l_type = lock->type; fl.l_whence = 0 ; fl.l_start = lock->start; fl.l_len = lock->length; re…` | fs_disk.c | exported | Unvalidated |
| `if (ret < 0) … else` | fs_disk.c | exported | Unvalidated |
| `return ret;` | fs_disk.c | exported | Unvalidated |
| `fs_disk_end` | fs_disk.c | left out | not translated yet |
| `FSDeviceDisk *fs = (FSDeviceDisk *)fs1; free(fs->root_path);` | fs_disk.c | exported | Unvalidated |
| `fs_disk_init` | fs_disk.c | left out | not translated yet |
| `FSDeviceDisk *fs; struct stat st; lstat(root_path, &st);` | fs_disk.c | exported | Unvalidated |
| `if (! (((( st.st_mode )) & 0170000) == (0040000)` | fs_disk.c | exported | Unvalidated |
| `fs = mallocz(sizeof(*fs)); fs->common.fs_end = fs_disk_end; fs->common.fs_delete = fs_delete; f…` | fs_disk.c | exported | Unvalidated |
| `file_buffer_init` | fs_net.c | left out | not translated yet |
| `bs->data = ((void *)0) ; bs->allocated_size = 0;` | fs_net.c | exported | Unvalidated |
| `file_buffer_reset` | fs_net.c | left out | not translated yet |
| `free(bs->data); file_buffer_init(bs);` | fs_net.c | exported | Unvalidated |
| `file_buffer_resize` | fs_net.c | left out | not translated yet |
| `uint8_t *new_data; new_data = realloc(bs->data, new_size);` | fs_net.c | exported | Unvalidated |
| `if (!new_data && new_size != 0)` | fs_net.c | exported | Unvalidated |
| `bs->data = new_data; bs->allocated_size = new_size; return 0;` | fs_net.c | exported | Unvalidated |
| `file_buffer_write` | fs_net.c | left out | not translated yet |
| `memcpy(bs->data + offset, buf, size);` | fs_net.c | exported | Unvalidated |
| `file_buffer_set` | fs_net.c | left out | not translated yet |
| `memset(bs->data + offset, val, size);` | fs_net.c | exported | Unvalidated |
| `file_buffer_read` | fs_net.c | left out | not translated yet |
| `memcpy(buf, bs->data + offset, size);` | fs_net.c | exported | Unvalidated |
| `to_blocks` | fs_net.c | left out | not translated yet |
| `return (size + fs->block_size - 1) >> fs->block_size_log2;` | fs_net.c | exported | Unvalidated |
| `inode_incref` | fs_net.c | left out | not translated yet |
| `n->refcount++; return n;` | fs_net.c | exported | Unvalidated |
| `inode_inc_open` | fs_net.c | left out | not translated yet |
| `n->open_count++; return n;` | fs_net.c | exported | Unvalidated |
| `inode_free` | fs_net.c | left out | not translated yet |
| `FSDeviceMem *fs = (FSDeviceMem *)fs1; ((void) sizeof (__assert_single_arg ( n->refcount == 0 ))…` | fs_net.c | exported | Unvalidated |
| `switch (n->type)` | fs_net.c | exported | Unvalidated |
| `list_del(&n->link); free(n); fs->inode_count--; ((void) sizeof (__assert_single_arg ( fs->inode…` | fs_net.c | exported | Unvalidated |
| `inode_decref` | fs_net.c | left out | not translated yet |
| `((void) sizeof (__assert_single_arg ( n->refcount >= 1 )), __extension__ ({ if ( n->refcount >=…` | fs_net.c | exported | Unvalidated |
| `if (--n->refcount <= 0 && n->open_count <= 0)` | fs_net.c | exported | Unvalidated |
| `inode_dec_open` | fs_net.c | left out | not translated yet |
| `((void) sizeof (__assert_single_arg ( n->open_count >= 1 )), __extension__ ({ if ( n->open_coun…` | fs_net.c | exported | Unvalidated |
| `if (--n->open_count <= 0 && n->refcount <= 0)` | fs_net.c | exported | Unvalidated |
| `inode_update_mtime` | fs_net.c | left out | not translated yet |
| `struct timeval tv; gettimeofday(&tv, ((void *)0) ); n->mtime_sec = tv.tv_sec; n->mtime_nsec = t…` | fs_net.c | exported | Unvalidated |
| `inode_new` | fs_net.c | left out | not translated yet |
| `FSDeviceMem *fs = (FSDeviceMem *)fs1; FSINode *n; n = mallocz(sizeof(*n)); n->refcount = 1; n->…` | fs_net.c | exported | Unvalidated |
| `switch (type)` | fs_net.c | exported | Unvalidated |
| `list_add(&n->link, &fs->inode_list); fs->inode_count++; inode_update_mtime(fs1, n); n->ctime_se…` | fs_net.c | exported | Unvalidated |
| `inode_dir_add` | fs_net.c | left out | not translated yet |
| `FSDeviceMem *fs = (FSDeviceMem *)fs1; FSDirEntry *de; int name_len, dirent_size, new_size; ((vo…` | fs_net.c | exported | Unvalidated |
| `inode_search` | fs_net.c | left out | not translated yet |
| `if (n->type != FT_DIR)` | fs_net.c | exported | Unvalidated |
| `for (el = (&n->u.dir.de_list)->next; el != (&n->u.dir.de_list; el = el->next)` | fs_net.c | exported | Unvalidated |
| `return ((void *)0) ;` | fs_net.c | exported | Unvalidated |
| `inode_search_path1` | fs_net.c | left out | not translated yet |
| `char name[1024]; const char *p, *p1; int len; FSDirEntry *de; p = path;` | fs_net.c | exported | Unvalidated |
| `if (*p == '/')` | fs_net.c | exported | Unvalidated |
| `if (*p == '\0')` | fs_net.c | exported | Unvalidated |
| `for (; ; )` | fs_net.c | exported | Unvalidated |
| `return n;` | fs_net.c | exported | Unvalidated |
| `inode_search_path` | fs_net.c | left out | not translated yet |
| `if (!fs1)` | fs_net.c | exported | Unvalidated |
| `return inode_search_path1(fs1, fs->root_inode, path);` | fs_net.c | exported | Unvalidated |
| `is_empty_dir` | fs_net.c | left out | not translated yet |
| `for (el = (&n->u.dir.de_list)->next; el != (&n->u.dir.de_list; el = el->next)` | fs_net.c | exported | Unvalidated |
| `return TRUE;` | fs_net.c | exported | Unvalidated |
| `inode_dirent_delete_no_decref` | fs_net.c | left out | not translated yet |
| `FSDeviceMem *fs = (FSDeviceMem *)fs1; int dirent_size, new_size; dirent_size = sizeof(*de) + st…` | fs_net.c | exported | Unvalidated |
| `inode_dirent_delete` | fs_net.c | left out | not translated yet |
| `FSINode *n1; n1 = de->inode; inode_dirent_delete_no_decref(fs, n, de); inode_decref(fs, n1);` | fs_net.c | exported | Unvalidated |
| `flush_dir` | fs_net.c | left out | not translated yet |
| `for (el = (&n->u.dir.de_list)->next, el1 = el->next; el != (&n->u.dir.de_list; el = el1, el1 =…` | fs_net.c | exported | Unvalidated |
| `((void) sizeof (__assert_single_arg ( n->u.dir.size == 0 )), __extension__ ({ if ( n->u.dir.siz…` | fs_net.c | exported | Unvalidated |
| `fs_delete` | fs_net.c | left out | not translated yet |
| `fs_close(fs, f); inode_dec_open(fs, f->inode); free(f);` | fs_net.c | exported | Unvalidated |
| `fid_create` | fs_net.c | left out | not translated yet |
| `FSFile *f; f = mallocz(sizeof(*f)); f->inode = inode_inc_open(fs1, n); f->uid = uid; return f;` | fs_net.c | exported | Unvalidated |
| `inode_to_qid` | fs_net.c | left out | not translated yet |
| `if (n->type == FT_DIR) … else` | fs_net.c | exported | Unvalidated |
| `qid->version = 0; qid->path = n->inode_num;` | fs_net.c | exported | Unvalidated |
| `fs_statfs` | fs_net.c | left out | not translated yet |
| `FSDeviceMem *fs = (FSDeviceMem *)fs1; st->f_bsize = 1024; st->f_blocks = fs->fs_max_blocks << (…` | fs_net.c | exported | Unvalidated |
| `fs_attach` | fs_net.c | left out | not translated yet |
| `FSDeviceMem *fs = (FSDeviceMem *)fs1; *pf = fid_create(fs1, fs->root_inode, uid); inode_to_qid(…` | fs_net.c | exported | Unvalidated |
| `fs_walk` | fs_net.c | left out | not translated yet |
| `int i; FSINode *n; FSDirEntry *de; n = f->inode;` | fs_net.c | exported | Unvalidated |
| `for (i = 0; i < count; i++)` | fs_net.c | exported | Unvalidated |
| `*pf = fid_create(fs, n, f->uid); return i;` | fs_net.c | exported | Unvalidated |
| `fs_mkdir` | fs_net.c | left out | not translated yet |
| `FSINode *n, *n1; n = f->inode;` | fs_net.c | exported | Unvalidated |
| `if (n->type != FT_DIR)` | fs_net.c | exported | Unvalidated |
| `if (inode_search(n, name))` | fs_net.c | exported | Unvalidated |
| `n1 = inode_new(fs, FT_DIR, mode, f->uid, gid); inode_dir_add(fs, n1, ".", inode_incref(fs, n1))…` | fs_net.c | exported | Unvalidated |
| `fs_trim_cache` | fs_net.c | left out | not translated yet |
| `if (fs->inode_cache_size + added_size) <= fs->inode_cache_size_limit)` | fs_net.c | exported | Unvalidated |
| `for (el = (&fs->inode_cache_list)->prev, el1 = el->prev; el != (&fs->inode_cache_list; el = el1…` | fs_net.c | exported | Unvalidated |
| `fs_open_end` | fs_net.c | left out | not translated yet |
| `if (oi->open_type == FS_OPEN_WGET_ARCHIVE_FILE)` | fs_net.c | exported | Unvalidated |
| `if (oi->dec_state)` | fs_net.c | exported | Unvalidated |
| `free(oi);` | fs_net.c | exported | Unvalidated |
| `fs_open_write_cb` | fs_net.c | left out | not translated yet |
| `FSOpenInfo *oi = opaque; size_t len; FSINode *n = oi->n; len = n->u.reg.size - oi->cur_pos;` | fs_net.c | exported | Unvalidated |
| `if (size < len)` | fs_net.c | exported | Unvalidated |
| `file_buffer_write(&n->u.reg.fbuf, oi->cur_pos, data, len); oi->cur_pos += len; return 0;` | fs_net.c | exported | Unvalidated |
| `fs_wget_set_loaded` | fs_net.c | left out | not translated yet |
| `FSOpenInfo *oi; FSDeviceMem *fs; FSFile *f; FSQID qid; ((void) sizeof (__assert_single_arg ( n-…` | fs_net.c | exported | Unvalidated |
| `if (oi->cb)` | fs_net.c | exported | Unvalidated |
| `fs_open_end(oi);` | fs_net.c | exported | Unvalidated |
| `fs_wget_set_error` | fs_net.c | left out | not translated yet |
| `FSOpenInfo *oi; ((void) sizeof (__assert_single_arg ( n->u.reg.state == REG_STATE_LOADING )), _…` | fs_net.c | exported | Unvalidated |
| `if (oi->cb)` | fs_net.c | exported | Unvalidated |
| `fs_open_end(oi);` | fs_net.c | exported | Unvalidated |
| `fs_read_archive` | fs_net.c | left out | not translated yet |
| `for (el = (&oi->archive_file_list)->next, el1 = el->next; el != (&oi->archive_file_list; el = e…` | fs_net.c | exported | Unvalidated |
| `fs_error_archive` | fs_net.c | left out | not translated yet |
| `for (el = (&oi->archive_file_list)->next, el1 = el->next; el != (&oi->archive_file_list; el = e…` | fs_net.c | exported | Unvalidated |
| `fs_open_cb` | fs_net.c | left out | not translated yet |
| `if (err < 0) … else` | fs_net.c | exported | Unvalidated |
| `fs_open_wget` | fs_net.c | left out | not translated yet |
| `char *url; FSOpenInfo *oi; char fname[32]; FSBaseURL *bu; ((void) sizeof (__assert_single_arg (…` | fs_net.c | exported | Unvalidated |
| `if (file_buffer_resize(&n->u.reg.fbuf, n->u.reg.size) < 0)` | fs_net.c | exported | Unvalidated |
| `n->u.reg.state = REG_STATE_LOADING; oi = mallocz(sizeof(*oi)); oi->cur_pos = 0; oi->fs = fs1; o…` | fs_net.c | exported | Unvalidated |
| `if (open_type != FS_OPEN_WGET_ARCHIVE_FILE)` | fs_net.c | exported | Unvalidated |
| `n->u.reg.open_info = oi; return 0;` | fs_net.c | exported | Unvalidated |
| `fs_preload_file` | fs_net.c | left out | not translated yet |
| `FSINode *n; n = inode_search_path(fs1, filename);` | fs_net.c | exported | Unvalidated |
| `if (n && n->type == FT_REG && n->u.reg.state == REG_STATE_UNLOADED)` | fs_net.c | exported | Unvalidated |
| `find_preload_archive` | fs_net.c | left out | not translated yet |
| `for (el = (&fs->preload_archive_list)->next; el != (&fs->preload_archive_list; el = el->next)` | fs_net.c | exported | Unvalidated |
| `return ((void *)0) ;` | fs_net.c | exported | Unvalidated |
| `fs_preload_archive` | fs_net.c | left out | not translated yet |
| `FSDeviceMem *fs = (FSDeviceMem *)fs1; PreloadArchive *pa; PreloadArchiveFile *paf; struct list_…` | fs_net.c | exported | Unvalidated |
| `if (!pa)` | fs_net.c | exported | Unvalidated |
| `n = inode_search_path(fs1, filename);` | fs_net.c | exported | Unvalidated |
| `if (n && n->type == FT_REG && n->u.reg.state == REG_STATE_UNLOADED) … else` | fs_net.c | exported | Unvalidated |
| `fs_preload_files` | fs_net.c | left out | not translated yet |
| `for (el = (&fs->preload_list)->next; el != (&fs->preload_list; el = el->next)` | fs_net.c | exported | Unvalidated |
| `return;` | fs_net.c | exported | Unvalidated |
| `found: for(el = (&pe->file_list)->next; el != (&pe->file_list); el = el->next) { pf = ((Preload…` | fs_net.c | exported | Unvalidated |
| `fs_open` | fs_net.c | left out | not translated yet |
| `FSINode *n = f->inode; FSDeviceMem *fs = (FSDeviceMem *)fs1; int ret; fs_close(fs1, f);` | fs_net.c | exported | Unvalidated |
| `if (flags & 0x00010000) … else` | fs_net.c | exported | Unvalidated |
| `f->open_flags = flags;` | fs_net.c | exported | Unvalidated |
| `if (n->type == FT_REG) … else` | fs_net.c | exported | Unvalidated |
| `fs_create` | fs_net.c | left out | not translated yet |
| `if (n->type != FT_DIR)` | fs_net.c | exported | Unvalidated |
| `if (inode_search(n, name)) … else` | fs_net.c | exported | Unvalidated |
| `fs_readdir` | fs_net.c | left out | not translated yet |
| `if (!f->is_opened || n->type != FT_DIR)` | fs_net.c | exported | Unvalidated |
| `el = n->u.dir.de_list.next; offset = 0;` | fs_net.c | exported | Unvalidated |
| `while (offset < offset1)` | fs_net.c | exported | Unvalidated |
| `pos = 0;` | fs_net.c | exported | Unvalidated |
| `for (; ; )` | fs_net.c | exported | Unvalidated |
| `return pos;` | fs_net.c | exported | Unvalidated |
| `fs_read` | fs_net.c | left out | not translated yet |
| `if (!f->is_opened)` | fs_net.c | exported | Unvalidated |
| `if (n->type != FT_REG)` | fs_net.c | exported | Unvalidated |
| `if (f->open_flags & 0x00000003) == 0x00000001)` | fs_net.c | exported | Unvalidated |
| `if (n->u.reg.is_fscmd)` | fs_net.c | exported | Unvalidated |
| `if (offset >= n->u.reg.size)` | fs_net.c | exported | Unvalidated |
| `count1 = n->u.reg.size - offset;` | fs_net.c | exported | Unvalidated |
| `if (count1 < count)` | fs_net.c | exported | Unvalidated |
| `file_buffer_read(&n->u.reg.fbuf, offset, buf, count); return count;` | fs_net.c | exported | Unvalidated |
| `fs_truncate` | fs_net.c | left out | not translated yet |
| `if (n->type != FT_REG)` | fs_net.c | exported | Unvalidated |
| `if (size > (18446744073709551615UL)` | fs_net.c | exported | Unvalidated |
| `diff = size - n->u.reg.size;` | fs_net.c | exported | Unvalidated |
| `if (diff == 0)` | fs_net.c | exported | Unvalidated |
| `diff_blocks = to_blocks(fs, size) - to_blocks(fs, n->u.reg.size);` | fs_net.c | exported | Unvalidated |
| `switch (n->u.reg.state)` | fs_net.c | exported | Unvalidated |
| `fs->fs_blocks += diff_blocks; ((void) sizeof (__assert_single_arg ( fs->fs_blocks >= 0 )), __ex…` | fs_net.c | exported | Unvalidated |
| `fs_write` | fs_net.c | left out | not translated yet |
| `if (!f->is_opened)` | fs_net.c | exported | Unvalidated |
| `if (n->type != FT_REG)` | fs_net.c | exported | Unvalidated |
| `if (f->open_flags & 0x00000003) == 0x00000000)` | fs_net.c | exported | Unvalidated |
| `if (count == 0)` | fs_net.c | exported | Unvalidated |
| `if (n->u.reg.is_fscmd)` | fs_net.c | exported | Unvalidated |
| `end = offset + count;` | fs_net.c | exported | Unvalidated |
| `if (end > n->u.reg.size)` | fs_net.c | exported | Unvalidated |
| `inode_update_mtime(fs1, n);` | fs_net.c | exported | Unvalidated |
| `if (n->u.reg.state == REG_STATE_LOADED)` | fs_net.c | exported | Unvalidated |
| `file_buffer_write(&n->u.reg.fbuf, offset, buf, count); return count;` | fs_net.c | exported | Unvalidated |
| `fs_close` | fs_net.c | left out | not translated yet |
| `if (f->is_opened)` | fs_net.c | exported | Unvalidated |
| `if (f->req)` | fs_net.c | exported | Unvalidated |
| `fs_stat` | fs_net.c | left out | not translated yet |
| `FSDeviceMem *fs = (FSDeviceMem *)fs1; FSINode *n = f->inode; inode_to_qid(&st->qid, n); st->st_…` | fs_net.c | exported | Unvalidated |
| `if (n->type == FT_BLK || n->type == FT_CHR) … else` | fs_net.c | exported | Unvalidated |
| `st->st_blksize = fs->block_size;` | fs_net.c | exported | Unvalidated |
| `if (n->type == FT_REG) … else` | fs_net.c | exported | Unvalidated |
| `st->st_blocks = to_blocks(fs, st->st_size) << (fs->block_size_log2 - 9); st->st_atime_sec = n->…` | fs_net.c | exported | Unvalidated |
| `fs_setattr` | fs_net.c | left out | not translated yet |
| `if (mask & 0x00000001)` | fs_net.c | exported | Unvalidated |
| `if (mask & 0x00000002)` | fs_net.c | exported | Unvalidated |
| `if (mask & 0x00000004)` | fs_net.c | exported | Unvalidated |
| `if (mask & 0x00000008)` | fs_net.c | exported | Unvalidated |
| `if (mask & 0x00000020)` | fs_net.c | exported | Unvalidated |
| `if (mask & 0x00000040)` | fs_net.c | exported | Unvalidated |
| `return 0;` | fs_net.c | exported | Unvalidated |
| `fs_link` | fs_net.c | left out | not translated yet |
| `if (f->inode->type == FT_DIR)` | fs_net.c | exported | Unvalidated |
| `if (inode_search(n, name))` | fs_net.c | exported | Unvalidated |
| `inode_dir_add(fs, n, name, inode_incref(fs, f->inode)); return 0;` | fs_net.c | exported | Unvalidated |
| `fs_symlink` | fs_net.c | left out | not translated yet |
| `if (inode_search(n, name))` | fs_net.c | exported | Unvalidated |
| `n1 = inode_new(fs, FT_LNK, 0777, f->uid, gid); n1->u.symlink.name = strdup(symgt); inode_dir_ad…` | fs_net.c | exported | Unvalidated |
| `fs_mknod` | fs_net.c | left out | not translated yet |
| `int type; FSINode *n1, *n = f->inode; type = (mode & 0xF000) >> 12;` | fs_net.c | exported | Unvalidated |
| `if (type != FT_FIFO && type != FT_CHR && type != FT_BLK && type != FT_REG && type != FT_SOCK)` | fs_net.c | exported | Unvalidated |
| `if (inode_search(n, name))` | fs_net.c | exported | Unvalidated |
| `n1 = inode_new(fs, type, mode, f->uid, gid);` | fs_net.c | exported | Unvalidated |
| `if (type == FT_CHR || type == FT_BLK)` | fs_net.c | exported | Unvalidated |
| `inode_dir_add(fs, n, name, n1); inode_to_qid(qid, n1); return 0;` | fs_net.c | exported | Unvalidated |
| `fs_readlink` | fs_net.c | left out | not translated yet |
| `if (n->type != FT_LNK)` | fs_net.c | exported | Unvalidated |
| `len = min_int(strlen(n->u.symlink.name), buf_size - 1); memcpy(buf, n->u.symlink.name, len); bu…` | fs_net.c | exported | Unvalidated |
| `fs_renameat` | fs_net.c | left out | not translated yet |
| `FSDirEntry *de, *de1; FSINode *n1; de = inode_search(f->inode, name);` | fs_net.c | exported | Unvalidated |
| `if (!de)` | fs_net.c | exported | Unvalidated |
| `de1 = inode_search(new_f->inode, new_name); n1 = ((void *)0) ;` | fs_net.c | exported | Unvalidated |
| `if (de1)` | fs_net.c | exported | Unvalidated |
| `inode_dir_add(fs, new_f->inode, new_name, inode_incref(fs, de->inode)); inode_dirent_delete(fs,…` | fs_net.c | exported | Unvalidated |
| `if (n1)` | fs_net.c | exported | Unvalidated |
| `return 0;` | fs_net.c | exported | Unvalidated |
| `fs_unlinkat` | fs_net.c | left out | not translated yet |
| `if (!strcmp(name, ".") || !strcmp(name, ".."))` | fs_net.c | exported | Unvalidated |
| `de = inode_search(f->inode, name);` | fs_net.c | exported | Unvalidated |
| `if (!de)` | fs_net.c | exported | Unvalidated |
| `n = de->inode;` | fs_net.c | exported | Unvalidated |
| `if (n->type == FT_DIR)` | fs_net.c | exported | Unvalidated |
| `inode_dirent_delete(fs, f->inode, de); return 0;` | fs_net.c | exported | Unvalidated |
| `fs_lock` | fs_net.c | left out | not translated yet |
| `if (!f->is_opened)` | fs_net.c | exported | Unvalidated |
| `if (n->type != FT_REG)` | fs_net.c | exported | Unvalidated |
| `return 0;` | fs_net.c | exported | Unvalidated |
| `fs_getlock` | fs_net.c | left out | not translated yet |
| `if (!f->is_opened)` | fs_net.c | exported | Unvalidated |
| `if (n->type != FT_REG)` | fs_net.c | exported | Unvalidated |
| `return 0;` | fs_net.c | exported | Unvalidated |
| `fs_mem_end` | fs_net.c | left out | not translated yet |
| `for (el = (&fs->inode_list)->next, el1 = el->next; el != (&fs->inode_list; el = el1, el1 = el->…` | fs_net.c | exported | Unvalidated |
| `((void) sizeof (__assert_single_arg ( list_empty(&fs->inode_cache_list) )), __extension__ ({ if…` | fs_net.c | exported | Unvalidated |
| `fs_mem_init` | fs_net.c | left out | not translated yet |
| `FSDeviceMem *fs; FSDevice *fs1; FSINode *n; fs = mallocz(sizeof(*fs)); fs1 = &fs->common; fs->c…` | fs_net.c | exported | Unvalidated |
| `fs_is_net` | fs_net.c | left out | not translated yet |
| `return (fs->fs_end == fs_mem_end);` | fs_net.c | exported | Unvalidated |
| `fs_find_base_url` | fs_net.c | left out | not translated yet |
| `for (el = (&fs->base_url_list)->next; el != (&fs->base_url_list; el = el->next)` | fs_net.c | exported | Unvalidated |
| `return ((void *)0) ;` | fs_net.c | exported | Unvalidated |
| `fs_base_url_decref` | fs_net.c | left out | not translated yet |
| `((void) sizeof (__assert_single_arg ( bu->ref_count >= 1 )), __extension__ ({ if ( bu->ref_coun…` | fs_net.c | exported | Unvalidated |
| `if (--bu->ref_count == 0)` | fs_net.c | exported | Unvalidated |
| `fs_net_set_base_url` | fs_net.c | left out | not translated yet |
| `FSDeviceMem *fs = (FSDeviceMem *)fs1; FSBaseURL *bu; ((void) sizeof (__assert_single_arg ( fs_i…` | fs_net.c | exported | Unvalidated |
| `if (!bu) … else` | fs_net.c | exported | Unvalidated |
| `bu->url = strdup(url);` | fs_net.c | exported | Unvalidated |
| `if (user) … else` | fs_net.c | exported | Unvalidated |
| `if (password) … else` | fs_net.c | exported | Unvalidated |
| `if (aes_state) … else` | fs_net.c | exported | Unvalidated |
| `return bu;` | fs_net.c | exported | Unvalidated |
| `fs_net_reset_base_url` | fs_net.c | left out | not translated yet |
| `FSBaseURL *bu; ((void) sizeof (__assert_single_arg ( fs_is_net(fs1) )), __extension__ ({ if ( f…` | fs_net.c | exported | Unvalidated |
| `if (!bu)` | fs_net.c | exported | Unvalidated |
| `fs_base_url_decref(fs1, bu); return 0;` | fs_net.c | exported | Unvalidated |
| `fs_net_set_fs_max_size` | fs_net.c | left out | not translated yet |
| `FSDeviceMem *fs = (FSDeviceMem *)fs1; ((void) sizeof (__assert_single_arg ( fs_is_net(fs1) )),…` | fs_net.c | exported | Unvalidated |
| `fs_net_set_url` | fs_net.c | left out | not translated yet |
| `FSDeviceMem *fs = (FSDeviceMem *)fs1; FSBaseURL *bu; ((void) sizeof (__assert_single_arg ( fs_i…` | fs_net.c | exported | Unvalidated |
| `if (!bu)` | fs_net.c | exported | Unvalidated |
| `if (n->type != FT_REG || n->u.reg.state != REG_STATE_LOCAL || n->u.reg.fbuf.allocated_size != 0)` | fs_net.c | exported | Unvalidated |
| `if (size > 0)` | fs_net.c | exported | Unvalidated |
| `return 0;` | fs_net.c | exported | Unvalidated |
| `fs_dump_add_file` | fs_net.c | left out | not translated yet |
| `PreloadFile *pf; pf = mallocz(sizeof(*pf)); pf->name = strdup(name); list_add_tail(&pf->link, h…` | fs_net.c | exported | Unvalidated |
| `fs_dump_find_file` | fs_net.c | left out | not translated yet |
| `for (el = (head)->next; el != (head; el = el->next)` | fs_net.c | exported | Unvalidated |
| `return ((void *)0) ;` | fs_net.c | exported | Unvalidated |
| `dump_close_archive` | fs_net.c | left out | not translated yet |
| `if (fs->dump_archive_file)` | fs_net.c | exported | Unvalidated |
| `fs->dump_archive_file = ((void *)0) ; fs->dump_archive_size = 0;` | fs_net.c | exported | Unvalidated |
| `dump_loaded_file` | fs_net.c | left out | not translated yet |
| `if (!fs->dump_cache_load || !n->u.reg.filename)` | fs_net.c | exported | Unvalidated |
| `fname = n->u.reg.filename;` | fs_net.c | exported | Unvalidated |
| `if (fs_dump_find_file(&fs->dump_preload_list, fname))` | fs_net.c | exported | Unvalidated |
| `if (!fs->dump_started)` | fs_net.c | exported | Unvalidated |
| `if (!fs->dump_archive_file)` | fs_net.c | exported | Unvalidated |
| `if (n->u.reg.size >= (4 << 20) … else` | fs_net.c | exported | Unvalidated |
| `json_load` | fs_net.c | left out | not translated yet |
| `FILE *f; JSONValue val; size_t size; char *buf; f = fopen(filename, "rb");` | fs_net.c | exported | Unvalidated |
| `if (!f)` | fs_net.c | exported | Unvalidated |
| `fseek(f, 0, 2 ); size = ftell(f); fseek(f, 0, 0 ); buf = malloc(size + 1); fread(buf, 1, size,…` | fs_net.c | exported | Unvalidated |
| `fs_dump_cache_load` | fs_net.c | left out | not translated yet |
| `if (!fs_is_net(fs1))` | fs_net.c | exported | Unvalidated |
| `cfg = json_load(cfg_filename);` | fs_net.c | exported | Unvalidated |
| `if (json_is_error(cfg))` | fs_net.c | exported | Unvalidated |
| `val = json_object_get(cfg, "preload_dir");` | fs_net.c | exported | Unvalidated |
| `if (json_is_undefined(cfg))` | fs_net.c | exported | Unvalidated |
| `preload_dir = json_get_str(val);` | fs_net.c | exported | Unvalidated |
| `if (!preload_dir)` | fs_net.c | exported | Unvalidated |
| `fs->dump_preload_dir = strdup(preload_dir); init_list_head(&fs->dump_preload_list); init_list_h…` | fs_net.c | exported | Unvalidated |
| `if (array.type != JSON_ARRAY)` | fs_net.c | exported | Unvalidated |
| `for (i = 0; i < array.u.array->len; i++)` | fs_net.c | exported | Unvalidated |
| `json_free(cfg); fname = compose_path(fs->dump_preload_dir, "preload.txt"); fs->dump_preload_fil…` | fs_net.c | exported | Unvalidated |
| `if (!fs->dump_preload_file)` | fs_net.c | exported | Unvalidated |
| `free(fname); fname = compose_path(fs->dump_preload_dir, "preload_archive.txt"); fs->dump_preloa…` | fs_net.c | exported | Unvalidated |
| `if (!fs->dump_preload_archive_file)` | fs_net.c | exported | Unvalidated |
| `free(fname); fs->dump_cache_load = TRUE;` | fs_net.c | exported | Unvalidated |
| `filelist_load_rec` | fs_net.c | left out | not translated yet |
| `char fname[1024], lname[1024]; int ret; const char *p; FSINodeTypeEnum type; uint32_t mode, uid…` | fs_net.c | exported | Unvalidated |
| `for (; ; )` | fs_net.c | exported | Unvalidated |
| `*pp = p; return 0;` | fs_net.c | exported | Unvalidated |
| `filelist_load` | fs_net.c | left out | not translated yet |
| `if (parse_tag_version(str) != 1)` | fs_net.c | exported | Unvalidated |
| `p = skip_header(str);` | fs_net.c | exported | Unvalidated |
| `if (!p)` | fs_net.c | exported | Unvalidated |
| `ret = filelist_load_rec(fs1, &p, fs->root_inode, ""); return ret;` | fs_net.c | exported | Unvalidated |
| `fs_create_cmd` | fs_net.c | left out | not translated yet |
| `FSFile *root_fd; FSQID qid; FSINode *n; ((void) sizeof (__assert_single_arg ( !fs->fs_attach(fs…` | fs_net.c | exported | Unvalidated |
| `fs_net_init` | fs_net.c | left out | not translated yet |
| `FSDevice *fs; FSDeviceMem *fs1; fs_wget_init(); fs = fs_mem_init(); fs1 = (FSDeviceMem *)fs; fs…` | fs_net.c | exported | Unvalidated |
| `if (url)` | fs_net.c | exported | Unvalidated |
| `return fs;` | fs_net.c | exported | Unvalidated |
| `fs_initial_sync` | fs_net.c | left out | not translated yet |
| `FSNetInitState *s; FSFile *head_fd; FSQID qid; char *head_url; char buf[128]; struct timeval tv…` | fs_net.c | exported | Unvalidated |
| `head_loaded` | fs_net.c | left out | not translated yet |
| `if (size < 0)` | fs_net.c | exported | Unvalidated |
| `buf = malloc(size + 1); fs->fs_read(fs, f, 0, (uint8_t *)buf, size); buf[size] = '\0'; fs->fs_d…` | fs_net.c | exported | Unvalidated |
| `if (parse_tag_version(buf) != 1)` | fs_net.c | exported | Unvalidated |
| `if (parse_tag_file_id(&root_id, buf, "RootID") < 0)` | fs_net.c | exported | Unvalidated |
| `if (parse_tag_uint64(&fs_max_size, buf, "FSMaxSize") == 0 && fs_max_size >= ((uint64_t)1 << 20)` | fs_net.c | exported | Unvalidated |
| `root_url = compose_url(s->url, "files"); fs_net_set_base_url(fs, "/", root_url, ((void *)0) , (…` | fs_net.c | exported | Unvalidated |
| `filelist_loaded` | fs_net.c | left out | not translated yet |
| `if (size < 0)` | fs_net.c | exported | Unvalidated |
| `buf = malloc(size + 1); fs->fs_read(fs, f, 0, buf, size); buf[size] = '\0'; fs->fs_delete(fs, f…` | fs_net.c | exported | Unvalidated |
| `if (filelist_load(fs, (char *)buf) != 0)` | fs_net.c | exported | Unvalidated |
| `s->file_index = 0; kernel_load_cb(fs, ((void *)0) , 0, s);` | fs_net.c | exported | Unvalidated |
| `kernel_load_cb` | fs_net.c | left out | not translated yet |
| `if ((FSDeviceMem *)fs)->dump_cache_load)` | fs_net.c | exported | Unvalidated |
| `if (s->fd)` | fs_net.c | exported | Unvalidated |
| `if (s->file_index >= 2) … else` | fs_net.c | exported | Unvalidated |
| `preload_parse_str_old` | fs_net.c | left out | not translated yet |
| `for (; ; )` | fs_net.c | exported | Unvalidated |
| `preload_parse_str` | fs_net.c | left out | not translated yet |
| `FSDeviceMem *fs = (FSDeviceMem *)fs1; PreloadEntry *pe; PreloadArchive *pa; FSINode *n; BOOL is…` | fs_net.c | exported | Unvalidated |
| `for (; ; )` | fs_net.c | exported | Unvalidated |
| `preload_parse` | fs_net.c | left out | not translated yet |
| `FSINode *n; char *buf; size_t size; n = inode_search_path(fs, fname);` | fs_net.c | exported | Unvalidated |
| `if (!n || n->type != FT_REG || n->u.reg.state != REG_STATE_LOADED)` | fs_net.c | exported | Unvalidated |
| `size = n->u.reg.size; buf = malloc(size + 1); file_buffer_read(&n->u.reg.fbuf, 0, (uint8_t *)bu…` | fs_net.c | exported | Unvalidated |
| `if (is_new) … else` | fs_net.c | exported | Unvalidated |
| `free(buf); return 0;` | fs_net.c | exported | Unvalidated |
| `parse_hex_buf` | fs_net.c | left out | not translated yet |
| `if (parse_fname(buf1, sizeof(buf1), pp) < 0)` | fs_net.c | exported | Unvalidated |
| `len = strlen(buf1);` | fs_net.c | exported | Unvalidated |
| `if (len & 1) != 0)` | fs_net.c | exported | Unvalidated |
| `len >>= 1;` | fs_net.c | exported | Unvalidated |
| `if (len > buf_size)` | fs_net.c | exported | Unvalidated |
| `if (decode_hex(buf, buf1, len) < 0)` | fs_net.c | exported | Unvalidated |
| `return len;` | fs_net.c | exported | Unvalidated |
| `fs_cmd_xhr` | fs_net.c | left out | not translated yet |
| `if (f->req != ((void *)0)` | fs_net.c | exported | Unvalidated |
| `if (parse_fname(url, sizeof(url), &p) < 0)` | fs_net.c | exported | Unvalidated |
| `if (parse_fname(user_buf, sizeof(user_buf), &p) < 0)` | fs_net.c | exported | Unvalidated |
| `if (parse_fname(password_buf, sizeof(password_buf), &p) < 0)` | fs_net.c | exported | Unvalidated |
| `if (parse_fname(post_filename, sizeof(post_filename), &p) < 0)` | fs_net.c | exported | Unvalidated |
| `if (parse_fname(filename, sizeof(filename), &p) < 0)` | fs_net.c | exported | Unvalidated |
| `aes_key_len = parse_hex_buf(aes_key, 16, &p);` | fs_net.c | exported | Unvalidated |
| `if (aes_key_len < 0)` | fs_net.c | exported | Unvalidated |
| `if (parse_uint32(&flags, &p) < 0)` | fs_net.c | exported | Unvalidated |
| `if (aes_key_len != 0 && aes_key_len != 16)` | fs_net.c | exported | Unvalidated |
| `if (user_buf[0] != '\0') … else` | fs_net.c | exported | Unvalidated |
| `if (password_buf[0] != '\0') … else` | fs_net.c | exported | Unvalidated |
| `((void) sizeof (__assert_single_arg ( !fs->fs_attach(fs, &root_fd, &qid, uid, "", "") )), __ext…` | fs_net.c | exported | Unvalidated |
| `if (!fd)` | fs_net.c | exported | Unvalidated |
| `fs->fs_unlinkat(fs, fd, name); err = fs->fs_create(fs, &qid, fd, name, 0x00000002 | 0x00000200,…` | fs_net.c | exported | Unvalidated |
| `if (err < 0)` | fs_net.c | exported | Unvalidated |
| `if (post_filename[0] != '\0') … else` | fs_net.c | exported | Unvalidated |
| `s = mallocz(sizeof(*s)); s->root_fd = root_fd; s->fd = fd; s->post_fd = post_fd;` | fs_net.c | exported | Unvalidated |
| `if (aes_key_len != 0) … else` | fs_net.c | exported | Unvalidated |
| `req = mallocz(sizeof(*req)); req->type = FS_CMD_XHR; req->reply_len = 0; req->xhr_state = s; s-…` | fs_net.c | exported | Unvalidated |
| `fail1: if (fd) fs->fs_delete(fs, fd);` | fs_net.c | exported | Unvalidated |
| `if (post_fd)` | fs_net.c | exported | Unvalidated |
| `fs->fs_delete(fs, root_fd); return err;` | fs_net.c | exported | Unvalidated |
| `fail: return -5;` | fs_net.c | exported | Unvalidated |
| `fs_cmd_xhr_on_load` | fs_net.c | left out | not translated yet |
| `if (s->fd)` | fs_net.c | exported | Unvalidated |
| `if (s->post_fd)` | fs_net.c | exported | Unvalidated |
| `fs->fs_delete(fs, s->root_fd);` | fs_net.c | exported | Unvalidated |
| `if (s->req_fd)` | fs_net.c | exported | Unvalidated |
| `free(s);` | fs_net.c | exported | Unvalidated |
| `fs_cmd_set_base_url` | fs_net.c | left out | not translated yet |
| `if (parse_fname(base_url_id, sizeof(base_url_id), &p) < 0)` | fs_net.c | exported | Unvalidated |
| `if (parse_fname(url, sizeof(url), &p) < 0)` | fs_net.c | exported | Unvalidated |
| `if (parse_fname(user_buf, sizeof(user_buf), &p) < 0)` | fs_net.c | exported | Unvalidated |
| `if (parse_fname(password_buf, sizeof(password_buf), &p) < 0)` | fs_net.c | exported | Unvalidated |
| `aes_key_len = parse_hex_buf(aes_key, 16, &p);` | fs_net.c | exported | Unvalidated |
| `if (aes_key_len < 0)` | fs_net.c | exported | Unvalidated |
| `if (user_buf[0] != '\0') … else` | fs_net.c | exported | Unvalidated |
| `if (password_buf[0] != '\0') … else` | fs_net.c | exported | Unvalidated |
| `if (aes_key_len != 0) … else` | fs_net.c | exported | Unvalidated |
| `fs_net_set_base_url(fs, base_url_id, url, user, password, paes_state); return 0;` | fs_net.c | exported | Unvalidated |
| `fail: return -22;` | fs_net.c | exported | Unvalidated |
| `fs_cmd_reset_base_url` | fs_net.c | left out | not translated yet |
| `if (parse_fname(base_url_id, sizeof(base_url_id), &p) < 0)` | fs_net.c | exported | Unvalidated |
| `fs_net_reset_base_url(fs, base_url_id); return 0;` | fs_net.c | exported | Unvalidated |
| `fail: return -22;` | fs_net.c | exported | Unvalidated |
| `fs_cmd_set_url` | fs_net.c | left out | not translated yet |
| `if (parse_fname(filename, sizeof(filename), &p) < 0)` | fs_net.c | exported | Unvalidated |
| `if (parse_fname(base_url_id, sizeof(base_url_id), &p) < 0)` | fs_net.c | exported | Unvalidated |
| `if (parse_file_id(&file_id, &p) < 0)` | fs_net.c | exported | Unvalidated |
| `if (parse_uint64(&size, &p) < 0)` | fs_net.c | exported | Unvalidated |
| `n = inode_search_path(fs, filename);` | fs_net.c | exported | Unvalidated |
| `if (!n)` | fs_net.c | exported | Unvalidated |
| `return fs_net_set_url(fs, n, base_url_id, file_id, size);` | fs_net.c | exported | Unvalidated |
| `fail: return -22;` | fs_net.c | exported | Unvalidated |
| `fs_cmd_export_file` | fs_net.c | left out | not translated yet |
| `if (parse_fname(filename, sizeof(filename), &p) < 0)` | fs_net.c | exported | Unvalidated |
| `n = inode_search_path(fs, filename);` | fs_net.c | exported | Unvalidated |
| `if (!n)` | fs_net.c | exported | Unvalidated |
| `if (n->type != FT_REG || (n->u.reg.state != REG_STATE_LOCAL && n->u.reg.state != REG_STATE_LOAD…` | fs_net.c | exported | Unvalidated |
| `name = _Generic (0 ? ( filename ) : (void *) 1, const void *: (const char *) (strrchr ( filenam…` | fs_net.c | exported | Unvalidated |
| `if (name) … else` | fs_net.c | exported | Unvalidated |
| `buf = malloc(n->u.reg.size); file_buffer_read(&n->u.reg.fbuf, 0, buf, n->u.reg.size); fs_export…` | fs_net.c | exported | Unvalidated |
| `fail: return -5;` | fs_net.c | exported | Unvalidated |
| `fs_cmd_pbkdf2` | fs_net.c | left out | not translated yet |
| `if (f->req != ((void *)0)` | fs_net.c | exported | Unvalidated |
| `pwd_len = parse_hex_buf(pwd, sizeof(pwd), &p);` | fs_net.c | exported | Unvalidated |
| `if (pwd_len < 0)` | fs_net.c | exported | Unvalidated |
| `salt_len = parse_hex_buf(salt, sizeof(salt), &p);` | fs_net.c | exported | Unvalidated |
| `if (pwd_len < 0)` | fs_net.c | exported | Unvalidated |
| `if (parse_uint32(&iter, &p) < 0)` | fs_net.c | exported | Unvalidated |
| `if (parse_uint32(&key_len, &p) < 0)` | fs_net.c | exported | Unvalidated |
| `if (key_len > 64 || key_len == 0)` | fs_net.c | exported | Unvalidated |
| `req = mallocz(sizeof(*req)); req->type = FS_CMD_PBKDF2; req->reply_len = key_len; pbkdf2_hmac_s…` | fs_net.c | exported | Unvalidated |
| `fail: return -22;` | fs_net.c | exported | Unvalidated |
| `fs_cmd_set_import_dir` | fs_net.c | left out | not translated yet |
| `if (parse_fname(filename, sizeof(filename), &p) < 0)` | fs_net.c | exported | Unvalidated |
| `free(fs1->import_dir); fs1->import_dir = strdup(filename); return 0;` | fs_net.c | exported | Unvalidated |
| `fs_cmd_write` | fs_net.c | left out | not translated yet |
| `char *buf1; const char *p; char cmd[64]; int err; buf1 = malloc(buf_len + 1); memcpy(buf1, buf,…` | fs_net.c | exported | Unvalidated |
| `if (parse_fname(cmd, sizeof(cmd), &p) < 0)` | fs_net.c | exported | Unvalidated |
| `if (!strcmp(cmd, "xhr")) … else` | fs_net.c | exported | Unvalidated |
| `free(buf1);` | fs_net.c | exported | Unvalidated |
| `if (err == 0) … else` | fs_net.c | exported | Unvalidated |
| `fs_cmd_read` | fs_net.c | left out | not translated yet |
| `FSCMDRequest *req; int l; req = f->req;` | fs_net.c | exported | Unvalidated |
| `if (!req)` | fs_net.c | exported | Unvalidated |
| `l = min_int(req->reply_len, buf_len); memcpy(buf, req->reply_buf, l); return l;` | fs_net.c | exported | Unvalidated |
| `fs_cmd_close` | fs_net.c | left out | not translated yet |
| `FSCMDRequest *req; req = f->req;` | fs_net.c | exported | Unvalidated |
| `if (req)` | fs_net.c | exported | Unvalidated |
| `fs_net_set_pwd` | fs_net.c | left out | not translated yet |
| `FSFile *root_fd; FSQID qid; ((void) sizeof (__assert_single_arg ( fs_is_net(fs) )), __extension…` | fs_net.c | exported | Unvalidated |
| `fs_export_file` | fs_net.c | left out | not translated yet |
| `compose_path` | fs_utils.c | left out | not translated yet |
| `if (path[0] == '\0') … else` | fs_utils.c | exported | Unvalidated |
| `return d;` | fs_utils.c | exported | Unvalidated |
| `compose_url` | fs_utils.c | left out | not translated yet |
| `if ( _Generic (0 ? ( name ) : (void *) 1, const void *: (const char *) (strchr ( name , ':' )),…` | fs_utils.c | exported | Unvalidated |
| `skip_line` | fs_utils.c | left out | not translated yet |
| `const char *p; p = *pp;` | fs_utils.c | exported | Unvalidated |
| `while (*p != '\n' && *p != '\0')` | fs_utils.c | exported | Unvalidated |
| `if (*p == '\n')` | fs_utils.c | exported | Unvalidated |
| `*pp = p;` | fs_utils.c | exported | Unvalidated |
| `quoted_str` | fs_utils.c | left out | not translated yet |
| `if (str[0] == '\0')` | fs_utils.c | exported | Unvalidated |
| `s = str;` | fs_utils.c | exported | Unvalidated |
| `while (*s != '\0')` | fs_utils.c | exported | Unvalidated |
| `return strdup(str);` | fs_utils.c | exported | Unvalidated |
| `use_quote: buf = malloc(strlen(str) * 4 + 2 + 1);` | fs_utils.c | exported | Unvalidated |
| `q = buf; s = str; *q++ = '"';` | fs_utils.c | exported | Unvalidated |
| `while (*s != '\0')` | fs_utils.c | exported | Unvalidated |
| `*q++ = '"'; *q = '\0'; return buf;` | fs_utils.c | exported | Unvalidated |
| `parse_fname` | fs_utils.c | left out | not translated yet |
| `const char *p; char *q; int c, h; p = *pp;` | fs_utils.c | exported | Unvalidated |
| `while (isspace_nolf(*p))` | fs_utils.c | exported | Unvalidated |
| `if (*p == '\0')` | fs_utils.c | exported | Unvalidated |
| `q = buf;` | fs_utils.c | exported | Unvalidated |
| `if (*p == '"') … else` | fs_utils.c | exported | Unvalidated |
| `*q = '\0'; *pp = p; return 0;` | fs_utils.c | exported | Unvalidated |
| `parse_uint32_base` | fs_utils.c | left out | not translated yet |
| `const char *p, *p1; p = *pp;` | fs_utils.c | exported | Unvalidated |
| `while (isspace_nolf(*p))` | fs_utils.c | exported | Unvalidated |
| `*pval = strtoul(p, (char **)&p1, base);` | fs_utils.c | exported | Unvalidated |
| `if (p1 == p)` | fs_utils.c | exported | Unvalidated |
| `*pp = p1; return 0;` | fs_utils.c | exported | Unvalidated |
| `parse_uint64_base` | fs_utils.c | left out | not translated yet |
| `const char *p, *p1; p = *pp;` | fs_utils.c | exported | Unvalidated |
| `while (isspace_nolf(*p))` | fs_utils.c | exported | Unvalidated |
| `*pval = strtoull(p, (char **)&p1, base);` | fs_utils.c | exported | Unvalidated |
| `if (p1 == p)` | fs_utils.c | exported | Unvalidated |
| `*pp = p1; return 0;` | fs_utils.c | exported | Unvalidated |
| `parse_uint64` | fs_utils.c | left out | not translated yet |
| `return parse_uint64_base(pval, pp, 0);` | fs_utils.c | exported | Unvalidated |
| `parse_uint32` | fs_utils.c | left out | not translated yet |
| `return parse_uint32_base(pval, pp, 0);` | fs_utils.c | exported | Unvalidated |
| `parse_time` | fs_utils.c | left out | not translated yet |
| `const char *p; uint32_t v, m; p = *pp;` | fs_utils.c | exported | Unvalidated |
| `if (parse_uint32(psec, &p) < 0)` | fs_utils.c | exported | Unvalidated |
| `v = 0;` | fs_utils.c | exported | Unvalidated |
| `if (*p == '.')` | fs_utils.c | exported | Unvalidated |
| `*pnsec = v; *pp = p; return 0;` | fs_utils.c | exported | Unvalidated |
| `parse_file_id` | fs_utils.c | left out | not translated yet |
| `return parse_uint64_base(pval, pp, 16);` | fs_utils.c | exported | Unvalidated |
| `file_id_to_filename` | fs_utils.c | left out | not translated yet |
| `sprintf(buf, "%016" "l" "x" , file_id); return buf;` | fs_utils.c | exported | Unvalidated |
| `encode_hex` | fs_utils.c | left out | not translated yet |
| `for (i = 0; i < len; i++)` | fs_utils.c | exported | Unvalidated |
| `decode_hex` | fs_utils.c | left out | not translated yet |
| `for (i = 0; i < len; i++)` | fs_utils.c | exported | Unvalidated |
| `return 0;` | fs_utils.c | exported | Unvalidated |
| `skip_header` | fs_utils.c | left out | not translated yet |
| `p = _Generic (0 ? ( p ) : (void *) 1, const void *: (const char *) (strstr ( p , "\n\n" )), def…` | fs_utils.c | exported | Unvalidated |
| `if (!p)` | fs_utils.c | exported | Unvalidated |
| `return p + 2;` | fs_utils.c | exported | Unvalidated |
| `parse_tag` | fs_utils.c | left out | not translated yet |
| `char tagname[128], *q; const char *p, *p1; int len; p = str;` | fs_utils.c | exported | Unvalidated |
| `for (; ; )` | fs_utils.c | exported | Unvalidated |
| `return -1;` | fs_utils.c | exported | Unvalidated |
| `parse_tag_uint64` | fs_utils.c | left out | not translated yet |
| `if (parse_tag(buf, sizeof(buf), str, tag))` | fs_utils.c | exported | Unvalidated |
| `p = buf; return parse_uint64(pval, &p);` | fs_utils.c | exported | Unvalidated |
| `parse_tag_file_id` | fs_utils.c | left out | not translated yet |
| `if (parse_tag(buf, sizeof(buf), str, tag))` | fs_utils.c | exported | Unvalidated |
| `p = buf; return parse_uint64_base(pval, &p, 16);` | fs_utils.c | exported | Unvalidated |
| `parse_tag_version` | fs_utils.c | left out | not translated yet |
| `if (parse_tag_uint64(&version, str, "Version"))` | fs_utils.c | exported | Unvalidated |
| `return version;` | fs_utils.c | exported | Unvalidated |
| `is_url` | fs_utils.c | left out | not translated yet |
| `return (strstart(path, "http:", ((void *)0) ) || strstart(path, "https:", ((void *)0) ) || strs…` | fs_utils.c | exported | Unvalidated |
| `isspace_nolf` | fs_utils.h | left out | not translated yet |
| `return (c == ' ' || c == '\t');` | fs_utils.h | exported | Unvalidated |
| `from_hex` | fs_utils.h | left out | not translated yet |
| `if (c >= '0' && c <= '9') … else` | fs_utils.h | exported | Unvalidated |
| `block_align` | fs_utils.h | left out | not translated yet |
| `return (val + align - 1) & ~(align - 1);` | fs_utils.h | exported | Unvalidated |
| `padstr` | ide.c | left out | not translated yet |
| `for (i = 0; i < len; i++)` | ide.c | exported | Unvalidated |
| `stw` | ide.c | left out | not translated yet |
| `*buf = v;` | ide.c | exported | Unvalidated |
| `ide_identify` | ide.c | left out | not translated yet |
| `uint16_t *tab; uint32_t oldsize; tab = (uint16_t *)s->io_buffer; memset(tab, 0, 512 * 2); stw(t…` | ide.c | exported | Unvalidated |
| `if (s->mult_sectors)` | ide.c | exported | Unvalidated |
| `stw(tab + 60, s->nb_sectors); stw(tab + 61, s->nb_sectors >> 16); stw(tab + 80, (1 << 1) | (1 <…` | ide.c | exported | Unvalidated |
| `ide_set_signature` | ide.c | left out | not translated yet |
| `s->select &= 0xf0; s->nsector = 1; s->sector = 1; s->lcyl = 0; s->hcyl = 0;` | ide.c | exported | Unvalidated |
| `ide_abort_command` | ide.c | left out | not translated yet |
| `s->status = 0x40 | 0x01; s->error = 0x04;` | ide.c | exported | Unvalidated |
| `ide_set_irq` | ide.c | left out | not translated yet |
| `if (!(ide_if->cmd & 0x02)` | ide.c | exported | Unvalidated |
| `ide_transfer_start` | ide.c | left out | not translated yet |
| `s->end_transfer_func = end_transfer_func; s->data_index = 0; s->data_end = size;` | ide.c | exported | Unvalidated |
| `ide_transfer_stop` | ide.c | left out | not translated yet |
| `s->end_transfer_func = ide_transfer_stop; s->data_index = 0; s->data_end = 0;` | ide.c | exported | Unvalidated |
| `ide_get_sector` | ide.c | left out | not translated yet |
| `if (s->select & 0x40) … else` | ide.c | exported | Unvalidated |
| `return sector_num;` | ide.c | exported | Unvalidated |
| `ide_set_sector` | ide.c | left out | not translated yet |
| `if (s->select & 0x40) … else` | ide.c | exported | Unvalidated |
| `ide_sector_read` | ide.c | left out | not translated yet |
| `int64_t sector_num; int ret, n; sector_num = ide_get_sector(s); n = s->nsector;` | ide.c | exported | Unvalidated |
| `if (n == 0)` | ide.c | exported | Unvalidated |
| `if (n > s->req_nb_sectors)` | ide.c | exported | Unvalidated |
| `s->io_nb_sectors = n; ret = s->bs->read_async(s->bs, sector_num, s->io_buffer, n, ide_sector_re…` | ide.c | exported | Unvalidated |
| `if (ret < 0) … else` | ide.c | exported | Unvalidated |
| `ide_sector_read_cb` | ide.c | left out | not translated yet |
| `IDEState *s = opaque; int n; EndTransferFunc *func; n = s->io_nb_sectors; ide_set_sector(s, ide…` | ide.c | exported | Unvalidated |
| `if (s->nsector == 0) … else` | ide.c | exported | Unvalidated |
| `ide_transfer_start(s, 512 * n, func); ide_set_irq(s); s->status = 0x40 | 0x10 | 0x08; s->error…` | ide.c | exported | Unvalidated |
| `ide_sector_read_cb_end` | ide.c | left out | not translated yet |
| `s->status = 0x40 | 0x10; s->error = 0; ide_transfer_stop(s);` | ide.c | exported | Unvalidated |
| `ide_sector_write_cb1` | ide.c | left out | not translated yet |
| `int64_t sector_num; int ret; ide_transfer_stop(s); sector_num = ide_get_sector(s); ret = s->bs-…` | ide.c | exported | Unvalidated |
| `if (ret < 0) … else` | ide.c | exported | Unvalidated |
| `ide_sector_write_cb2` | ide.c | left out | not translated yet |
| `IDEState *s = opaque; int n; n = s->io_nb_sectors; ide_set_sector(s, ide_get_sector(s) + n); s-…` | ide.c | exported | Unvalidated |
| `if (s->nsector == 0) … else` | ide.c | exported | Unvalidated |
| `ide_set_irq(s);` | ide.c | exported | Unvalidated |
| `ide_sector_write` | ide.c | left out | not translated yet |
| `int n; n = s->nsector;` | ide.c | exported | Unvalidated |
| `if (n == 0)` | ide.c | exported | Unvalidated |
| `if (n > s->req_nb_sectors)` | ide.c | exported | Unvalidated |
| `s->io_nb_sectors = n; ide_transfer_start(s, 512 * n, ide_sector_write_cb1); s->status = 0x40 |…` | ide.c | exported | Unvalidated |
| `ide_identify_cb` | ide.c | left out | not translated yet |
| `ide_transfer_stop(s); s->status = 0x40;` | ide.c | exported | Unvalidated |
| `ide_exec_cmd` | ide.c | left out | not translated yet |
| `switch (val)` | ide.c | exported | Unvalidated |
| `ide_ioport_write` | ide.c | left out | not translated yet |
| `switch (addr)` | ide.c | exported | Unvalidated |
| `ide_ioport_read` | ide.c | left out | not translated yet |
| `if (!s) … else` | ide.c | exported | Unvalidated |
| `return ret;` | ide.c | exported | Unvalidated |
| `ide_status_read` | ide.c | left out | not translated yet |
| `if (s) … else` | ide.c | exported | Unvalidated |
| `return ret;` | ide.c | exported | Unvalidated |
| `ide_cmd_write` | ide.c | left out | not translated yet |
| `if (!(s1->cmd & 0x04) && (val & 0x04) … else` | ide.c | exported | Unvalidated |
| `s1->cmd = val;` | ide.c | exported | Unvalidated |
| `ide_data_writew` | ide.c | left out | not translated yet |
| `if (!s)` | ide.c | exported | Unvalidated |
| `p = s->data_index; tab = s->io_buffer; tab[p] = val & 0xff; tab[p + 1] = (val >> 8) & 0xff; p +…` | ide.c | exported | Unvalidated |
| `if (p >= s->data_end)` | ide.c | exported | Unvalidated |
| `ide_data_readw` | ide.c | left out | not translated yet |
| `if (!s) … else` | ide.c | exported | Unvalidated |
| `return ret;` | ide.c | exported | Unvalidated |
| `ide_drive_init` | ide.c | left out | not translated yet |
| `IDEState *s; uint32_t cylinders; uint64_t nb_sectors; s = malloc(sizeof(*s)); memset(s, 0, size…` | ide.c | exported | Unvalidated |
| `if (cylinders > 16383) … else` | ide.c | exported | Unvalidated |
| `s->cylinders = cylinders; s->heads = 16; s->sectors = 63; s->nb_sectors = nb_sectors; s->mult_s…` | ide.c | exported | Unvalidated |
| `ide_init` | ide.c | left out | not translated yet |
| `int i; IDEIFState *s; s = malloc(sizeof(IDEIFState)); memset(s, 0, sizeof(*s)); s->irq = irq; s…` | ide.c | exported | Unvalidated |
| `if (addr2)` | ide.c | exported | Unvalidated |
| `for (i = 0; i < 2; i++)` | ide.c | exported | Unvalidated |
| `s->cur_drive = s->drives[0]; return s;` | ide.c | exported | Unvalidated |
| `piix3_ide_init` | ide.c | left out | not translated yet |
| `PCIDevice *d; d = pci_register_device(pci_bus, "PIIX3 IDE", devfn, 0x8086, 0x7010, 0x00, 0x0101…` | ide.c | exported | Unvalidated |
| `phys_mem_map_init` | iomem.c | left out | not translated yet |
| `PhysMemoryMap *s; s = mallocz(sizeof(*s)); s->register_ram = default_register_ram; s->free_ram…` | iomem.c | exported | Unvalidated |
| `phys_mem_map_end` | iomem.c | left out | not translated yet |
| `for (i = 0; i < s->n_phys_mem_range; i++)` | iomem.c | exported | Unvalidated |
| `free(s);` | iomem.c | exported | Unvalidated |
| `get_phys_mem_range` | iomem.c | left out | not translated yet |
| `for (i = 0; i < s->n_phys_mem_range; i++)` | iomem.c | exported | Unvalidated |
| `return ((void *)0) ;` | iomem.c | exported | Unvalidated |
| `register_ram_entry` | iomem.c | left out | not translated yet |
| `PhysMemoryRange *pr; ((void) sizeof (__assert_single_arg ( s->n_phys_mem_range < 32 )), __exten…` | iomem.c | exported | Unvalidated |
| `if (devram_flags & (1 << 2) … else` | iomem.c | exported | Unvalidated |
| `pr->phys_mem = ((void *)0) ; pr->dirty_bits = ((void *)0) ; return pr;` | iomem.c | exported | Unvalidated |
| `default_register_ram` | iomem.c | left out | not translated yet |
| `PhysMemoryRange *pr; pr = register_ram_entry(s, addr, size, devram_flags); pr->phys_mem = mallo…` | iomem.c | exported | Unvalidated |
| `if (!pr->phys_mem)` | iomem.c | exported | Unvalidated |
| `if (devram_flags & (1 << 1)` | iomem.c | exported | Unvalidated |
| `return pr;` | iomem.c | exported | Unvalidated |
| `default_get_dirty_bits` | iomem.c | left out | not translated yet |
| `uint32_t *dirty_bits; BOOL has_dirty_bits; size_t n, i; dirty_bits = pr->dirty_bits; has_dirty_…` | iomem.c | exported | Unvalidated |
| `for (i = 0; i < n; i++)` | iomem.c | exported | Unvalidated |
| `if (has_dirty_bits && pr->size != 0)` | iomem.c | exported | Unvalidated |
| `pr->dirty_bits_index ^= 1; pr->dirty_bits = pr->dirty_bits_tab[pr->dirty_bits_index]; memset(pr…` | iomem.c | exported | Unvalidated |
| `phys_mem_reset_dirty_bit` | iomem.c | left out | not translated yet |
| `if (pr->dirty_bits)` | iomem.c | exported | Unvalidated |
| `default_free_ram` | iomem.c | left out | not translated yet |
| `free(pr->phys_mem);` | iomem.c | exported | Unvalidated |
| `cpu_register_device` | iomem.c | left out | not translated yet |
| `PhysMemoryRange *pr; ((void) sizeof (__assert_single_arg ( s->n_phys_mem_range < 32 )), __exten…` | iomem.c | exported | Unvalidated |
| `if (devio_flags & (1 << 4) … else` | iomem.c | exported | Unvalidated |
| `pr->is_ram = FALSE; pr->opaque = opaque; pr->read_func = read_func; pr->write_func = write_func…` | iomem.c | exported | Unvalidated |
| `default_set_addr` | iomem.c | left out | not translated yet |
| `if (enabled) … else` | iomem.c | exported | Unvalidated |
| `phys_mem_set_addr` | iomem.c | left out | not translated yet |
| `if (!pr->is_ram) … else` | iomem.c | exported | Unvalidated |
| `phys_mem_get_ram_ptr` | iomem.c | left out | not translated yet |
| `if (!pr || !pr->is_ram)` | iomem.c | exported | Unvalidated |
| `offset = paddr - pr->addr;` | iomem.c | exported | Unvalidated |
| `if (is_rw)` | iomem.c | exported | Unvalidated |
| `return pr->phys_mem + (uintptr_t)offset;` | iomem.c | exported | Unvalidated |
| `irq_init` | iomem.c | left out | not translated yet |
| `irq->set_irq = set_irq; irq->opaque = opaque; irq->irq_num = irq_num;` | iomem.c | exported | Unvalidated |
| `cpu_register_ram` | iomem.h | left out | not translated yet |
| `return s->register_ram(s, addr, size, devram_flags);` | iomem.h | exported | Unvalidated |
| `phys_mem_get_dirty_bits` | iomem.h | left out | not translated yet |
| `PhysMemoryMap *map = pr->map; return map->get_dirty_bits(map, pr);` | iomem.h | exported | Unvalidated |
| `phys_mem_set_dirty_bit` | iomem.h | left out | not translated yet |
| `if (pr->dirty_bits)` | iomem.h | exported | Unvalidated |
| `phys_mem_is_dirty_bit` | iomem.h | left out | not translated yet |
| `if (!pr->dirty_bits)` | iomem.h | exported | Unvalidated |
| `page_index = offset >> 12; dirty_bits_ptr = pr->dirty_bits + (page_index >> 5); return (*dirty_…` | iomem.h | exported | Unvalidated |
| `set_irq` | iomem.h | left out | not translated yet |
| `irq->set_irq(irq->opaque, irq->irq_num, level);` | iomem.h | exported | Unvalidated |
| `parse_string` | json.c | left out | not translated yet |
| `char buf[4096], *q; const char *p; int c, h; q = buf; p = *pp; p++;` | json.c | exported | Unvalidated |
| `for (; ; )` | json.c | exported | Unvalidated |
| `*q = '\0'; *pp = p; return json_string_new(buf);` | json.c | exported | Unvalidated |
| `json_object_get2` | json.c | left out | not translated yet |
| `for (i = 0; i < obj->len; i++)` | json.c | exported | Unvalidated |
| `return ((void *)0) ;` | json.c | exported | Unvalidated |
| `json_object_get` | json.c | left out | not translated yet |
| `if (val.type != JSON_OBJ)` | json.c | exported | Unvalidated |
| `obj = val.u.obj; f = json_object_get2(obj, name);` | json.c | exported | Unvalidated |
| `if (!f)` | json.c | exported | Unvalidated |
| `return f->value;` | json.c | exported | Unvalidated |
| `json_object_set` | json.c | left out | not translated yet |
| `if (val.type != JSON_OBJ)` | json.c | exported | Unvalidated |
| `obj = val.u.obj; f = json_object_get2(obj, name);` | json.c | exported | Unvalidated |
| `if (f) … else` | json.c | exported | Unvalidated |
| `return 0;` | json.c | exported | Unvalidated |
| `json_array_get` | json.c | left out | not translated yet |
| `if (val.type != JSON_ARRAY)` | json.c | exported | Unvalidated |
| `array = val.u.array;` | json.c | exported | Unvalidated |
| `if (idx < array->len) … else` | json.c | exported | Unvalidated |
| `json_array_set` | json.c | left out | not translated yet |
| `if (val.type != JSON_ARRAY)` | json.c | exported | Unvalidated |
| `array = val.u.array;` | json.c | exported | Unvalidated |
| `if (idx < array->len) … else` | json.c | exported | Unvalidated |
| `return 0;` | json.c | exported | Unvalidated |
| `json_get_str` | json.c | left out | not translated yet |
| `if (val.type != JSON_STR)` | json.c | exported | Unvalidated |
| `return val.u.str->data;` | json.c | exported | Unvalidated |
| `json_get_error` | json.c | left out | not translated yet |
| `if (val.type != JSON_EXCEPTION)` | json.c | exported | Unvalidated |
| `return val.u.str->data;` | json.c | exported | Unvalidated |
| `json_string_new2` | json.c | left out | not translated yet |
| `JSONValue val; JSONString *str1; str1 = malloc(sizeof(JSONString) + len + 1); str1->len = len;…` | json.c | exported | Unvalidated |
| `json_string_new` | json.c | left out | not translated yet |
| `return json_string_new2(str, strlen(str));` | json.c | exported | Unvalidated |
| `json_object_new` | json.c | left out | not translated yet |
| `JSONValue val; JSONObject *obj; obj = mallocz(sizeof(JSONObject)); val.type = JSON_OBJ; val.u.o…` | json.c | exported | Unvalidated |
| `json_array_new` | json.c | left out | not translated yet |
| `JSONValue val; JSONArray *array; array = mallocz(sizeof(JSONArray)); val.type = JSON_ARRAY; val…` | json.c | exported | Unvalidated |
| `json_free` | json.c | left out | not translated yet |
| `switch (val.type)` | json.c | exported | Unvalidated |
| `skip_spaces` | json.c | left out | not translated yet |
| `const char *p; p = *pp;` | json.c | exported | Unvalidated |
| `for (; ; )` | json.c | exported | Unvalidated |
| `*pp = p;` | json.c | exported | Unvalidated |
| `is_ident_first` | json.c | left out | not translated yet |
| `return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_' || c == '$';` | json.c | exported | Unvalidated |
| `parse_ident` | json.c | left out | not translated yet |
| `char *q; const char *p; p = *pp; q = buf; *q++ = *p++;` | json.c | exported | Unvalidated |
| `while (is_ident_first(*p) || ((*__ctype_b_loc ())[(int) (( *p ))] & (unsigned short int) _ISdig…` | json.c | exported | Unvalidated |
| `*pp = p; *q = '\0'; return 0;` | json.c | exported | Unvalidated |
| `json_parse_value2` | json.c | left out | not translated yet |
| `char buf[128]; const char *p; JSONValue val, val1, tag; p = *pp; skip_spaces(&p);` | json.c | exported | Unvalidated |
| `if (*p == '\0')` | json.c | exported | Unvalidated |
| `if (*__ctype_b_loc ())[(int) (( *p ))] & (unsigned short int) _ISdigit) … else` | json.c | exported | Unvalidated |
| `*pp = p; return val;` | json.c | exported | Unvalidated |
| `json_parse_value` | json.c | left out | not translated yet |
| `JSONValue val; val = json_parse_value2(&p);` | json.c | exported | Unvalidated |
| `if (json_is_error(val))` | json.c | exported | Unvalidated |
| `skip_spaces(&p);` | json.c | exported | Unvalidated |
| `if (*p != '\0')` | json.c | exported | Unvalidated |
| `return val;` | json.c | exported | Unvalidated |
| `json_parse_value_len` | json.c | left out | not translated yet |
| `char *str; JSONValue val; str = malloc(len + 1); memcpy(str, p, len); str[len] = '\0'; val = js…` | json.c | exported | Unvalidated |
| `json_is_error` | json.h | left out | not translated yet |
| `return val.type == JSON_EXCEPTION;` | json.h | exported | Unvalidated |
| `json_is_undefined` | json.h | left out | not translated yet |
| `return val.type == JSON_UNDEFINED;` | json.h | exported | Unvalidated |
| `json_undefined_new` | json.h | left out | not translated yet |
| `JSONValue val; val.type = JSON_UNDEFINED; val.u.int32 = 0; return val;` | json.h | exported | Unvalidated |
| `json_null_new` | json.h | left out | not translated yet |
| `JSONValue val; val.type = JSON_NULL; val.u.int32 = 0; return val;` | json.h | exported | Unvalidated |
| `json_int32_new` | json.h | left out | not translated yet |
| `JSONValue val; val.type = JSON_INT; val.u.int32 = v; return val;` | json.h | exported | Unvalidated |
| `json_bool_new` | json.h | left out | not translated yet |
| `JSONValue val; val.type = JSON_BOOL; val.u.b = v; return val;` | json.h | exported | Unvalidated |
| `init_list_head` | list.h | left out | not translated yet |
| `head->prev = head; head->next = head;` | list.h | exported | Unvalidated |
| `__list_add` | list.h | left out | not translated yet |
| `prev->next = el; el->prev = prev; el->next = next; next->prev = el;` | list.h | exported | Unvalidated |
| `list_add` | list.h | left out | not translated yet |
| `__list_add(el, head, head->next);` | list.h | exported | Unvalidated |
| `list_add_tail` | list.h | left out | not translated yet |
| `__list_add(el, head->prev, head);` | list.h | exported | Unvalidated |
| `list_del` | list.h | left out | not translated yet |
| `struct list_head *prev, *next; prev = el->prev; next = el->next; prev->next = next; next->prev…` | list.h | exported | Unvalidated |
| `list_empty` | list.h | left out | not translated yet |
| `return el->next == el;` | list.h | exported | Unvalidated |
| `vm_get_int` | machine.c | left out | not translated yet |
| `JSONValue val; val = json_object_get(obj, name);` | machine.c | exported | Unvalidated |
| `if (json_is_undefined(val))` | machine.c | exported | Unvalidated |
| `if (val.type != JSON_INT)` | machine.c | exported | Unvalidated |
| `*pval = val.u.int32; return 0;` | machine.c | exported | Unvalidated |
| `vm_get_int_opt` | machine.c | left out | not translated yet |
| `JSONValue val; val = json_object_get(obj, name);` | machine.c | exported | Unvalidated |
| `if (json_is_undefined(val))` | machine.c | exported | Unvalidated |
| `if (val.type != JSON_INT)` | machine.c | exported | Unvalidated |
| `*pval = val.u.int32; return 0;` | machine.c | exported | Unvalidated |
| `vm_get_str2` | machine.c | left out | not translated yet |
| `JSONValue val; val = json_object_get(obj, name);` | machine.c | exported | Unvalidated |
| `if (json_is_undefined(val))` | machine.c | exported | Unvalidated |
| `if (val.type != JSON_STR)` | machine.c | exported | Unvalidated |
| `*pstr = val.u.str->data; return 0;` | machine.c | exported | Unvalidated |
| `vm_get_str` | machine.c | left out | not translated yet |
| `return vm_get_str2(obj, name, pstr, FALSE);` | machine.c | exported | Unvalidated |
| `vm_get_str_opt` | machine.c | left out | not translated yet |
| `return vm_get_str2(obj, name, pstr, TRUE);` | machine.c | exported | Unvalidated |
| `strdup_null` | machine.c | left out | not translated yet |
| `if (!str) … else` | machine.c | exported | Unvalidated |
| `cmdline_subst` | machine.c | left out | not translated yet |
| `DynBuf dbuf; const char *p; char var_name[32], *q, buf[32]; dbuf_init(&dbuf); p = cmdline;` | machine.c | exported | Unvalidated |
| `while (*p != '\0')` | machine.c | exported | Unvalidated |
| `dbuf_putc(&dbuf, 0); return (char *)dbuf.buf;` | machine.c | exported | Unvalidated |
| `find_name` | machine.c | left out | not translated yet |
| `size_t len; const char *p, *r; p = name_list;` | machine.c | exported | Unvalidated |
| `for (; ; )` | machine.c | exported | Unvalidated |
| `return FALSE;` | machine.c | exported | Unvalidated |
| `virt_machine_find_class` | machine.c | left out | not translated yet |
| `for (pvmc = virt_machine_list; *pvmc != ((void *)0; pvmc++)` | machine.c | exported | Unvalidated |
| `return ((void *)0) ;` | machine.c | exported | Unvalidated |
| `virt_machine_parse_config` | machine.c | left out | not translated yet |
| `int version, val; const char *tag_name, *str; char buf1[256]; JSONValue cfg, obj, el; cfg = jso…` | machine.c | exported | Unvalidated |
| `if (json_is_error(cfg))` | machine.c | exported | Unvalidated |
| `if (vm_get_int(cfg, "version", &version) < 0)` | machine.c | exported | Unvalidated |
| `if (version != 1)` | machine.c | exported | Unvalidated |
| `if (vm_get_str(cfg, "machine", &str) < 0)` | machine.c | exported | Unvalidated |
| `p->machine_name = strdup(str); p->vmc = virt_machine_find_class(p->machine_name);` | machine.c | exported | Unvalidated |
| `if (!p->vmc)` | machine.c | exported | Unvalidated |
| `p->vmc->virt_machine_set_defaults(p); tag_name = "memory_size";` | machine.c | exported | Unvalidated |
| `if (vm_get_int(cfg, tag_name, &val) < 0)` | machine.c | exported | Unvalidated |
| `p->ram_size = (uint64_t)val << 20; tag_name = "bios";` | machine.c | exported | Unvalidated |
| `if (vm_get_str_opt(cfg, tag_name, &str) < 0)` | machine.c | exported | Unvalidated |
| `if (str)` | machine.c | exported | Unvalidated |
| `tag_name = "kernel";` | machine.c | exported | Unvalidated |
| `if (vm_get_str_opt(cfg, tag_name, &str) < 0)` | machine.c | exported | Unvalidated |
| `if (str)` | machine.c | exported | Unvalidated |
| `if (vm_get_str_opt(cfg, "cmdline", &str) < 0)` | machine.c | exported | Unvalidated |
| `if (str)` | machine.c | exported | Unvalidated |
| `tag_name = "initrd";` | machine.c | exported | Unvalidated |
| `if (vm_get_str_opt(cfg, tag_name, &str) < 0)` | machine.c | exported | Unvalidated |
| `if (str)` | machine.c | exported | Unvalidated |
| `for (; ; )` | machine.c | exported | Unvalidated |
| `for (; ; )` | machine.c | exported | Unvalidated |
| `for (; ; )` | machine.c | exported | Unvalidated |
| `p->display_device = ((void *)0) ; obj = json_object_get(cfg, "display0");` | machine.c | exported | Unvalidated |
| `if (!json_is_undefined(obj))` | machine.c | exported | Unvalidated |
| `if (vm_get_str_opt(cfg, "input_device", &str) < 0)` | machine.c | exported | Unvalidated |
| `p->input_device = strdup_null(str);` | machine.c | exported | Unvalidated |
| `if (vm_get_str_opt(cfg, "accel", &str) < 0)` | machine.c | exported | Unvalidated |
| `if (str)` | machine.c | exported | Unvalidated |
| `tag_name = "rtc_local_time"; el = json_object_get(cfg, tag_name);` | machine.c | exported | Unvalidated |
| `if (!json_is_undefined(el))` | machine.c | exported | Unvalidated |
| `json_free(cfg); return 0;` | machine.c | exported | Unvalidated |
| `tag_fail: json_free(cfg);` | machine.c | exported | Unvalidated |
| `return -1;` | machine.c | exported | Unvalidated |
| `get_file_path` | machine.c | left out | not translated yet |
| `if (!base_filename)` | machine.c | exported | Unvalidated |
| `if ( _Generic (0 ? ( filename ) : (void *) 1, const void *: (const char *) (strchr ( filename ,…` | machine.c | exported | Unvalidated |
| `if (filename[0] == '/')` | machine.c | exported | Unvalidated |
| `p = _Generic (0 ? ( base_filename ) : (void *) 1, const void *: (const char *) (strrchr ( base_…` | machine.c | exported | Unvalidated |
| `if (!p)` | machine.c | exported | Unvalidated |
| `len = p + 1 - base_filename; len1 = strlen(filename); fname = malloc(len + len1 + 1); memcpy(fn…` | machine.c | exported | Unvalidated |
| `load_file` | machine.c | left out | not translated yet |
| `FILE *f; int size; uint8_t *buf; f = fopen(filename, "rb");` | machine.c | exported | Unvalidated |
| `if (!f)` | machine.c | exported | Unvalidated |
| `fseek(f, 0, 2 ); size = ftell(f); fseek(f, 0, 0 ); buf = malloc(size);` | machine.c | exported | Unvalidated |
| `if (fread(buf, 1, size, f) != size)` | machine.c | exported | Unvalidated |
| `fclose(f); *pbuf = buf; return size;` | machine.c | exported | Unvalidated |
| `config_load_file_cb` | machine.c | left out | not translated yet |
| `if (err < 0)` | machine.c | exported | Unvalidated |
| `s->file_load_cb(s->file_load_opaque, data, size);` | machine.c | exported | Unvalidated |
| `config_load_file` | machine.c | left out | not translated yet |
| `if (is_url(filename)) … else` | machine.c | exported | Unvalidated |
| `virt_machine_load_config_file` | machine.c | left out | not translated yet |
| `VMConfigLoadState *s; s = mallocz(sizeof(*s)); s->vm_params = p; s->start_cb = start_cb; s->opa…` | machine.c | exported | Unvalidated |
| `config_file_loaded` | machine.c | left out | not translated yet |
| `if (virt_machine_parse_config(p, (char *)buf, buf_len) < 0)` | machine.c | exported | Unvalidated |
| `s->file_index = 0; config_additional_file_load(s);` | machine.c | exported | Unvalidated |
| `config_additional_file_load` | machine.c | left out | not translated yet |
| `while (s->file_index < VM_FILE_COUNT && p->files[s->file_index].filename == ((void *)0)` | machine.c | exported | Unvalidated |
| `if (s->file_index == VM_FILE_COUNT) … else` | machine.c | exported | Unvalidated |
| `config_additional_file_load_cb` | machine.c | left out | not translated yet |
| `VMConfigLoadState *s = opaque; VirtMachineParams *p = s->vm_params; p->files[s->file_index].buf…` | machine.c | exported | Unvalidated |
| `vm_add_cmdline` | machine.c | left out | not translated yet |
| `if (cmdline[0] == '!') … else` | machine.c | exported | Unvalidated |
| `free(p->cmdline); p->cmdline = new_cmdline;` | machine.c | exported | Unvalidated |
| `virt_machine_free_config` | machine.c | left out | not translated yet |
| `int i; free(p->machine_name); free(p->cmdline);` | machine.c | exported | Unvalidated |
| `for (i = 0; i < VM_FILE_COUNT; i++)` | machine.c | exported | Unvalidated |
| `for (i = 0; i < p->drive_count; i++)` | machine.c | exported | Unvalidated |
| `for (i = 0; i < p->fs_count; i++)` | machine.c | exported | Unvalidated |
| `for (i = 0; i < p->eth_count; i++)` | machine.c | exported | Unvalidated |
| `free(p->input_device); free(p->display_device); free(p->cfg_filename);` | machine.c | exported | Unvalidated |
| `virt_machine_init` | machine.c | left out | not translated yet |
| `const VirtMachineClass *vmc = p->vmc; return vmc->virt_machine_init(p);` | machine.c | exported | Unvalidated |
| `virt_machine_set_defaults` | machine.c | left out | not translated yet |
| `memset(p, 0, sizeof(*p));` | machine.c | exported | Unvalidated |
| `virt_machine_end` | machine.c | left out | not translated yet |
| `s->vmc->virt_machine_end(s);` | machine.c | exported | Unvalidated |
| `virt_machine_get_sleep_duration` | machine.h | left out | not translated yet |
| `return s->vmc->virt_machine_get_sleep_duration(s, delay);` | machine.h | exported | Unvalidated |
| `virt_machine_interp` | machine.h | left out | not translated yet |
| `s->vmc->virt_machine_interp(s, max_exec_cycle);` | machine.h | exported | Unvalidated |
| `vm_mouse_is_absolute` | machine.h | left out | not translated yet |
| `return s->vmc->vm_mouse_is_absolute(s);` | machine.h | exported | Unvalidated |
| `vm_send_mouse_event` | machine.h | left out | not translated yet |
| `s1->vmc->vm_send_mouse_event(s1, dx, dy, dz, buttons);` | machine.h | exported | Unvalidated |
| `vm_send_key_event` | machine.h | left out | not translated yet |
| `s1->vmc->vm_send_key_event(s1, is_down, key_code);` | machine.h | exported | Unvalidated |
| `bus_map_irq` | pci.c | left out | not translated yet |
| `int slot_addend; slot_addend = (d->devfn >> 3) - 1; return (irq_num + slot_addend) & 3;` | pci.c | exported | Unvalidated |
| `pci_device_set_irq` | pci.c | left out | not translated yet |
| `PCIDevice *d = opaque; PCIBus *b = d->bus; uint32_t mask; int i, irq_level; irq_num = bus_map_i…` | pci.c | exported | Unvalidated |
| `if (level) … else` | pci.c | exported | Unvalidated |
| `mask = 0;` | pci.c | exported | Unvalidated |
| `for (i = 0; i < 8; i++)` | pci.c | exported | Unvalidated |
| `irq_level = (mask != 0); set_irq(&b->irq[irq_num], irq_level);` | pci.c | exported | Unvalidated |
| `devfn_alloc` | pci.c | left out | not translated yet |
| `for (devfn = 0; devfn < 256; devfn += 8)` | pci.c | exported | Unvalidated |
| `return -1;` | pci.c | exported | Unvalidated |
| `pci_register_device` | pci.c | left out | not translated yet |
| `if (devfn < 0)` | pci.c | exported | Unvalidated |
| `if (b->device[devfn])` | pci.c | exported | Unvalidated |
| `d = mallocz(sizeof(PCIDevice)); d->bus = b; d->name = strdup(name); d->devfn = devfn; put_le16(…` | pci.c | exported | Unvalidated |
| `for (i = 0; i < 4; i++)` | pci.c | exported | Unvalidated |
| `b->device[devfn] = d; return d;` | pci.c | exported | Unvalidated |
| `pci_device_get_irq` | pci.c | left out | not translated yet |
| `((void) sizeof (__assert_single_arg ( irq_num < 4 )), __extension__ ({ if ( irq_num < 4 ) ; els…` | pci.c | exported | Unvalidated |
| `pci_device_config_read` | pci.c | left out | not translated yet |
| `switch (size_log2)` | pci.c | exported | Unvalidated |
| `return val;` | pci.c | exported | Unvalidated |
| `pci_device_get_mem_map` | pci.c | left out | not translated yet |
| `return d->bus->mem_map;` | pci.c | exported | Unvalidated |
| `pci_device_get_port_map` | pci.c | left out | not translated yet |
| `return d->bus->port_map;` | pci.c | exported | Unvalidated |
| `pci_register_bar` | pci.c | left out | not translated yet |
| `PCIIORegion *r; uint32_t val, config_addr; ((void) sizeof (__assert_single_arg ( bar_num < 7 ))…` | pci.c | exported | Unvalidated |
| `if (bar_num == 6) … else` | pci.c | exported | Unvalidated |
| `put_le32(&d->config[config_addr], val);` | pci.c | exported | Unvalidated |
| `pci_update_mappings` | pci.c | left out | not translated yet |
| `int cmd, i, offset; uint32_t new_addr; BOOL new_enabled; PCIIORegion *r; cmd = get_le16(&d->con…` | pci.c | exported | Unvalidated |
| `for (i = 0; i < 7; i++)` | pci.c | exported | Unvalidated |
| `pci_write_bar` | pci.c | left out | not translated yet |
| `if (addr == 0x30) … else` | pci.c | exported | Unvalidated |
| `r = &d->io_regions[reg];` | pci.c | exported | Unvalidated |
| `if (r->size == 0)` | pci.c | exported | Unvalidated |
| `if (reg == 6) … else` | pci.c | exported | Unvalidated |
| `put_le32(d->config + addr, val); pci_update_mappings(d); return 0;` | pci.c | exported | Unvalidated |
| `pci_device_config_write8` | pci.c | left out | not translated yet |
| `if (addr == 0x06 || addr == (0x06 + 1)` | pci.c | exported | Unvalidated |
| `switch (d->config[0x0e])` | pci.c | exported | Unvalidated |
| `if (can_write)` | pci.c | exported | Unvalidated |
| `pci_device_config_write` | pci.c | left out | not translated yet |
| `if (size_log2 == 2 && ((addr >= 0x10 && addr < 0x10 + 4 * 6) || addr == 0x30)` | pci.c | exported | Unvalidated |
| `size = 1 << size_log2;` | pci.c | exported | Unvalidated |
| `for (i = 0; i < size; i++)` | pci.c | exported | Unvalidated |
| `if (0x04 >= addr && 0x04 < addr + size)` | pci.c | exported | Unvalidated |
| `pci_data_write` | pci.c | left out | not translated yet |
| `PCIDevice *d; int bus_num, devfn, config_addr; bus_num = (addr >> 16) & 0xff;` | pci.c | exported | Unvalidated |
| `if (bus_num != s->bus_num)` | pci.c | exported | Unvalidated |
| `devfn = (addr >> 8) & 0xff; d = s->device[devfn];` | pci.c | exported | Unvalidated |
| `if (!d)` | pci.c | exported | Unvalidated |
| `config_addr = addr & 0xff; pci_device_config_write(d, config_addr, data, size_log2);` | pci.c | exported | Unvalidated |
| `pci_data_read` | pci.c | left out | not translated yet |
| `PCIDevice *d; int bus_num, devfn, config_addr; bus_num = (addr >> 16) & 0xff;` | pci.c | exported | Unvalidated |
| `if (bus_num != s->bus_num)` | pci.c | exported | Unvalidated |
| `devfn = (addr >> 8) & 0xff; d = s->device[devfn];` | pci.c | exported | Unvalidated |
| `if (!d)` | pci.c | exported | Unvalidated |
| `config_addr = addr & 0xff; return pci_device_config_read(d, config_addr, size_log2);` | pci.c | exported | Unvalidated |
| `pci_device_get_dma_ptr` | pci.c | left out | not translated yet |
| `return phys_mem_get_ram_ptr(d->bus->mem_map, addr, is_rw);` | pci.c | exported | Unvalidated |
| `pci_device_set_config8` | pci.c | left out | not translated yet |
| `d->config[addr] = val;` | pci.c | exported | Unvalidated |
| `pci_device_set_config16` | pci.c | left out | not translated yet |
| `put_le16(&d->config[addr], val);` | pci.c | exported | Unvalidated |
| `pci_device_get_devfn` | pci.c | left out | not translated yet |
| `return d->devfn;` | pci.c | exported | Unvalidated |
| `pci_add_capability` | pci.c | left out | not translated yet |
| `int offset; offset = d->next_cap_offset;` | pci.c | exported | Unvalidated |
| `if (offset + size) > 256)` | pci.c | exported | Unvalidated |
| `d->next_cap_offset += size; d->config[0x06] |= (1 << 4); memcpy(d->config + offset, buf, size);…` | pci.c | exported | Unvalidated |
| `i440fx_write_addr` | pci.c | left out | not translated yet |
| `I440FXState *s = opaque; s->config_reg = data;` | pci.c | exported | Unvalidated |
| `i440fx_read_addr` | pci.c | left out | not translated yet |
| `I440FXState *s = opaque; return s->config_reg;` | pci.c | exported | Unvalidated |
| `i440fx_write_data` | pci.c | left out | not translated yet |
| `if (s->config_reg & 0x80000000)` | pci.c | exported | Unvalidated |
| `i440fx_read_data` | pci.c | left out | not translated yet |
| `if (!(s->config_reg & 0x80000000)` | pci.c | exported | Unvalidated |
| `if (size_log2 == 2) … else` | pci.c | exported | Unvalidated |
| `i440fx_set_irq` | pci.c | left out | not translated yet |
| `I440FXState *s = opaque; PCIDevice *hd = s->piix3_dev; int pic_irq; hd->config[0x60 + irq_num]…` | pci.c | exported | Unvalidated |
| `if (pic_irq < 16)` | pci.c | exported | Unvalidated |
| `i440fx_init` | pci.c | left out | not translated yet |
| `I440FXState *s; PCIBus *b; PCIDevice *d; int i; s = mallocz(sizeof(*s)); b = mallocz(sizeof(PCI…` | pci.c | exported | Unvalidated |
| `for (i = 0; i < 4; i++)` | pci.c | exported | Unvalidated |
| `cpu_register_device(port_map, 0xcf8, 1, s, i440fx_read_addr, i440fx_write_addr, (1 << 2)); cpu_…` | pci.c | exported | Unvalidated |
| `i440fx_map_interrupts` | pci.c | left out | not translated yet |
| `PCIBus *b = s->pci_bus; PCIDevice *d, *hd; int irq_num, pic_irq, devfn, i; hd = s->piix3_dev; e…` | pci.c | exported | Unvalidated |
| `for (i = 0; i < 4; i++)` | pci.c | exported | Unvalidated |
| `for (devfn = 0; devfn < 256; devfn++)` | pci.c | exported | Unvalidated |
| `qemu_system_reset_request` | pckbd.c | left out | not translated yet |
| `printf("system_reset_request\n"); exit(1);` | pckbd.c | exported | Unvalidated |
| `ioport_set_a20` | pckbd.c | left out | not translated yet |
| `ioport_get_a20` | pckbd.c | left out | not translated yet |
| `return 1;` | pckbd.c | exported | Unvalidated |
| `kbd_update_irq` | pckbd.c | left out | not translated yet |
| `int irq_kbd_level, irq_mouse_level; irq_kbd_level = 0; irq_mouse_level = 0; s->status &= ~(0x01…` | pckbd.c | exported | Unvalidated |
| `if (s->pending)` | pckbd.c | exported | Unvalidated |
| `set_irq(s->irq_kbd, irq_kbd_level); set_irq(s->irq_mouse, irq_mouse_level);` | pckbd.c | exported | Unvalidated |
| `kbd_update_kbd_irq` | pckbd.c | left out | not translated yet |
| `if (level) … else` | pckbd.c | exported | Unvalidated |
| `kbd_update_irq(s);` | pckbd.c | exported | Unvalidated |
| `kbd_update_aux_irq` | pckbd.c | left out | not translated yet |
| `if (level) … else` | pckbd.c | exported | Unvalidated |
| `kbd_update_irq(s);` | pckbd.c | exported | Unvalidated |
| `kbd_read_status` | pckbd.c | left out | not translated yet |
| `KBDState *s = opaque; int val; val = s->status; return val;` | pckbd.c | exported | Unvalidated |
| `kbd_queue` | pckbd.c | left out | not translated yet |
| `if (aux) … else` | pckbd.c | exported | Unvalidated |
| `kbd_write_command` | pckbd.c | left out | not translated yet |
| `switch (val)` | pckbd.c | exported | Unvalidated |
| `kbd_read_data` | pckbd.c | left out | not translated yet |
| `if (s->pending == 2) … else` | pckbd.c | exported | Unvalidated |
| `return val;` | pckbd.c | exported | Unvalidated |
| `kbd_write_data` | pckbd.c | left out | not translated yet |
| `switch (s->write_cmd)` | pckbd.c | exported | Unvalidated |
| `s->write_cmd = 0;` | pckbd.c | exported | Unvalidated |
| `kbd_reset` | pckbd.c | left out | not translated yet |
| `KBDState *s = opaque; s->mode = 0x01 | 0x02; s->status = 0x08 | 0x10;` | pckbd.c | exported | Unvalidated |
| `i8042_init` | pckbd.c | left out | not translated yet |
| `KBDState *s; s = mallocz(sizeof(*s)); s->irq_kbd = kbd_irq; s->irq_mouse = mouse_irq; kbd_reset…` | pckbd.c | exported | Unvalidated |
| `ps2_queue` | ps2.c | left out | not translated yet |
| `if (q->count >= 256)` | ps2.c | exported | Unvalidated |
| `q->data[q->wptr] = b;` | ps2.c | exported | Unvalidated |
| `if (++q->wptr == 256)` | ps2.c | exported | Unvalidated |
| `q->count++; s->update_irq(s->update_arg, 1);` | ps2.c | exported | Unvalidated |
| `ps2_put_keycode` | ps2.c | left out | not translated yet |
| `if (keycode >= 96)` | ps2.c | exported | Unvalidated |
| `ps2_queue(&s->common, keycode | ((!is_down) << 7));` | ps2.c | exported | Unvalidated |
| `ps2_read_data` | ps2.c | left out | not translated yet |
| `PS2State *s = (PS2State *)opaque; PS2Queue *q; int val, index; q = &s->queue;` | ps2.c | exported | Unvalidated |
| `if (q->count == 0) … else` | ps2.c | exported | Unvalidated |
| `return val;` | ps2.c | exported | Unvalidated |
| `ps2_reset_keyboard` | ps2.c | left out | not translated yet |
| `s->scan_enabled = 1;` | ps2.c | exported | Unvalidated |
| `ps2_write_keyboard` | ps2.c | left out | not translated yet |
| `switch (s->common.write_cmd)` | ps2.c | exported | Unvalidated |
| `ps2_keyboard_set_translation` | ps2.c | left out | not translated yet |
| `PS2KbdState *s = (PS2KbdState *)opaque; s->translate = mode;` | ps2.c | exported | Unvalidated |
| `ps2_mouse_send_packet` | ps2.c | left out | not translated yet |
| `unsigned int b; int dx1, dy1, dz1; dx1 = s->mouse_dx; dy1 = s->mouse_dy; dz1 = s->mouse_dz;` | ps2.c | exported | Unvalidated |
| `if (dx1 > 127) … else` | ps2.c | exported | Unvalidated |
| `if (dy1 > 127) … else` | ps2.c | exported | Unvalidated |
| `b = 0x08 | ((dx1 < 0) << 4) | ((dy1 < 0) << 5) | (s->mouse_buttons & 0x07); ps2_queue(&s->commo…` | ps2.c | exported | Unvalidated |
| `switch (s->mouse_type)` | ps2.c | exported | Unvalidated |
| `s->mouse_dx -= dx1; s->mouse_dy -= dy1; s->mouse_dz -= dz1;` | ps2.c | exported | Unvalidated |
| `ps2_mouse_event` | ps2.c | left out | not translated yet |
| `if (!(s->mouse_status & 0x20)` | ps2.c | exported | Unvalidated |
| `s->mouse_dx += dx; s->mouse_dy -= dy; s->mouse_dz += dz;` | ps2.c | exported | Unvalidated |
| `if (s->mouse_dx == 0 && s->mouse_dy == 0 && s->mouse_dz == 0 && s->mouse_buttons == buttons_sta…` | ps2.c | exported | Unvalidated |
| `s->mouse_buttons = buttons_state;` | ps2.c | exported | Unvalidated |
| `if (!(s->mouse_status & 0x40) && (s->common.queue.count < (256 - 16)` | ps2.c | exported | Unvalidated |
| `ps2_write_mouse` | ps2.c | left out | not translated yet |
| `switch (s->common.write_cmd)` | ps2.c | exported | Unvalidated |
| `ps2_reset` | ps2.c | left out | not translated yet |
| `PS2State *s = (PS2State *)opaque; PS2Queue *q; s->write_cmd = -1; q = &s->queue; q->rptr = 0; q…` | ps2.c | exported | Unvalidated |
| `ps2_kbd_init` | ps2.c | left out | not translated yet |
| `PS2KbdState *s = (PS2KbdState *)mallocz(sizeof(PS2KbdState)); s->common.update_irq = update_irq…` | ps2.c | exported | Unvalidated |
| `ps2_mouse_init` | ps2.c | left out | not translated yet |
| `PS2MouseState *s = (PS2MouseState *)mallocz(sizeof(PS2MouseState)); s->common.update_irq = upda…` | ps2.c | exported | Unvalidated |
| `log_vprintf` | riscv_cpu.c | left out | translation failed |
| `vprintf(fmt, ap);` | riscv_cpu.c | left out | translation failed |
| `fprint_target_ulong` | riscv_cpu.c | left out | translation failed |
| `fprintf(f, "%016" "l" "x" "%016" "l" "x" , (uint64_t)(a >> 64), (uint64_t)a);` | riscv_cpu.c | left out | translation failed |
| `print_target_ulong` | riscv_cpu.c | left out | translation failed |
| `fprint_target_ulong( stdout , a);` | riscv_cpu.c | left out | translation failed |
| `dump_regs` | riscv_cpu.c | left out | translation failed |
| `int i, cols; const char priv_str[4] = "USHM"; cols = 256 / 128; printf("pc ="); print_target_ul…` | riscv_cpu.c | left out | translation failed |
| `for (i = 1; i < 32; i++)` | riscv_cpu.c | left out | translation failed |
| `printf("priv=%c", priv_str[s->priv]); printf(" mstatus="); print_target_ulong(s->mstatus); prin…` | riscv_cpu.c | left out | translation failed |
| `cpu_abort` | riscv_cpu.c | left out | translation failed |
| `dump_regs(s); abort();` | riscv_cpu.c | left out | translation failed |
| `phys_write_u8` | riscv_cpu.c | left out | translation failed |
| `if (!pr || !pr->is_ram)` | riscv_cpu.c | left out | translation failed |
| `*(uint8_t *)(pr->phys_mem + (uintptr_t)(addr - pr->addr)) = val;` | riscv_cpu.c | left out | translation failed |
| `phys_read_u8` | riscv_cpu.c | left out | translation failed |
| `if (!pr || !pr->is_ram)` | riscv_cpu.c | left out | translation failed |
| `return *(uint8_t *)(pr->phys_mem + (uintptr_t)(addr - pr->addr));` | riscv_cpu.c | left out | translation failed |
| `phys_write_u32` | riscv_cpu.c | left out | translation failed |
| `if (!pr || !pr->is_ram)` | riscv_cpu.c | left out | translation failed |
| `*(uint32_t *)(pr->phys_mem + (uintptr_t)(addr - pr->addr)) = val;` | riscv_cpu.c | left out | translation failed |
| `phys_read_u32` | riscv_cpu.c | left out | translation failed |
| `if (!pr || !pr->is_ram)` | riscv_cpu.c | left out | translation failed |
| `return *(uint32_t *)(pr->phys_mem + (uintptr_t)(addr - pr->addr));` | riscv_cpu.c | left out | translation failed |
| `phys_write_u64` | riscv_cpu.c | left out | translation failed |
| `if (!pr || !pr->is_ram)` | riscv_cpu.c | left out | translation failed |
| `*(uint64_t *)(pr->phys_mem + (uintptr_t)(addr - pr->addr)) = val;` | riscv_cpu.c | left out | translation failed |
| `phys_read_u64` | riscv_cpu.c | left out | translation failed |
| `if (!pr || !pr->is_ram)` | riscv_cpu.c | left out | translation failed |
| `return *(uint64_t *)(pr->phys_mem + (uintptr_t)(addr - pr->addr));` | riscv_cpu.c | left out | translation failed |
| `get_phys_addr` | riscv_cpu.c | left out | translation failed |
| `if (s->mstatus & (1 << 17)) && access != 2) … else` | riscv_cpu.c | left out | translation failed |
| `if (priv == 3)` | riscv_cpu.c | left out | translation failed |
| `mode = (s->satp >> 60) & 0xf;` | riscv_cpu.c | left out | translation failed |
| `if (mode == 0) … else` | riscv_cpu.c | left out | translation failed |
| `pte_addr = (s->satp & (((target_ulong)1 << pte_addr_bits) - 1)) << 12; pte_bits = 12 - pte_size…` | riscv_cpu.c | left out | translation failed |
| `for (i = 0; i < levels; i++)` | riscv_cpu.c | left out | translation failed |
| `return -1;` | riscv_cpu.c | left out | translation failed |
| `riscv128_read_slow` | riscv_cpu.c | left out | translation failed |
| `int size, tlb_idx, err, al; target_ulong paddr, offset; uint8_t *ptr; PhysMemoryRange *pr; mem_…` | riscv_cpu.c | left out | translation failed |
| `if (al != 0) … else` | riscv_cpu.c | left out | translation failed |
| `*pval = ret; return 0;` | riscv_cpu.c | left out | translation failed |
| `riscv128_write_slow` | riscv_cpu.c | left out | translation failed |
| `int size, i, tlb_idx, err; target_ulong paddr, offset; uint8_t *ptr; PhysMemoryRange *pr; size…` | riscv_cpu.c | left out | translation failed |
| `if (addr & (size - 1)) != 0) … else` | riscv_cpu.c | left out | translation failed |
| `return 0;` | riscv_cpu.c | left out | translation failed |
| `get_insn32` | riscv_cpu.c | left out | translation failed |
| `return ((struct unaligned_u32 *)ptr)->u32;` | riscv_cpu.c | left out | translation failed |
| `target_read_insn_slow` | riscv_cpu.c | left out | translation failed |
| `if (get_phys_addr(s, &paddr, addr, 2))` | riscv_cpu.c | left out | translation failed |
| `pr = get_phys_mem_range(s->mem_map, paddr);` | riscv_cpu.c | left out | translation failed |
| `if (!pr || !pr->is_ram)` | riscv_cpu.c | left out | translation failed |
| `tlb_idx = (addr >> 12) & (256 - 1); ptr = pr->phys_mem + (uintptr_t)(paddr - pr->addr); s->tlb_…` | riscv_cpu.c | left out | translation failed |
| `target_read_insn_u16` | riscv_cpu.c | left out | translation failed |
| `uint32_t tlb_idx; uint8_t *ptr; tlb_idx = (addr >> 12) & (256 - 1);` | riscv_cpu.c | left out | translation failed |
| `if (__builtin_expect(!!(s->tlb_code[tlb_idx].vaddr == (addr & ~((1 << 12) - 1))), 1)) … else` | riscv_cpu.c | left out | translation failed |
| `*pinsn = *(uint16_t *)ptr; return 0;` | riscv_cpu.c | left out | translation failed |
| `tlb_init` | riscv_cpu.c | left out | translation failed |
| `for (i = 0; i < 256; i++)` | riscv_cpu.c | left out | translation failed |
| `tlb_flush_all` | riscv_cpu.c | left out | translation failed |
| `tlb_init(s);` | riscv_cpu.c | left out | translation failed |
| `tlb_flush_vaddr` | riscv_cpu.c | left out | translation failed |
| `tlb_flush_all(s);` | riscv_cpu.c | left out | translation failed |
| `riscv_cpu_flush_tlb_write_range_ram128` | riscv_cpu.c | left out | translation failed |
| `uint8_t *ptr, *ram_end; int i; ram_end = ram_ptr + ram_size;` | riscv_cpu.c | left out | translation failed |
| `for (i = 0; i < 256; i++)` | riscv_cpu.c | left out | translation failed |
| `get_mstatus` | riscv_cpu.c | left out | translation failed |
| `target_ulong val; BOOL sd; val = s->mstatus | (s->fs << 13); val &= mask; sd = ((val & (3 << 13…` | riscv_cpu.c | left out | translation failed |
| `if (sd)` | riscv_cpu.c | left out | translation failed |
| `return val;` | riscv_cpu.c | left out | translation failed |
| `get_base_from_xlen` | riscv_cpu.c | left out | translation failed |
| `if (xlen == 32) … else` | riscv_cpu.c | left out | translation failed |
| `set_mstatus` | riscv_cpu.c | left out | translation failed |
| `target_ulong mod, mask; mod = s->mstatus ^ val;` | riscv_cpu.c | left out | translation failed |
| `if (mod & ((1 << 17) | (1 << 18) | (1 << 19))) != 0 || ((s->mstatus & (1 << 17)) && (mod & (3 <…` | riscv_cpu.c | left out | translation failed |
| `s->fs = (val >> 13) & 3; mask = ((1 << 0) | (1 << 1) | (1 << 3) | (1 << 4) | (1 << 5) | (1 << 7…` | riscv_cpu.c | left out | translation failed |
| `{ … }` | riscv_cpu.c | left out | translation failed |
| `s->mstatus = (s->mstatus & ~mask) | (val & mask);` | riscv_cpu.c | left out | translation failed |
| `csr_read` | riscv_cpu.c | left out | translation failed |
| `if (csr & 0xc00) == 0xc00) && will_write)` | riscv_cpu.c | left out | translation failed |
| `if (s->priv < ((csr >> 8) & 3)` | riscv_cpu.c | left out | translation failed |
| `switch (csr)` | riscv_cpu.c | left out | translation failed |
| `*pval = val; return 0;` | riscv_cpu.c | left out | translation failed |
| `set_frm` | riscv_cpu.c | left out | translation failed |
| `if (val >= 5)` | riscv_cpu.c | left out | translation failed |
| `s->frm = val;` | riscv_cpu.c | left out | translation failed |
| `get_insn_rm` | riscv_cpu.c | left out | translation failed |
| `if (rm == 7)` | riscv_cpu.c | left out | translation failed |
| `if (rm >= 5) … else` | riscv_cpu.c | left out | translation failed |
| `csr_write` | riscv_cpu.c | left out | translation failed |
| `switch (csr)` | riscv_cpu.c | left out | translation failed |
| `return 0;` | riscv_cpu.c | left out | translation failed |
| `set_priv` | riscv_cpu.c | left out | translation failed |
| `if (s->priv != priv)` | riscv_cpu.c | left out | translation failed |
| `raise_exception2` | riscv_cpu.c | left out | translation failed |
| `if (s->priv <= 1) … else` | riscv_cpu.c | left out | translation failed |
| `causel = cause & 0x7fffffff;` | riscv_cpu.c | left out | translation failed |
| `if (cause & ((uint32_t)1 << 31)` | riscv_cpu.c | left out | translation failed |
| `if (deleg) … else` | riscv_cpu.c | left out | translation failed |
| `raise_exception` | riscv_cpu.c | left out | translation failed |
| `raise_exception2(s, cause, 0);` | riscv_cpu.c | left out | translation failed |
| `handle_sret` | riscv_cpu.c | left out | translation failed |
| `int spp, spie; spp = (s->mstatus >> 8) & 1; spie = (s->mstatus >> 5) & 1; s->mstatus = (s->msta…` | riscv_cpu.c | left out | translation failed |
| `handle_mret` | riscv_cpu.c | left out | translation failed |
| `int mpp, mpie; mpp = (s->mstatus >> 11) & 3; mpie = (s->mstatus >> 7) & 1; s->mstatus = (s->mst…` | riscv_cpu.c | left out | translation failed |
| `get_pending_irq_mask` | riscv_cpu.c | left out | translation failed |
| `uint32_t pending_ints, enabled_ints; pending_ints = s->mip & s->mie;` | riscv_cpu.c | left out | translation failed |
| `if (pending_ints == 0)` | riscv_cpu.c | left out | translation failed |
| `enabled_ints = 0;` | riscv_cpu.c | left out | translation failed |
| `switch (s->priv)` | riscv_cpu.c | left out | translation failed |
| `return pending_ints & enabled_ints;` | riscv_cpu.c | left out | translation failed |
| `raise_interrupt` | riscv_cpu.c | left out | translation failed |
| `uint32_t mask; int irq_num; mask = get_pending_irq_mask(s);` | riscv_cpu.c | left out | translation failed |
| `if (mask == 0)` | riscv_cpu.c | left out | translation failed |
| `irq_num = ctz32(mask); raise_exception(s, irq_num | ((uint32_t)1 << 31)); return -1;` | riscv_cpu.c | left out | translation failed |
| `sext` | riscv_cpu.c | left out | translation failed |
| `return (val << (32 - n)) >> (32 - n);` | riscv_cpu.c | left out | translation failed |
| `get_field1` | riscv_cpu.c | left out | translation failed |
| `int mask; ((void) sizeof (__assert_single_arg ( dst_pos_max >= dst_pos )), __extension__ ({ if…` | riscv_cpu.c | left out | translation failed |
| `if (dst_pos >= src_pos) … else` | riscv_cpu.c | left out | translation failed |
| `riscv_cpu_interp128` | riscv_cpu.c | left out | translation failed |
| `uint64_t timeout; timeout = s->insn_counter + n_cycles;` | riscv_cpu.c | left out | translation failed |
| `while (!s->power_down_flag && (int)(timeout - s->insn_counter) > 0)` | riscv_cpu.c | left out | translation failed |
| `riscv_cpu_get_cycles128` | riscv_cpu.c | left out | translation failed |
| `return s->insn_counter;` | riscv_cpu.c | left out | translation failed |
| `riscv_cpu_set_mip128` | riscv_cpu.c | left out | translation failed |
| `s->mip |= mask;` | riscv_cpu.c | left out | translation failed |
| `if (s->power_down_flag && (s->mip & s->mie) != 0)` | riscv_cpu.c | left out | translation failed |
| `riscv_cpu_reset_mip128` | riscv_cpu.c | left out | translation failed |
| `s->mip &= ~mask;` | riscv_cpu.c | left out | translation failed |
| `riscv_cpu_get_mip128` | riscv_cpu.c | left out | translation failed |
| `return s->mip;` | riscv_cpu.c | left out | translation failed |
| `riscv_cpu_get_power_down128` | riscv_cpu.c | left out | translation failed |
| `return s->power_down_flag;` | riscv_cpu.c | left out | translation failed |
| `riscv_cpu_init128` | riscv_cpu.c | left out | translation failed |
| `RISCVCPUState *s; s = mallocz(sizeof(*s)); s->common.class_ptr = &riscv_cpu_class128; s->mem_ma…` | riscv_cpu.c | left out | translation failed |
| `riscv_cpu_end128` | riscv_cpu.c | left out | translation failed |
| `riscv_cpu_get_misa128` | riscv_cpu.c | left out | translation failed |
| `return s->misa;` | riscv_cpu.c | left out | translation failed |
| `riscv_cpu_init` | riscv_cpu.c | left out | translation failed |
| `switch (max_xlen)` | riscv_cpu.c | left out | translation failed |
| `return c->riscv_cpu_init(mem_map);` | riscv_cpu.c | left out | translation failed |
| `riscv_cpu_end` | riscv_cpu.h | left out | translation failed |
| `const RISCVCPUClass *c = ((RISCVCPUCommonState *)s)->class_ptr; c->riscv_cpu_end(s);` | riscv_cpu.h | left out | translation failed |
| `riscv_cpu_interp` | riscv_cpu.h | left out | translation failed |
| `const RISCVCPUClass *c = ((RISCVCPUCommonState *)s)->class_ptr; c->riscv_cpu_interp(s, n_cycles…` | riscv_cpu.h | left out | translation failed |
| `riscv_cpu_get_cycles` | riscv_cpu.h | left out | translation failed |
| `const RISCVCPUClass *c = ((RISCVCPUCommonState *)s)->class_ptr; return c->riscv_cpu_get_cycles(…` | riscv_cpu.h | left out | translation failed |
| `riscv_cpu_set_mip` | riscv_cpu.h | left out | translation failed |
| `const RISCVCPUClass *c = ((RISCVCPUCommonState *)s)->class_ptr; c->riscv_cpu_set_mip(s, mask);` | riscv_cpu.h | left out | translation failed |
| `riscv_cpu_reset_mip` | riscv_cpu.h | left out | translation failed |
| `const RISCVCPUClass *c = ((RISCVCPUCommonState *)s)->class_ptr; c->riscv_cpu_reset_mip(s, mask);` | riscv_cpu.h | left out | translation failed |
| `riscv_cpu_get_mip` | riscv_cpu.h | left out | translation failed |
| `const RISCVCPUClass *c = ((RISCVCPUCommonState *)s)->class_ptr; return c->riscv_cpu_get_mip(s);` | riscv_cpu.h | left out | translation failed |
| `riscv_cpu_get_power_down` | riscv_cpu.h | left out | translation failed |
| `const RISCVCPUClass *c = ((RISCVCPUCommonState *)s)->class_ptr; return c->riscv_cpu_get_power_d…` | riscv_cpu.h | left out | translation failed |
| `riscv_cpu_get_misa` | riscv_cpu.h | left out | translation failed |
| `const RISCVCPUClass *c = ((RISCVCPUCommonState *)s)->class_ptr; return c->riscv_cpu_get_misa(s);` | riscv_cpu.h | left out | translation failed |
| `riscv_cpu_flush_tlb_write_range_ram` | riscv_cpu.h | left out | translation failed |
| `const RISCVCPUClass *c = ((RISCVCPUCommonState *)s)->class_ptr; c->riscv_cpu_flush_tlb_write_ra…` | riscv_cpu.h | left out | translation failed |
| `target_read_u8` | riscv_cpu_priv.h | left out | translation failed |
| `uint32_t tlb_idx; tlb_idx = (addr >> 12) & (256 - 1);` | riscv_cpu_priv.h | left out | translation failed |
| `if (__builtin_expect(!!(s->tlb_read[tlb_idx].vaddr == (addr & ~(((1 << 12) - 1) & ~((8 / 8) - 1… … else` | riscv_cpu_priv.h | left out | translation failed |
| `return 0;` | riscv_cpu_priv.h | left out | translation failed |
| `target_write_u8` | riscv_cpu_priv.h | left out | translation failed |
| `uint32_t tlb_idx; tlb_idx = (addr >> 12) & (256 - 1);` | riscv_cpu_priv.h | left out | translation failed |
| `if (__builtin_expect(!!(s->tlb_write[tlb_idx].vaddr == (addr & ~(((1 << 12) - 1) & ~((8 / 8) -… … else` | riscv_cpu_priv.h | left out | translation failed |
| `target_read_u16` | riscv_cpu_priv.h | left out | translation failed |
| `uint32_t tlb_idx; tlb_idx = (addr >> 12) & (256 - 1);` | riscv_cpu_priv.h | left out | translation failed |
| `if (__builtin_expect(!!(s->tlb_read[tlb_idx].vaddr == (addr & ~(((1 << 12) - 1) & ~((16 / 8) -… … else` | riscv_cpu_priv.h | left out | translation failed |
| `return 0;` | riscv_cpu_priv.h | left out | translation failed |
| `target_write_u16` | riscv_cpu_priv.h | left out | translation failed |
| `uint32_t tlb_idx; tlb_idx = (addr >> 12) & (256 - 1);` | riscv_cpu_priv.h | left out | translation failed |
| `if (__builtin_expect(!!(s->tlb_write[tlb_idx].vaddr == (addr & ~(((1 << 12) - 1) & ~((16 / 8) -… … else` | riscv_cpu_priv.h | left out | translation failed |
| `target_read_u32` | riscv_cpu_priv.h | left out | translation failed |
| `uint32_t tlb_idx; tlb_idx = (addr >> 12) & (256 - 1);` | riscv_cpu_priv.h | left out | translation failed |
| `if (__builtin_expect(!!(s->tlb_read[tlb_idx].vaddr == (addr & ~(((1 << 12) - 1) & ~((32 / 8) -… … else` | riscv_cpu_priv.h | left out | translation failed |
| `return 0;` | riscv_cpu_priv.h | left out | translation failed |
| `target_write_u32` | riscv_cpu_priv.h | left out | translation failed |
| `uint32_t tlb_idx; tlb_idx = (addr >> 12) & (256 - 1);` | riscv_cpu_priv.h | left out | translation failed |
| `if (__builtin_expect(!!(s->tlb_write[tlb_idx].vaddr == (addr & ~(((1 << 12) - 1) & ~((32 / 8) -… … else` | riscv_cpu_priv.h | left out | translation failed |
| `target_read_u64` | riscv_cpu_priv.h | left out | translation failed |
| `uint32_t tlb_idx; tlb_idx = (addr >> 12) & (256 - 1);` | riscv_cpu_priv.h | left out | translation failed |
| `if (__builtin_expect(!!(s->tlb_read[tlb_idx].vaddr == (addr & ~(((1 << 12) - 1) & ~((64 / 8) -… … else` | riscv_cpu_priv.h | left out | translation failed |
| `return 0;` | riscv_cpu_priv.h | left out | translation failed |
| `target_write_u64` | riscv_cpu_priv.h | left out | translation failed |
| `uint32_t tlb_idx; tlb_idx = (addr >> 12) & (256 - 1);` | riscv_cpu_priv.h | left out | translation failed |
| `if (__builtin_expect(!!(s->tlb_write[tlb_idx].vaddr == (addr & ~(((1 << 12) - 1) & ~((64 / 8) -… … else` | riscv_cpu_priv.h | left out | translation failed |
| `target_read_u128` | riscv_cpu_priv.h | left out | translation failed |
| `uint32_t tlb_idx; tlb_idx = (addr >> 12) & (256 - 1);` | riscv_cpu_priv.h | left out | translation failed |
| `if (__builtin_expect(!!(s->tlb_read[tlb_idx].vaddr == (addr & ~(((1 << 12) - 1) & ~((128 / 8) -… … else` | riscv_cpu_priv.h | left out | translation failed |
| `return 0;` | riscv_cpu_priv.h | left out | translation failed |
| `target_write_u128` | riscv_cpu_priv.h | left out | translation failed |
| `uint32_t tlb_idx; tlb_idx = (addr >> 12) & (256 - 1);` | riscv_cpu_priv.h | left out | translation failed |
| `if (__builtin_expect(!!(s->tlb_write[tlb_idx].vaddr == (addr & ~(((1 << 12) - 1) & ~((128 / 8)… … else` | riscv_cpu_priv.h | left out | translation failed |
| `div32` | riscv_cpu_template.h | left out | translation failed |
| `div64` | riscv_cpu_template.h | left out | translation failed |
| `div128` | riscv_cpu_template.h | left out | translation failed |
| `if (b == 0) … else` | riscv_cpu_template.h | left out | translation failed |
| `if (b == 0) … else` | riscv_cpu_template.h | left out | translation failed |
| `if (b == 0) … else` | riscv_cpu_template.h | left out | translation failed |
| `divu32` | riscv_cpu_template.h | left out | translation failed |
| `divu64` | riscv_cpu_template.h | left out | translation failed |
| `divu128` | riscv_cpu_template.h | left out | translation failed |
| `if (b == 0) … else` | riscv_cpu_template.h | left out | translation failed |
| `if (b == 0) … else` | riscv_cpu_template.h | left out | translation failed |
| `if (b == 0) … else` | riscv_cpu_template.h | left out | translation failed |
| `rem32` | riscv_cpu_template.h | left out | translation failed |
| `rem64` | riscv_cpu_template.h | left out | translation failed |
| `rem128` | riscv_cpu_template.h | left out | translation failed |
| `if (b == 0) … else` | riscv_cpu_template.h | left out | translation failed |
| `if (b == 0) … else` | riscv_cpu_template.h | left out | translation failed |
| `if (b == 0) … else` | riscv_cpu_template.h | left out | translation failed |
| `remu32` | riscv_cpu_template.h | left out | translation failed |
| `remu64` | riscv_cpu_template.h | left out | translation failed |
| `remu128` | riscv_cpu_template.h | left out | translation failed |
| `if (b == 0) … else` | riscv_cpu_template.h | left out | translation failed |
| `if (b == 0) … else` | riscv_cpu_template.h | left out | translation failed |
| `if (b == 0) … else` | riscv_cpu_template.h | left out | translation failed |
| `mulh32` | riscv_cpu_template.h | left out | translation failed |
| `return ((int64_t)a * (int64_t)b) >> 32;` | riscv_cpu_template.h | left out | translation failed |
| `mulhsu32` | riscv_cpu_template.h | left out | translation failed |
| `return ((int64_t)a * (int64_t)b) >> 32;` | riscv_cpu_template.h | left out | translation failed |
| `mulhu32` | riscv_cpu_template.h | left out | translation failed |
| `return ((int64_t)a * (int64_t)b) >> 32;` | riscv_cpu_template.h | left out | translation failed |
| `mulh64` | riscv_cpu_template.h | left out | translation failed |
| `return ((int128_t)a * (int128_t)b) >> 64;` | riscv_cpu_template.h | left out | translation failed |
| `mulhsu64` | riscv_cpu_template.h | left out | translation failed |
| `return ((int128_t)a * (int128_t)b) >> 64;` | riscv_cpu_template.h | left out | translation failed |
| `mulhu64` | riscv_cpu_template.h | left out | translation failed |
| `return ((int128_t)a * (int128_t)b) >> 64;` | riscv_cpu_template.h | left out | translation failed |
| `mulhu128` | riscv_cpu_template.h | left out | translation failed |
| `uint64_t a0, a1, b0, b1, r2, r3; uint128_t r00, r01, r10, r11, c; a0 = a; a1 = a >> 64; b0 = b;…` | riscv_cpu_template.h | left out | translation failed |
| `mulh128` | riscv_cpu_template.h | left out | translation failed |
| `uint128_t r1; r1 = mulhu128(a, b);` | riscv_cpu_template.h | left out | translation failed |
| `if (a < 0)` | riscv_cpu_template.h | left out | translation failed |
| `if (b < 0)` | riscv_cpu_template.h | left out | translation failed |
| `return r1;` | riscv_cpu_template.h | left out | translation failed |
| `mulhsu128` | riscv_cpu_template.h | left out | translation failed |
| `uint128_t r1; r1 = mulhu128(a, b);` | riscv_cpu_template.h | left out | translation failed |
| `if (a < 0)` | riscv_cpu_template.h | left out | translation failed |
| `return r1;` | riscv_cpu_template.h | left out | translation failed |
| `riscv_cpu_interp_x32` | riscv_cpu_template.h | left out | translation failed |
| `riscv_cpu_interp_x64` | riscv_cpu_template.h | left out | translation failed |
| `riscv_cpu_interp_x128` | riscv_cpu_template.h | left out | translation failed |
| `if (n_cycles1 == 0)` | riscv_cpu_template.h | left out | translation failed |
| `if (n_cycles1 == 0)` | riscv_cpu_template.h | left out | translation failed |
| `if (n_cycles1 == 0)` | riscv_cpu_template.h | left out | translation failed |
| `insn_counter_addend = s->insn_counter + n_cycles1; s->n_cycles = n_cycles1;` | riscv_cpu_template.h | left out | translation failed |
| `insn_counter_addend = s->insn_counter + n_cycles1; s->n_cycles = n_cycles1;` | riscv_cpu_template.h | left out | translation failed |
| `insn_counter_addend = s->insn_counter + n_cycles1; s->n_cycles = n_cycles1;` | riscv_cpu_template.h | left out | translation failed |
| `if (__builtin_expect(!!((s->mip & s->mie) != 0), 0))` | riscv_cpu_template.h | left out | translation failed |
| `if (__builtin_expect(!!((s->mip & s->mie) != 0), 0))` | riscv_cpu_template.h | left out | translation failed |
| `if (__builtin_expect(!!((s->mip & s->mie) != 0), 0))` | riscv_cpu_template.h | left out | translation failed |
| `s->pending_exception = -1; code_ptr = ((void *)0) ; code_end = ((void *)0) ; code_to_pc_addend…` | riscv_cpu_template.h | left out | translation failed |
| `s->pending_exception = -1; code_ptr = ((void *)0) ; code_end = ((void *)0) ; code_to_pc_addend…` | riscv_cpu_template.h | left out | translation failed |
| `s->pending_exception = -1; code_ptr = ((void *)0) ; code_end = ((void *)0) ; code_to_pc_addend…` | riscv_cpu_template.h | left out | translation failed |
| `for (; ; )` | riscv_cpu_template.h | left out | translation failed |
| `for (; ; )` | riscv_cpu_template.h | left out | translation failed |
| `for (; ; )` | riscv_cpu_template.h | left out | translation failed |
| `illegal_insn: s->pending_exception = 0x2;` | riscv_cpu_template.h | left out | translation failed |
| `illegal_insn: s->pending_exception = 0x2;` | riscv_cpu_template.h | left out | translation failed |
| `illegal_insn: s->pending_exception = 0x2;` | riscv_cpu_template.h | left out | translation failed |
| `s->pending_tval = insn;` | riscv_cpu_template.h | left out | translation failed |
| `s->pending_tval = insn;` | riscv_cpu_template.h | left out | translation failed |
| `s->pending_tval = insn;` | riscv_cpu_template.h | left out | translation failed |
| `mmu_exception: exception: s->pc = (target_ulong)((uintptr_t)code_ptr + code_to_pc_addend);` | riscv_cpu_template.h | left out | translation failed |
| `mmu_exception: exception: s->pc = (target_ulong)((uintptr_t)code_ptr + code_to_pc_addend);` | riscv_cpu_template.h | left out | translation failed |
| `mmu_exception: exception: s->pc = (target_ulong)((uintptr_t)code_ptr + code_to_pc_addend);` | riscv_cpu_template.h | left out | translation failed |
| `if (s->pending_exception >= 0)` | riscv_cpu_template.h | left out | translation failed |
| `if (s->pending_exception >= 0)` | riscv_cpu_template.h | left out | translation failed |
| `if (s->pending_exception >= 0)` | riscv_cpu_template.h | left out | translation failed |
| `done_interp: the_end: s->insn_counter = (insn_counter_addend - s->n_cycles);` | riscv_cpu_template.h | left out | translation failed |
| `done_interp: the_end: s->insn_counter = (insn_counter_addend - s->n_cycles);` | riscv_cpu_template.h | left out | translation failed |
| `done_interp: the_end: s->insn_counter = (insn_counter_addend - s->n_cycles);` | riscv_cpu_template.h | left out | translation failed |
| `rtc_get_real_time` | riscv_machine.c | left out | not translated yet |
| `struct timespec ts; clock_gettime( 1 , &ts); return (uint64_t)ts.tv_sec * 10000000 + (ts.tv_nse…` | riscv_machine.c | exported | Unvalidated |
| `rtc_get_time` | riscv_machine.c | left out | not translated yet |
| `if (m->rtc_real_time) … else` | riscv_machine.c | exported | Unvalidated |
| `return val;` | riscv_machine.c | exported | Unvalidated |
| `htif_read` | riscv_machine.c | left out | not translated yet |
| `RISCVMachine *s = opaque; uint32_t val; ((void) sizeof (__assert_single_arg ( size_log2 == 2 ))…` | riscv_machine.c | exported | Unvalidated |
| `switch (offset)` | riscv_machine.c | exported | Unvalidated |
| `return val;` | riscv_machine.c | exported | Unvalidated |
| `htif_handle_cmd` | riscv_machine.c | left out | not translated yet |
| `uint32_t device, cmd; device = s->htif_tohost >> 56; cmd = (s->htif_tohost >> 48) & 0xff;` | riscv_machine.c | exported | Unvalidated |
| `if (s->htif_tohost == 1) … else` | riscv_machine.c | exported | Unvalidated |
| `htif_write` | riscv_machine.c | left out | not translated yet |
| `RISCVMachine *s = opaque; ((void) sizeof (__assert_single_arg ( size_log2 == 2 )), __extension_…` | riscv_machine.c | exported | Unvalidated |
| `switch (offset)` | riscv_machine.c | exported | Unvalidated |
| `clint_read` | riscv_machine.c | left out | not translated yet |
| `RISCVMachine *m = opaque; uint32_t val; ((void) sizeof (__assert_single_arg ( size_log2 == 2 ))…` | riscv_machine.c | exported | Unvalidated |
| `switch (offset)` | riscv_machine.c | exported | Unvalidated |
| `return val;` | riscv_machine.c | exported | Unvalidated |
| `clint_write` | riscv_machine.c | left out | not translated yet |
| `RISCVMachine *m = opaque; ((void) sizeof (__assert_single_arg ( size_log2 == 2 )), __extension_…` | riscv_machine.c | exported | Unvalidated |
| `switch (offset)` | riscv_machine.c | exported | Unvalidated |
| `plic_update_mip` | riscv_machine.c | left out | not translated yet |
| `RISCVCPUState *cpu = s->cpu_state; uint32_t mask; mask = s->plic_pending_irq & ~s->plic_served_…` | riscv_machine.c | exported | Unvalidated |
| `if (mask) … else` | riscv_machine.c | exported | Unvalidated |
| `plic_read` | riscv_machine.c | left out | not translated yet |
| `RISCVMachine *s = opaque; uint32_t val, mask; int i; ((void) sizeof (__assert_single_arg ( size…` | riscv_machine.c | exported | Unvalidated |
| `switch (offset)` | riscv_machine.c | exported | Unvalidated |
| `return val;` | riscv_machine.c | exported | Unvalidated |
| `plic_write` | riscv_machine.c | left out | not translated yet |
| `RISCVMachine *s = opaque; ((void) sizeof (__assert_single_arg ( size_log2 == 2 )), __extension_…` | riscv_machine.c | exported | Unvalidated |
| `switch (offset)` | riscv_machine.c | exported | Unvalidated |
| `plic_set_irq` | riscv_machine.c | left out | not translated yet |
| `RISCVMachine *s = opaque; uint32_t mask; mask = 1 << (irq_num - 1);` | riscv_machine.c | exported | Unvalidated |
| `if (state) … else` | riscv_machine.c | exported | Unvalidated |
| `plic_update_mip(s);` | riscv_machine.c | exported | Unvalidated |
| `get_ram_ptr` | riscv_machine.c | left out | not translated yet |
| `return phys_mem_get_ram_ptr(s->mem_map, paddr, is_rw);` | riscv_machine.c | exported | Unvalidated |
| `fdt_init` | riscv_machine.c | left out | not translated yet |
| `FDTState *s; s = mallocz(sizeof(*s)); return s;` | riscv_machine.c | exported | Unvalidated |
| `fdt_alloc_len` | riscv_machine.c | left out | not translated yet |
| `if (__builtin_expect(!!(len > s->tab_size), 0))` | riscv_machine.c | exported | Unvalidated |
| `fdt_put32` | riscv_machine.c | left out | not translated yet |
| `fdt_alloc_len(s, s->tab_len + 1); s->tab[s->tab_len++] = cpu_to_be32(v);` | riscv_machine.c | exported | Unvalidated |
| `fdt_put_data` | riscv_machine.c | left out | not translated yet |
| `int len1; len1 = (len + 3) / 4; fdt_alloc_len(s, s->tab_len + len1); memcpy(s->tab + s->tab_len…` | riscv_machine.c | exported | Unvalidated |
| `fdt_begin_node` | riscv_machine.c | left out | not translated yet |
| `fdt_put32(s, 1); fdt_put_data(s, (uint8_t *)name, strlen(name) + 1); s->open_node_count++;` | riscv_machine.c | exported | Unvalidated |
| `fdt_begin_node_num` | riscv_machine.c | left out | not translated yet |
| `char buf[256]; snprintf(buf, sizeof(buf), "%s@%" "l" "x" , name, n); fdt_begin_node(s, buf);` | riscv_machine.c | exported | Unvalidated |
| `fdt_end_node` | riscv_machine.c | left out | not translated yet |
| `fdt_put32(s, 2); s->open_node_count--;` | riscv_machine.c | exported | Unvalidated |
| `fdt_get_string_offset` | riscv_machine.c | left out | not translated yet |
| `int pos, new_size, name_size, new_len; pos = 0;` | riscv_machine.c | exported | Unvalidated |
| `while (pos < s->string_table_len)` | riscv_machine.c | exported | Unvalidated |
| `name_size = strlen(name) + 1; new_len = s->string_table_len + name_size;` | riscv_machine.c | exported | Unvalidated |
| `if (new_len > s->string_table_size)` | riscv_machine.c | exported | Unvalidated |
| `pos = s->string_table_len; memcpy(s->string_table + pos, name, name_size); s->string_table_len…` | riscv_machine.c | exported | Unvalidated |
| `fdt_prop` | riscv_machine.c | left out | not translated yet |
| `fdt_put32(s, 3); fdt_put32(s, data_len); fdt_put32(s, fdt_get_string_offset(s, prop_name)); fdt…` | riscv_machine.c | exported | Unvalidated |
| `fdt_prop_tab_u32` | riscv_machine.c | left out | not translated yet |
| `int i; fdt_put32(s, 3); fdt_put32(s, tab_len * sizeof(uint32_t)); fdt_put32(s, fdt_get_string_o…` | riscv_machine.c | exported | Unvalidated |
| `for (i = 0; i < tab_len; i++)` | riscv_machine.c | exported | Unvalidated |
| `fdt_prop_u32` | riscv_machine.c | left out | not translated yet |
| `fdt_prop_tab_u32(s, prop_name, &val, 1);` | riscv_machine.c | exported | Unvalidated |
| `fdt_prop_tab_u64` | riscv_machine.c | left out | not translated yet |
| `uint32_t tab[2]; tab[0] = v0 >> 32; tab[1] = v0; fdt_prop_tab_u32(s, prop_name, tab, 2);` | riscv_machine.c | exported | Unvalidated |
| `fdt_prop_tab_u64_2` | riscv_machine.c | left out | not translated yet |
| `uint32_t tab[4]; tab[0] = v0 >> 32; tab[1] = v0; tab[2] = v1 >> 32; tab[3] = v1; fdt_prop_tab_u…` | riscv_machine.c | exported | Unvalidated |
| `fdt_prop_str` | riscv_machine.c | left out | not translated yet |
| `fdt_prop(s, prop_name, str, strlen(str) + 1);` | riscv_machine.c | exported | Unvalidated |
| `fdt_output` | riscv_machine.c | left out | not translated yet |
| `struct fdt_header *h; struct fdt_reserve_entry *re; int dt_struct_size; int dt_strings_size; in…` | riscv_machine.c | exported | Unvalidated |
| `while (pos & 7) != 0)` | riscv_machine.c | exported | Unvalidated |
| `h->off_mem_rsvmap = cpu_to_be32(pos); re = (struct fdt_reserve_entry *)(dst + pos); re->address…` | riscv_machine.c | exported | Unvalidated |
| `while (pos & 7) != 0)` | riscv_machine.c | exported | Unvalidated |
| `h->totalsize = cpu_to_be32(pos); return pos;` | riscv_machine.c | exported | Unvalidated |
| `fdt_end` | riscv_machine.c | left out | not translated yet |
| `free(s->tab); free(s->string_table); free(s);` | riscv_machine.c | exported | Unvalidated |
| `riscv_build_fdt` | riscv_machine.c | left out | not translated yet |
| `FDTState *s; int size, max_xlen, i, cur_phandle, intc_phandle, plic_phandle; char isa_string[12…` | riscv_machine.c | exported | Unvalidated |
| `for (i = 0; i < 26; i++)` | riscv_machine.c | exported | Unvalidated |
| `*q = '\0'; fdt_prop_str(s, "riscv,isa", isa_string); fdt_prop_str(s, "mmu-type", max_xlen <= 32…` | riscv_machine.c | exported | Unvalidated |
| `for (i = 0; i < m->virtio_count; i++)` | riscv_machine.c | exported | Unvalidated |
| `fb_dev = m->common.fb_dev;` | riscv_machine.c | exported | Unvalidated |
| `if (fb_dev)` | riscv_machine.c | exported | Unvalidated |
| `fdt_end_node(s); fdt_begin_node(s, "chosen"); fdt_prop_str(s, "bootargs", cmd_line ? cmd_line :…` | riscv_machine.c | exported | Unvalidated |
| `if (kernel_size > 0)` | riscv_machine.c | exported | Unvalidated |
| `if (initrd_size > 0)` | riscv_machine.c | exported | Unvalidated |
| `fdt_end_node(s); fdt_end_node(s); size = fdt_output(s, dst); fdt_end(s); return size;` | riscv_machine.c | exported | Unvalidated |
| `copy_bios` | riscv_machine.c | left out | not translated yet |
| `uint64_t fdt_addr, kernel_align, kernel_base, kernel_size; uint64_t bios_base, bios_size, initr…` | riscv_machine.c | exported | Unvalidated |
| `if (elf_detect_magic(buf, buf_len)) … else` | riscv_machine.c | exported | Unvalidated |
| `if (kernel_buf_len > 0) … else` | riscv_machine.c | exported | Unvalidated |
| `if (initrd_buf_len > 0) … else` | riscv_machine.c | exported | Unvalidated |
| `ram_ptr = get_ram_ptr(s, 0, TRUE); fdt_addr = 0x1000 + 8 * 8; riscv_build_fdt(s, ram_ptr + fdt_…` | riscv_machine.c | exported | Unvalidated |
| `riscv_flush_tlb_write_range` | riscv_machine.c | left out | not translated yet |
| `RISCVMachine *s = opaque; riscv_cpu_flush_tlb_write_range_ram(s->cpu_state, ram_addr, ram_size);` | riscv_machine.c | exported | Unvalidated |
| `riscv_machine_set_defaults` | riscv_machine.c | left out | not translated yet |
| `riscv_machine_init` | riscv_machine.c | left out | not translated yet |
| `if (!strcmp(p->machine_name, "riscv32")) … else` | riscv_machine.c | exported | Unvalidated |
| `s = mallocz(sizeof(*s)); s->common.vmc = p->vmc; s->ram_size = p->ram_size; s->max_xlen = max_x…` | riscv_machine.c | exported | Unvalidated |
| `if (!s->cpu_state)` | riscv_machine.c | exported | Unvalidated |
| `if (p->files[VM_FILE_BIOS].buf)` | riscv_machine.c | exported | Unvalidated |
| `cpu_register_device(s->mem_map, htif_start, 16, s, htif_read, htif_write, (1 << 2)); ram_flags…` | riscv_machine.c | exported | Unvalidated |
| `if (p->rtc_real_time)` | riscv_machine.c | exported | Unvalidated |
| `cpu_register_device(s->mem_map, 0x02000000, 0x000c0000, s, clint_read, clint_write, (1 << 2));…` | riscv_machine.c | exported | Unvalidated |
| `for (i = 1; i < 32; i++)` | riscv_machine.c | exported | Unvalidated |
| `s->common.console = p->console; memset(vbus, 0, sizeof(*vbus)); vbus->mem_map = s->mem_map; vbu…` | riscv_machine.c | exported | Unvalidated |
| `if (p->console)` | riscv_machine.c | exported | Unvalidated |
| `for (i = 0; i < p->eth_count; i++)` | riscv_machine.c | exported | Unvalidated |
| `for (i = 0; i < p->drive_count; i++)` | riscv_machine.c | exported | Unvalidated |
| `for (i = 0; i < p->fs_count; i++)` | riscv_machine.c | exported | Unvalidated |
| `if (p->display_device)` | riscv_machine.c | exported | Unvalidated |
| `if (p->input_device)` | riscv_machine.c | exported | Unvalidated |
| `if (!p->files[VM_FILE_BIOS].buf)` | riscv_machine.c | exported | Unvalidated |
| `copy_bios(s, p->files[VM_FILE_BIOS].buf, p->files[VM_FILE_BIOS].len, p->files[VM_FILE_KERNEL].b…` | riscv_machine.c | exported | Unvalidated |
| `riscv_machine_end` | riscv_machine.c | left out | not translated yet |
| `RISCVMachine *s = (RISCVMachine *)s1; riscv_cpu_end(s->cpu_state); phys_mem_map_end(s->mem_map)…` | riscv_machine.c | exported | Unvalidated |
| `riscv_machine_get_sleep_duration` | riscv_machine.c | left out | not translated yet |
| `if (!(riscv_cpu_get_mip(s) & (1 << 7)` | riscv_machine.c | exported | Unvalidated |
| `if (!riscv_cpu_get_power_down(s))` | riscv_machine.c | exported | Unvalidated |
| `return delay;` | riscv_machine.c | exported | Unvalidated |
| `riscv_machine_interp` | riscv_machine.c | left out | not translated yet |
| `RISCVMachine *s = (RISCVMachine *)s1; riscv_cpu_interp(s->cpu_state, max_exec_cycle);` | riscv_machine.c | exported | Unvalidated |
| `riscv_vm_send_key_event` | riscv_machine.c | left out | not translated yet |
| `if (s->keyboard_dev)` | riscv_machine.c | exported | Unvalidated |
| `riscv_vm_mouse_is_absolute` | riscv_machine.c | left out | not translated yet |
| `return TRUE;` | riscv_machine.c | exported | Unvalidated |
| `riscv_vm_send_mouse_event` | riscv_machine.c | left out | not translated yet |
| `if (s->mouse_dev)` | riscv_machine.c | exported | Unvalidated |
| `sdl_update_fb_texture` | sdl.c | left out | translation failed |
| `if (!fb_texture || fb_width != fb_dev->width || fb_height != fb_dev->height)` | sdl.c | left out | translation failed |
| `sdl_update` | sdl.c | left out | translation failed |
| `int *dirty = (int *)opaque; *dirty = 1;` | sdl.c | left out | translation failed |
| `sdl_get_keycode` | sdl.c | left out | translation failed |
| `switch (ev->keysym.scancode)` | sdl.c | left out | translation failed |
| `sdl_reset_keys` | sdl.c | left out | translation failed |
| `for (i = 1; i <= 127; i++)` | sdl.c | left out | translation failed |
| `sdl_handle_key_event` | sdl.c | left out | translation failed |
| `int keycode, keypress; keycode = sdl_get_keycode(ev);` | sdl.c | left out | translation failed |
| `if (keycode) … else` | sdl.c | left out | translation failed |
| `sdl_send_mouse_event` | sdl.c | left out | translation failed |
| `int buttons, x, y; buttons = 0;` | sdl.c | left out | translation failed |
| `if (state & (1 << ((1)-1)` | sdl.c | left out | translation failed |
| `if (state & (1 << ((3)-1)` | sdl.c | left out | translation failed |
| `if (state & (1 << ((2)-1)` | sdl.c | left out | translation failed |
| `if (is_absolute) … else` | sdl.c | left out | translation failed |
| `vm_send_mouse_event(m, x, y, dz, buttons);` | sdl.c | left out | translation failed |
| `sdl_handle_mouse_motion_event` | sdl.c | left out | translation failed |
| `if (is_absolute) … else` | sdl.c | left out | translation failed |
| `sdl_send_mouse_event(m, x, y, 0, ev->motion.state, is_absolute);` | sdl.c | left out | translation failed |
| `sdl_handle_mouse_button_event` | sdl.c | left out | translation failed |
| `BOOL is_absolute = vm_mouse_is_absolute(m); int state, dz; dz = 0;` | sdl.c | left out | translation failed |
| `if (ev->type == SDL_MOUSEWHEEL)` | sdl.c | left out | translation failed |
| `state = SDL_GetMouseState( ((void *)0) , ((void *)0) );` | sdl.c | left out | translation failed |
| `if (ev->type == SDL_MOUSEBUTTONDOWN) … else` | sdl.c | left out | translation failed |
| `if (is_absolute) … else` | sdl.c | left out | translation failed |
| `sdl_refresh` | sdl.c | left out | translation failed |
| `if (!m->fb_dev)` | sdl.c | left out | translation failed |
| `sdl_update_fb_texture(m->fb_dev); int dirty = 0; m->fb_dev->refresh(m->fb_dev, sdl_update, &dir…` | sdl.c | left out | translation failed |
| `if (dirty)` | sdl.c | left out | translation failed |
| `while (SDL_PollEvent(ev))` | sdl.c | left out | translation failed |
| `sdl_hide_cursor` | sdl.c | left out | translation failed |
| `uint8_t data = 0; sdl_cursor_hidden = SDL_CreateCursor(&data, &data, 8, 1, 0, 0); SDL_ShowCurso…` | sdl.c | left out | translation failed |
| `sdl_init` | sdl.c | left out | translation failed |
| `window_width = width; window_height = height;` | sdl.c | left out | translation failed |
| `if (SDL_Init( 0x00000020u | 0x00100000u ))` | sdl.c | left out | translation failed |
| `if (result == -1)` | sdl.c | left out | translation failed |
| `SDL_SetWindowTitle(window, "TinyEMU"); sdl_hide_cursor();` | sdl.c | left out | translation failed |
| `sha256_compress` | sha256.c | left out | not translated yet |
| `for (i = 0; i < 8; i++)` | sha256.c | exported | Unvalidated |
| `for (i = 0; i < 16; i++)` | sha256.c | exported | Unvalidated |
| `for (i = 16; i < 64; i++)` | sha256.c | exported | Unvalidated |
| `t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))…` | sha256.c | exported | Unvalidated |
| `for (i = 0; i < 8; i++)` | sha256.c | exported | Unvalidated |
| `SHA256_Init` | sha256.c | left out | not translated yet |
| `s->curlen = 0; s->length = 0; s->state[0] = 0x6A09E667UL; s->state[1] = 0xBB67AE85UL; s->state[…` | sha256.c | exported | Unvalidated |
| `SHA256_Update` | sha256.c | left out | not translated yet |
| `if (s->curlen > sizeof(s->buf)` | sha256.c | exported | Unvalidated |
| `if (s->length + inlen) < s->length)` | sha256.c | exported | Unvalidated |
| `while (inlen > 0)` | sha256.c | exported | Unvalidated |
| `SHA256_Final` | sha256.c | left out | not translated yet |
| `if (s->curlen >= sizeof(s->buf)` | sha256.c | exported | Unvalidated |
| `s->length += s->curlen * 8; s->buf[s->curlen++] = (unsigned char)0x80;` | sha256.c | exported | Unvalidated |
| `if (s->curlen > 56)` | sha256.c | exported | Unvalidated |
| `while (s->curlen < 56)` | sha256.c | exported | Unvalidated |
| `put_be64(s->buf+56, s->length); sha256_compress(s, s->buf);` | sha256.c | exported | Unvalidated |
| `for (i = 0; i < 8; i++)` | sha256.c | exported | Unvalidated |
| `SHA256` | sha256.c | left out | not translated yet |
| `SHA256_CTX ctx; SHA256_Init(&ctx); SHA256_Update(&ctx, buf, buf_len); SHA256_Final(out, &ctx);` | sha256.c | exported | Unvalidated |
| `simplefb_refresh` | simplefb.c | left out | not translated yet |
| `const uint32_t *dirty_bits; uint32_t dirty_val; int y0, y1, page_y0, page_y1, byte_pos, page_in…` | simplefb.c | exported | Unvalidated |
| `while (page_index < fb_page_count)` | simplefb.c | exported | Unvalidated |
| `if (y0 != y1)` | simplefb.c | exported | Unvalidated |
| `simplefb_refresh1` | simplefb.c | left out | not translated yet |
| `SimpleFBState *s = fb_dev->device_opaque; simplefb_refresh(fb_dev, redraw_func, opaque, s->mem_…` | simplefb.c | exported | Unvalidated |
| `simplefb_init` | simplefb.c | left out | not translated yet |
| `SimpleFBState *s; s = mallocz(sizeof(*s)); s->fb_dev = fb_dev; fb_dev->width = width; fb_dev->h…` | simplefb.c | exported | Unvalidated |
| `get_new_addr` | slirp/bootp.c | left out | not translated yet |
| `for (i = 0; i < 16; i++)` | slirp/bootp.c | exported | Unvalidated |
| `return ((void *)0) ;` | slirp/bootp.c | exported | Unvalidated |
| `found: bc = &slirp->bootp_clients[i];` | slirp/bootp.c | exported | Unvalidated |
| `bc->allocated = 1; paddr->s_addr = slirp->vdhcp_startaddr.s_addr + htonl(i); return bc;` | slirp/bootp.c | exported | Unvalidated |
| `request_addr` | slirp/bootp.c | left out | not translated yet |
| `if (req_addr >= dhcp_addr && req_addr < (dhcp_addr + 16)` | slirp/bootp.c | exported | Unvalidated |
| `return ((void *)0) ;` | slirp/bootp.c | exported | Unvalidated |
| `find_addr` | slirp/bootp.c | left out | not translated yet |
| `for (i = 0; i < 16; i++)` | slirp/bootp.c | exported | Unvalidated |
| `return ((void *)0) ;` | slirp/bootp.c | exported | Unvalidated |
| `found: bc = &slirp->bootp_clients[i];` | slirp/bootp.c | exported | Unvalidated |
| `bc->allocated = 1; paddr->s_addr = slirp->vdhcp_startaddr.s_addr + htonl(i); return bc;` | slirp/bootp.c | exported | Unvalidated |
| `dhcp_decode` | slirp/bootp.c | left out | not translated yet |
| `const uint8_t *p, *p_end; int len, tag; *pmsg_type = 0; preq_addr->s_addr = htonl(0L); p = bp->…` | slirp/bootp.c | exported | Unvalidated |
| `if (memcmp(p, rfc1533_cookie, 4) != 0)` | slirp/bootp.c | exported | Unvalidated |
| `p += 4;` | slirp/bootp.c | exported | Unvalidated |
| `while (p < p_end)` | slirp/bootp.c | exported | Unvalidated |
| `if (*pmsg_type == 3 && preq_addr->s_addr == htonl(0L) && bp->bp_ciaddr.s_addr)` | slirp/bootp.c | exported | Unvalidated |
| `bootp_reply` | slirp/bootp.c | left out | not translated yet |
| `BOOTPClient *bc = ((void *)0) ; struct mbuf *m; struct bootp_t *rbp; struct sockaddr_in saddr,…` | slirp/bootp.c | exported | Unvalidated |
| `do … while (0)` | slirp/bootp.c | exported | Unvalidated |
| `if (preq_addr.s_addr != htonl(0L)) … else` | slirp/bootp.c | exported | Unvalidated |
| `if (dhcp_msg_type == 0)` | slirp/bootp.c | exported | Unvalidated |
| `if (dhcp_msg_type != 1 && dhcp_msg_type != 3)` | slirp/bootp.c | exported | Unvalidated |
| `memcpy(slirp->client_ethaddr, bp->bp_hwaddr, 6); m = m_get(slirp);` | slirp/bootp.c | exported | Unvalidated |
| `if (!m)` | slirp/bootp.c | exported | Unvalidated |
| `m->m_hdr.mh_data += (2 + 14 + 40); rbp = (struct bootp_t *)m->m_hdr.mh_data; m->m_hdr.mh_data +…` | slirp/bootp.c | exported | Unvalidated |
| `if (dhcp_msg_type == 1) … else` | slirp/bootp.c | exported | Unvalidated |
| `saddr.sin_addr = slirp->vhost_addr; saddr.sin_port = htons(67); daddr.sin_port = htons(68); rbp…` | slirp/bootp.c | exported | Unvalidated |
| `if (bc) … else` | slirp/bootp.c | exported | Unvalidated |
| `*q = 255; daddr.sin_addr.s_addr = 0xffffffffu; m->m_hdr.mh_len = sizeof(struct bootp_t) - sizeo…` | slirp/bootp.c | exported | Unvalidated |
| `bootp_input` | slirp/bootp.c | left out | not translated yet |
| `if (bp->bp_op == 1)` | slirp/bootp.c | exported | Unvalidated |
| `cksum` | slirp/cksum.c | left out | not translated yet |
| `if (m->m_hdr.mh_len == 0)` | slirp/cksum.c | exported | Unvalidated |
| `w = ((uint16_t *)(m)->m_hdr.mh_data); mlen = m->m_hdr.mh_len;` | slirp/cksum.c | exported | Unvalidated |
| `if (len < mlen)` | slirp/cksum.c | exported | Unvalidated |
| `if (1 & (long) w) && (mlen > 0)` | slirp/cksum.c | exported | Unvalidated |
| `while (mlen -= 32) >= 0)` | slirp/cksum.c | exported | Unvalidated |
| `mlen += 32;` | slirp/cksum.c | exported | Unvalidated |
| `while (mlen -= 8) >= 0)` | slirp/cksum.c | exported | Unvalidated |
| `mlen += 8;` | slirp/cksum.c | exported | Unvalidated |
| `if (mlen == 0 && byte_swapped == 0)` | slirp/cksum.c | exported | Unvalidated |
| `{ … }` | slirp/cksum.c | exported | Unvalidated |
| `;` | slirp/cksum.c | exported | Unvalidated |
| `while (mlen -= 2) >= 0)` | slirp/cksum.c | exported | Unvalidated |
| `if (byte_swapped) … else` | slirp/cksum.c | exported | Unvalidated |
| `cont: if (mlen == -1) { s_util.c[1] = 0; sum += s_util.s; }` | slirp/cksum.c | exported | Unvalidated |
| `{ … }` | slirp/cksum.c | exported | Unvalidated |
| `; return (~sum & 0xffff);` | slirp/cksum.c | exported | Unvalidated |
| `ifs_insque` | slirp/if.c | left out | not translated yet |
| `ifm->m_hdr.mh_nextpkt = ifmhead->m_hdr.mh_nextpkt; ifmhead->m_hdr.mh_nextpkt = ifm; ifm->m_hdr.…` | slirp/if.c | exported | Unvalidated |
| `ifs_remque` | slirp/if.c | left out | not translated yet |
| `ifm->m_hdr.mh_prevpkt->m_hdr.mh_nextpkt = ifm->m_hdr.mh_nextpkt; ifm->m_hdr.mh_nextpkt->m_hdr.m…` | slirp/if.c | exported | Unvalidated |
| `if_init` | slirp/if.c | left out | not translated yet |
| `slirp->if_fastq.m_hdr.mh_next = slirp->if_fastq.m_hdr.mh_prev = &slirp->if_fastq; slirp->if_bat…` | slirp/if.c | exported | Unvalidated |
| `if_output` | slirp/if.c | left out | not translated yet |
| `Slirp *slirp = ifm->slirp; struct mbuf *ifq; int on_fastq = 1; ; ; ;` | slirp/if.c | exported | Unvalidated |
| `if (ifm->m_hdr.mh_flags & 0x04)` | slirp/if.c | exported | Unvalidated |
| `for (ifq = slirp->if_batchq.m_hdr.mh_prev; ifq != &slirp->if_batchq; ifq = ifq->m_hdr.mh_prev)` | slirp/if.c | exported | Unvalidated |
| `if (so && (so->so_iptos & 0x10) … else` | slirp/if.c | exported | Unvalidated |
| `ifm->m_hdr.mh_so = so; ((ifm)->m_hdr.mh_nextpkt = (ifm)->m_hdr.mh_prevpkt = (ifm)); slirp_insqu…` | slirp/if.c | exported | Unvalidated |
| `diddit: slirp->if_queued++;` | slirp/if.c | exported | Unvalidated |
| `if (so)` | slirp/if.c | exported | Unvalidated |
| `if_start(ifm->slirp);` | slirp/if.c | exported | Unvalidated |
| `if_start` | slirp/if.c | left out | not translated yet |
| `struct mbuf *ifm, *ifqt; ;` | slirp/if.c | exported | Unvalidated |
| `if (slirp->if_queued == 0)` | slirp/if.c | exported | Unvalidated |
| `again: if (!slirp_can_output(slirp->opaque)) return;` | slirp/if.c | exported | Unvalidated |
| `if (slirp->if_fastq.m_hdr.mh_next != &slirp->if_fastq) … else` | slirp/if.c | exported | Unvalidated |
| `ifqt = ifm->m_hdr.mh_prev; slirp_remque(ifm); slirp->if_queued--;` | slirp/if.c | exported | Unvalidated |
| `if (ifm->m_hdr.mh_nextpkt != ifm)` | slirp/if.c | exported | Unvalidated |
| `if (ifm->m_hdr.mh_so)` | slirp/if.c | exported | Unvalidated |
| `if_encap(slirp, (uint8_t *)ifm->m_hdr.mh_data, ifm->m_hdr.mh_len); m_free(ifm);` | slirp/if.c | exported | Unvalidated |
| `if (slirp->if_queued)` | slirp/if.c | exported | Unvalidated |
| `icmp_input` | slirp/ip_icmp.c | left out | not translated yet |
| `register struct icmp *icp; register struct ip *ip=((struct ip *)(m)->m_hdr.mh_data); int icmple…` | slirp/ip_icmp.c | exported | Unvalidated |
| `if (icmplen < 8)` | slirp/ip_icmp.c | exported | Unvalidated |
| `m->m_hdr.mh_len -= hlen; m->m_hdr.mh_data += hlen; icp = ((struct icmp *)(m)->m_hdr.mh_data);` | slirp/ip_icmp.c | exported | Unvalidated |
| `if (cksum(m, icmplen))` | slirp/ip_icmp.c | exported | Unvalidated |
| `m->m_hdr.mh_len += hlen; m->m_hdr.mh_data -= hlen; ;` | slirp/ip_icmp.c | exported | Unvalidated |
| `switch (icp->icmp_type)` | slirp/ip_icmp.c | exported | Unvalidated |
| `end_error: return;` | slirp/ip_icmp.c | exported | Unvalidated |
| `icmp_error` | slirp/ip_icmp.c | left out | not translated yet |
| `unsigned hlen, shlen, s_ip_len; register struct ip *ip; register struct icmp *icp; register str…` | slirp/ip_icmp.c | exported | Unvalidated |
| `if (type!=3 && type!=11)` | slirp/ip_icmp.c | exported | Unvalidated |
| `if (!msrc)` | slirp/ip_icmp.c | exported | Unvalidated |
| `ip = ((struct ip *)(msrc)->m_hdr.mh_data);` | slirp/ip_icmp.c | exported | Unvalidated |
| `if (ip->ip_off & 0x1fff)` | slirp/ip_icmp.c | exported | Unvalidated |
| `shlen=ip->ip_hl << 2; s_ip_len=ip->ip_len;` | slirp/ip_icmp.c | exported | Unvalidated |
| `if (ip->ip_p == IPPROTO_ICMP)` | slirp/ip_icmp.c | exported | Unvalidated |
| `m = m_get(msrc->slirp);` | slirp/ip_icmp.c | exported | Unvalidated |
| `if (!m)` | slirp/ip_icmp.c | exported | Unvalidated |
| `{ … }` | slirp/ip_icmp.c | exported | Unvalidated |
| `memcpy(m->m_hdr.mh_data, msrc->m_hdr.mh_data, msrc->m_hdr.mh_len); m->m_hdr.mh_len = msrc->m_hd…` | slirp/ip_icmp.c | exported | Unvalidated |
| `if (minsize) … else` | slirp/ip_icmp.c | exported | Unvalidated |
| `m->m_hdr.mh_len=8 +s_ip_len; icp->icmp_type = type; icp->icmp_code = code; icp->icmp_hun.ih_ids…` | slirp/ip_icmp.c | exported | Unvalidated |
| `end_error: return;` | slirp/ip_icmp.c | exported | Unvalidated |
| `icmp_reflect` | slirp/ip_icmp.c | left out | not translated yet |
| `register struct ip *ip = ((struct ip *)(m)->m_hdr.mh_data); int hlen = ip->ip_hl << 2; int optl…` | slirp/ip_icmp.c | exported | Unvalidated |
| `if (optlen > 0)` | slirp/ip_icmp.c | exported | Unvalidated |
| `ip->ip_ttl = 255;` | slirp/ip_icmp.c | exported | Unvalidated |
| `{ … }` | slirp/ip_icmp.c | exported | Unvalidated |
| `(void ) ip_output((struct socket *) ((void *)0) , m);` | slirp/ip_icmp.c | exported | Unvalidated |
| `ip_init` | slirp/ip_input.c | left out | not translated yet |
| `slirp->ipq.ip_link.next = slirp->ipq.ip_link.prev = &slirp->ipq.ip_link; udp_init(slirp); tcp_i…` | slirp/ip_input.c | exported | Unvalidated |
| `ip_input` | slirp/ip_input.c | left out | not translated yet |
| `Slirp *slirp = m->slirp; register struct ip *ip; int hlen; ; ; ;` | slirp/ip_input.c | exported | Unvalidated |
| `if (m->m_hdr.mh_len < sizeof (struct ip))` | slirp/ip_input.c | exported | Unvalidated |
| `ip = ((struct ip *)(m)->m_hdr.mh_data);` | slirp/ip_input.c | exported | Unvalidated |
| `if (ip->ip_v != 4)` | slirp/ip_input.c | exported | Unvalidated |
| `hlen = ip->ip_hl << 2;` | slirp/ip_input.c | exported | Unvalidated |
| `if (hlen<sizeof(struct ip ) || hlen>m->m_hdr.mh_len)` | slirp/ip_input.c | exported | Unvalidated |
| `if (cksum(m,hlen))` | slirp/ip_input.c | exported | Unvalidated |
| `((ip->ip_len) = ntohs((uint16_t)(ip->ip_len)));` | slirp/ip_input.c | exported | Unvalidated |
| `if (ip->ip_len < hlen)` | slirp/ip_input.c | exported | Unvalidated |
| `((ip->ip_id) = ntohs((uint16_t)(ip->ip_id))); ((ip->ip_off) = ntohs((uint16_t)(ip->ip_off)));` | slirp/ip_input.c | exported | Unvalidated |
| `if (m->m_hdr.mh_len < ip->ip_len)` | slirp/ip_input.c | exported | Unvalidated |
| `if (slirp->restricted)` | slirp/ip_input.c | exported | Unvalidated |
| `if (m->m_hdr.mh_len > ip->ip_len)` | slirp/ip_input.c | exported | Unvalidated |
| `if (ip->ip_ttl==0)` | slirp/ip_input.c | exported | Unvalidated |
| `if (ip->ip_off &~ 0x4000) … else` | slirp/ip_input.c | exported | Unvalidated |
| `switch (ip->ip_p)` | slirp/ip_input.c | exported | Unvalidated |
| `return;` | slirp/ip_input.c | exported | Unvalidated |
| `bad: m_free(m);` | slirp/ip_input.c | exported | Unvalidated |
| `return;` | slirp/ip_input.c | exported | Unvalidated |
| `ip_reass` | slirp/ip_input.c | left out | not translated yet |
| `register struct mbuf *m = dtom(slirp, ip); register struct ipasfrag *q; int hlen = ip->ip_hl <<…` | slirp/ip_input.c | exported | Unvalidated |
| `if (fp == ((void *)0)` | slirp/ip_input.c | exported | Unvalidated |
| `for (q = fp->frag_link.next; q != (struct ipasfrag *)&fp->frag_link; q = q->ipf_link.next)` | slirp/ip_input.c | exported | Unvalidated |
| `if (q->ipf_link.prev != &fp->frag_link)` | slirp/ip_input.c | exported | Unvalidated |
| `while (q != (struct ipasfrag*)&fp->frag_link && ip->ip_off + ip->ip_len > q->ipf_ip.ip_off)` | slirp/ip_input.c | exported | Unvalidated |
| `insert: ip_enq(((struct ipasfrag *)(((char*)(ip)) - sizeof(struct qlink))), q->ipf_link.prev);` | slirp/ip_input.c | exported | Unvalidated |
| `next = 0;` | slirp/ip_input.c | exported | Unvalidated |
| `for (q = fp->frag_link.next; q != (struct ipasfrag*)&fp->frag_link; q = q->ipf_link.next)` | slirp/ip_input.c | exported | Unvalidated |
| `if ((struct ipasfrag *)(q->ipf_link.prev))->ipf_ip.ip_tos & 1)` | slirp/ip_input.c | exported | Unvalidated |
| `q = fp->frag_link.next; m = dtom(slirp, q); q = (struct ipasfrag *) q->ipf_link.next;` | slirp/ip_input.c | exported | Unvalidated |
| `while (q != (struct ipasfrag*)&fp->frag_link)` | slirp/ip_input.c | exported | Unvalidated |
| `q = fp->frag_link.next;` | slirp/ip_input.c | exported | Unvalidated |
| `if (m->m_hdr.mh_flags & 0x01)` | slirp/ip_input.c | exported | Unvalidated |
| `ip = ((struct ip*)(((char*)(q)) + sizeof(struct qlink))); ip->ip_len = next; ip->ip_tos &= ~1;…` | slirp/ip_input.c | exported | Unvalidated |
| `dropfrag: m_free(m);` | slirp/ip_input.c | exported | Unvalidated |
| `return ((void *)0) ;` | slirp/ip_input.c | exported | Unvalidated |
| `ip_freef` | slirp/ip_input.c | left out | not translated yet |
| `for (q = fp->frag_link.next; q != (struct ipasfrag*)&fp->frag_link; q = p)` | slirp/ip_input.c | exported | Unvalidated |
| `slirp_remque(&fp->ip_link); (void) m_free(dtom(slirp, fp));` | slirp/ip_input.c | exported | Unvalidated |
| `ip_enq` | slirp/ip_input.c | left out | not translated yet |
| `; ; p->ipf_link.prev = prev; p->ipf_link.next = prev->ipf_link.next; ((struct ipasfrag *)(prev-…` | slirp/ip_input.c | exported | Unvalidated |
| `ip_deq` | slirp/ip_input.c | left out | not translated yet |
| `((struct ipasfrag *)(p->ipf_link.prev))->ipf_link.next = p->ipf_link.next; ((struct ipasfrag *)…` | slirp/ip_input.c | exported | Unvalidated |
| `ip_slowtimo` | slirp/ip_input.c | left out | not translated yet |
| `struct qlink *l; ; l = slirp->ipq.ip_link.next;` | slirp/ip_input.c | exported | Unvalidated |
| `if (l == ((void *)0)` | slirp/ip_input.c | exported | Unvalidated |
| `while (l != &slirp->ipq.ip_link)` | slirp/ip_input.c | exported | Unvalidated |
| `ip_stripoptions` | slirp/ip_input.c | left out | not translated yet |
| `register int i; struct ip *ip = ((struct ip *)(m)->m_hdr.mh_data); register caddr_t opts; int o…` | slirp/ip_input.c | exported | Unvalidated |
| `ip_output` | slirp/ip_output.c | left out | not translated yet |
| `Slirp *slirp = m0->slirp; register struct ip *ip; register struct mbuf *m = m0; register int hl…` | slirp/ip_output.c | exported | Unvalidated |
| `if ((uint16_t)ip->ip_len <= 1500)` | slirp/ip_output.c | exported | Unvalidated |
| `if (ip->ip_off & 0x4000)` | slirp/ip_output.c | exported | Unvalidated |
| `len = (1500 - hlen) &~ 7;` | slirp/ip_output.c | exported | Unvalidated |
| `if (len < 8)` | slirp/ip_output.c | exported | Unvalidated |
| `{ … }` | slirp/ip_output.c | exported | Unvalidated |
| `done: return (error);` | slirp/ip_output.c | exported | Unvalidated |
| `bad: m_free(m0);` | slirp/ip_output.c | exported | Unvalidated |
| `goto done;` | slirp/ip_output.c | exported | Unvalidated |
| `m_init` | slirp/mbuf.c | left out | not translated yet |
| `slirp->m_freelist.m_hdr.mh_next = slirp->m_freelist.m_hdr.mh_prev = &slirp->m_freelist; slirp->…` | slirp/mbuf.c | exported | Unvalidated |
| `m_get` | slirp/mbuf.c | left out | not translated yet |
| `register struct mbuf *m; int flags = 0; ;` | slirp/mbuf.c | exported | Unvalidated |
| `if (slirp->m_freelist.m_hdr.mh_next == &slirp->m_freelist) … else` | slirp/mbuf.c | exported | Unvalidated |
| `slirp_insque(m,&slirp->m_usedlist); m->m_hdr.mh_flags = (flags | 0x04); m->m_hdr.mh_size = (150…` | slirp/mbuf.c | exported | Unvalidated |
| `end_error: ;` | slirp/mbuf.c | exported | Unvalidated |
| `return m;` | slirp/mbuf.c | exported | Unvalidated |
| `m_free` | slirp/mbuf.c | left out | not translated yet |
| `; ;` | slirp/mbuf.c | exported | Unvalidated |
| `if (m)` | slirp/mbuf.c | exported | Unvalidated |
| `m_cat` | slirp/mbuf.c | left out | not translated yet |
| `if (m->m_hdr.mh_flags & 0x01)? (((m)->M_dat.m_ext_ + (m)->m_hdr.mh_size) - (m)->m_hdr.mh_data)…` | slirp/mbuf.c | exported | Unvalidated |
| `memcpy(m->m_hdr.mh_data+m->m_hdr.mh_len, n->m_hdr.mh_data, n->m_hdr.mh_len); m->m_hdr.mh_len +=…` | slirp/mbuf.c | exported | Unvalidated |
| `m_inc` | slirp/mbuf.c | left out | not translated yet |
| `if (m->m_hdr.mh_size>size)` | slirp/mbuf.c | exported | Unvalidated |
| `if (m->m_hdr.mh_flags & 0x01) … else` | slirp/mbuf.c | exported | Unvalidated |
| `m->m_hdr.mh_size = size;` | slirp/mbuf.c | exported | Unvalidated |
| `m_adj` | slirp/mbuf.c | left out | not translated yet |
| `if (m == ((void *)0)` | slirp/mbuf.c | exported | Unvalidated |
| `if (len >= 0) … else` | slirp/mbuf.c | exported | Unvalidated |
| `m_copy` | slirp/mbuf.c | left out | not translated yet |
| `if (len > (((n->m_hdr.mh_flags & 0x01)? (((n)->M_dat.m_ext_ + (n)->m_hdr.mh_size) - (n)->m_hdr.…` | slirp/mbuf.c | exported | Unvalidated |
| `memcpy((n->m_hdr.mh_data + n->m_hdr.mh_len), (m->m_hdr.mh_data + off), len); n->m_hdr.mh_len +=…` | slirp/mbuf.c | exported | Unvalidated |
| `dtom` | slirp/mbuf.c | left out | not translated yet |
| `struct mbuf *m; ; ;` | slirp/mbuf.c | exported | Unvalidated |
| `for (m = slirp->m_usedlist.m_hdr.mh_next; m != &slirp->m_usedlist; m = m->m_hdr.mh_next)` | slirp/mbuf.c | exported | Unvalidated |
| `; return (struct mbuf *)0;` | slirp/mbuf.c | exported | Unvalidated |
| `slirp_insque` | slirp/misc.c | left out | not translated yet |
| `register struct quehead *element = (struct quehead *) a; register struct quehead *head = (struc…` | slirp/misc.c | exported | Unvalidated |
| `slirp_remque` | slirp/misc.c | left out | not translated yet |
| `register struct quehead *element = (struct quehead *) a; ((struct quehead *)(element->qh_link))…` | slirp/misc.c | exported | Unvalidated |
| `add_exec` | slirp/misc.c | left out | not translated yet |
| `for (tmp_ptr = *ex_ptr; tmp_ptr; tmp_ptr = tmp_ptr->ex_next)` | slirp/misc.c | exported | Unvalidated |
| `tmp_ptr = *ex_ptr; *ex_ptr = (struct ex_list *)malloc(sizeof(struct ex_list)); (*ex_ptr)->ex_fp…` | slirp/misc.c | exported | Unvalidated |
| `os_socket` | slirp/misc.c | left out | not translated yet |
| `return socket(domain, type, protocol);` | slirp/misc.c | exported | Unvalidated |
| `os_get_time_ms` | slirp/misc.c | left out | not translated yet |
| `struct timespec ts; clock_gettime( 1 , &ts); return ts.tv_sec * 1000 + (ts.tv_nsec / 1000000);` | slirp/misc.c | exported | Unvalidated |
| `fork_exec` | slirp/misc.c | left out | not translated yet |
| `return 0;` | slirp/misc.c | exported | Unvalidated |
| `fd_nonblock` | slirp/misc.c | left out | not translated yet |
| `int opt = 1; ioctl(fd, 0x5421 , &opt);` | slirp/misc.c | exported | Unvalidated |
| `fd_block` | slirp/misc.c | left out | not translated yet |
| `int opt = 0; ioctl(fd, 0x5421 , &opt);` | slirp/misc.c | exported | Unvalidated |
| `sbfree` | slirp/sbuf.c | left out | not translated yet |
| `free(sb->sb_data);` | slirp/sbuf.c | exported | Unvalidated |
| `sbdrop` | slirp/sbuf.c | left out | not translated yet |
| `if (num > sb->sb_cc)` | slirp/sbuf.c | exported | Unvalidated |
| `sb->sb_cc -= num; sb->sb_rptr += num;` | slirp/sbuf.c | exported | Unvalidated |
| `if (sb->sb_rptr >= sb->sb_data + sb->sb_datalen)` | slirp/sbuf.c | exported | Unvalidated |
| `sbreserve` | slirp/sbuf.c | left out | not translated yet |
| `if (sb->sb_data) … else` | slirp/sbuf.c | exported | Unvalidated |
| `sbappend` | slirp/sbuf.c | left out | not translated yet |
| `int ret = 0; ; ; ; ;` | slirp/sbuf.c | exported | Unvalidated |
| `if (m->m_hdr.mh_len <= 0)` | slirp/sbuf.c | exported | Unvalidated |
| `if (so->so_urgc)` | slirp/sbuf.c | exported | Unvalidated |
| `if (!so->so_rcv.sb_cc)` | slirp/sbuf.c | exported | Unvalidated |
| `if (ret <= 0) … else` | slirp/sbuf.c | exported | Unvalidated |
| `m_free(m);` | slirp/sbuf.c | exported | Unvalidated |
| `sbappendsb` | slirp/sbuf.c | left out | not translated yet |
| `int len, n, nn; len = m->m_hdr.mh_len;` | slirp/sbuf.c | exported | Unvalidated |
| `if (sb->sb_wptr < sb->sb_rptr) … else` | slirp/sbuf.c | exported | Unvalidated |
| `sb->sb_cc += n; sb->sb_wptr += n;` | slirp/sbuf.c | exported | Unvalidated |
| `if (sb->sb_wptr >= sb->sb_data + sb->sb_datalen)` | slirp/sbuf.c | exported | Unvalidated |
| `sbcopy` | slirp/sbuf.c | left out | not translated yet |
| `char *from; from = sb->sb_rptr + off;` | slirp/sbuf.c | exported | Unvalidated |
| `if (from >= sb->sb_data + sb->sb_datalen)` | slirp/sbuf.c | exported | Unvalidated |
| `if (from < sb->sb_wptr) … else` | slirp/sbuf.c | exported | Unvalidated |
| `get_dns_addr` | slirp/slirp.c | left out | not translated yet |
| `if (dns_addr.s_addr != 0)` | slirp/slirp.c | exported | Unvalidated |
| `f = fopen("/etc/resolv.conf", "r");` | slirp/slirp.c | exported | Unvalidated |
| `if (!f)` | slirp/slirp.c | exported | Unvalidated |
| `while (fgets(buff, 512, f) != ((void *)0)` | slirp/slirp.c | exported | Unvalidated |
| `fclose(f);` | slirp/slirp.c | exported | Unvalidated |
| `if (!found)` | slirp/slirp.c | exported | Unvalidated |
| `return 0;` | slirp/slirp.c | exported | Unvalidated |
| `slirp_init_once` | slirp/slirp.c | left out | not translated yet |
| `if (initialized)` | slirp/slirp.c | exported | Unvalidated |
| `initialized = 1; loopback_addr.s_addr = htonl( ((in_addr_t) 0x7f000001) );` | slirp/slirp.c | exported | Unvalidated |
| `slirp_init` | slirp/slirp.c | left out | not translated yet |
| `Slirp *slirp = mallocz(sizeof(Slirp)); slirp_init_once(); slirp->restricted = restricted; if_in…` | slirp/slirp.c | exported | Unvalidated |
| `if (vhostname)` | slirp/slirp.c | exported | Unvalidated |
| `if (tftp_path)` | slirp/slirp.c | exported | Unvalidated |
| `if (bootfile)` | slirp/slirp.c | exported | Unvalidated |
| `slirp->vdhcp_startaddr = vdhcp_start; slirp->vnameserver_addr = vnameserver; slirp->opaque = op…` | slirp/slirp.c | exported | Unvalidated |
| `slirp_cleanup` | slirp/slirp.c | left out | not translated yet |
| `free(slirp->tftp_prefix); free(slirp->bootp_filename); free(slirp);` | slirp/slirp.c | exported | Unvalidated |
| `slirp_select_fill` | slirp/slirp.c | left out | not translated yet |
| `struct socket *so, *so_next; int nfds; global_readfds = ((void *)0) ; global_writefds = ((void…` | slirp/slirp.c | exported | Unvalidated |
| `{ … }` | slirp/slirp.c | exported | Unvalidated |
| `*pnfds = nfds;` | slirp/slirp.c | exported | Unvalidated |
| `slirp_select_poll` | slirp/slirp.c | left out | not translated yet |
| `struct socket *so, *so_next; int ret; global_readfds = readfds; global_writefds = writefds; glo…` | slirp/slirp.c | exported | Unvalidated |
| `{ … }` | slirp/slirp.c | exported | Unvalidated |
| `global_readfds = ((void *)0) ; global_writefds = ((void *)0) ; global_xfds = ((void *)0) ;` | slirp/slirp.c | exported | Unvalidated |
| `arp_input` | slirp/slirp.c | left out | not translated yet |
| `struct ethhdr *eh = (struct ethhdr *)pkt; struct arphdr *ah = (struct arphdr *)(pkt + 14); uint…` | slirp/slirp.c | exported | Unvalidated |
| `switch (ar_op)` | slirp/slirp.c | exported | Unvalidated |
| `slirp_input` | slirp/slirp.c | left out | not translated yet |
| `if (pkt_len < 14)` | slirp/slirp.c | exported | Unvalidated |
| `proto = ntohs(*(uint16_t *)(pkt + 12));` | slirp/slirp.c | exported | Unvalidated |
| `switch (proto)` | slirp/slirp.c | exported | Unvalidated |
| `if_encap` | slirp/slirp.c | left out | not translated yet |
| `if (ip_data_len + 14 > sizeof(buf)` | slirp/slirp.c | exported | Unvalidated |
| `if (!memcmp(slirp->client_ethaddr, zero_ethaddr, 6)) … else` | slirp/slirp.c | exported | Unvalidated |
| `slirp_remove_hostfwd` | slirp/slirp.c | left out | not translated yet |
| `for (so = head->so_next; so != head; so = so->so_next)` | slirp/slirp.c | exported | Unvalidated |
| `return -1;` | slirp/slirp.c | exported | Unvalidated |
| `slirp_add_hostfwd` | slirp/slirp.c | left out | not translated yet |
| `if (!guest_addr.s_addr)` | slirp/slirp.c | exported | Unvalidated |
| `if (is_udp) … else` | slirp/slirp.c | exported | Unvalidated |
| `return 0;` | slirp/slirp.c | exported | Unvalidated |
| `slirp_add_exec` | slirp/slirp.c | left out | not translated yet |
| `if (!guest_addr->s_addr)` | slirp/slirp.c | exported | Unvalidated |
| `if (guest_addr->s_addr & slirp->vnetwork_mask.s_addr) != slirp->vnetwork_addr.s_addr || guest_a…` | slirp/slirp.c | exported | Unvalidated |
| `return add_exec(&slirp->exec_list, do_pty, (char *)args, *guest_addr, htons(guest_port));` | slirp/slirp.c | exported | Unvalidated |
| `slirp_send` | slirp/slirp.c | left out | not translated yet |
| `return send(so->s, buf, len, flags);` | slirp/slirp.c | exported | Unvalidated |
| `slirp_find_ctl_socket` | slirp/slirp.c | left out | not translated yet |
| `for (so = slirp->tcb.so_next; so != &slirp->tcb; so = so->so_next)` | slirp/slirp.c | exported | Unvalidated |
| `return ((void *)0) ;` | slirp/slirp.c | exported | Unvalidated |
| `slirp_socket_can_recv` | slirp/slirp.c | left out | not translated yet |
| `struct iovec iov[2]; struct socket *so; so = slirp_find_ctl_socket(slirp, guest_addr, guest_por…` | slirp/slirp.c | exported | Unvalidated |
| `if (!so || so->so_state & 0x001)` | slirp/slirp.c | exported | Unvalidated |
| `if (!(((so)->so_state & (0x008|0x004)) == 0x004) || so->so_snd.sb_cc >= (so->so_snd.sb_datalen/…` | slirp/slirp.c | exported | Unvalidated |
| `return sopreprbuf(so, iov, ((void *)0) );` | slirp/slirp.c | exported | Unvalidated |
| `slirp_socket_recv` | slirp/slirp.c | left out | not translated yet |
| `if (!so)` | slirp/slirp.c | exported | Unvalidated |
| `ret = soreadbuf(so, (const char *)buf, size);` | slirp/slirp.c | exported | Unvalidated |
| `if (ret > 0)` | slirp/slirp.c | exported | Unvalidated |
| `solookup` | slirp/socket.c | left out | not translated yet |
| `for (so = head->so_next; so != head; so = so->so_next)` | slirp/socket.c | exported | Unvalidated |
| `if (so == head)` | slirp/socket.c | exported | Unvalidated |
| `return so;` | slirp/socket.c | exported | Unvalidated |
| `socreate` | slirp/socket.c | left out | not translated yet |
| `struct socket *so; so = (struct socket *)malloc(sizeof(struct socket));` | slirp/socket.c | exported | Unvalidated |
| `if (so)` | slirp/socket.c | exported | Unvalidated |
| `return(so);` | slirp/socket.c | exported | Unvalidated |
| `sofree` | slirp/socket.c | left out | not translated yet |
| `if (so->so_emu==0x8 && so->extra)` | slirp/socket.c | exported | Unvalidated |
| `if (so == slirp->tcp_last_so) … else` | slirp/socket.c | exported | Unvalidated |
| `m_free(so->so_m);` | slirp/socket.c | exported | Unvalidated |
| `if (so->so_next && so->so_prev)` | slirp/socket.c | exported | Unvalidated |
| `free(so);` | slirp/socket.c | exported | Unvalidated |
| `sopreprbuf` | slirp/socket.c | left out | not translated yet |
| `int n, lss, total; struct sbuf *sb = &so->so_snd; int len = sb->sb_datalen - sb->sb_cc; int mss…` | slirp/socket.c | exported | Unvalidated |
| `if (len <= 0)` | slirp/socket.c | exported | Unvalidated |
| `iov[0].iov_base = sb->sb_wptr; iov[1].iov_base = ((void *)0) ; iov[1].iov_len = 0;` | slirp/socket.c | exported | Unvalidated |
| `if (sb->sb_wptr < sb->sb_rptr) … else` | slirp/socket.c | exported | Unvalidated |
| `if (np)` | slirp/socket.c | exported | Unvalidated |
| `return iov[0].iov_len + (n - 1) * iov[1].iov_len;` | slirp/socket.c | exported | Unvalidated |
| `soread` | slirp/socket.c | left out | not translated yet |
| `int n, nn; struct sbuf *sb = &so->so_snd; struct iovec iov[2]; ; ; sopreprbuf(so, iov, &n); nn…` | slirp/socket.c | exported | Unvalidated |
| `if (nn <= 0)` | slirp/socket.c | exported | Unvalidated |
| `if (n == 2 && nn == iov[0].iov_len)` | slirp/socket.c | exported | Unvalidated |
| `; sb->sb_cc += nn; sb->sb_wptr += nn;` | slirp/socket.c | exported | Unvalidated |
| `if (sb->sb_wptr >= (sb->sb_data + sb->sb_datalen)` | slirp/socket.c | exported | Unvalidated |
| `return nn;` | slirp/socket.c | exported | Unvalidated |
| `soreadbuf` | slirp/socket.c | left out | not translated yet |
| `int n, nn, copy = size; struct sbuf *sb = &so->so_snd; struct iovec iov[2]; ; ;` | slirp/socket.c | exported | Unvalidated |
| `if (sopreprbuf(so, iov, &n) < size)` | slirp/socket.c | exported | Unvalidated |
| `nn = ((iov[0].iov_len) < (copy) ? (iov[0].iov_len) : (copy)); memcpy(iov[0].iov_base, buf, nn);…` | slirp/socket.c | exported | Unvalidated |
| `if (copy == 0)` | slirp/socket.c | exported | Unvalidated |
| `memcpy(iov[1].iov_base, buf, copy);` | slirp/socket.c | exported | Unvalidated |
| `done: sb->sb_cc += size;` | slirp/socket.c | exported | Unvalidated |
| `sb->sb_wptr += size;` | slirp/socket.c | exported | Unvalidated |
| `if (sb->sb_wptr >= (sb->sb_data + sb->sb_datalen)` | slirp/socket.c | exported | Unvalidated |
| `return size;` | slirp/socket.c | exported | Unvalidated |
| `err: sofcantrcvmore(so);` | slirp/socket.c | exported | Unvalidated |
| `tcp_sockclosed(((so)->so_tcpcb)); fprintf( stderr , "soreadbuf buffer to small"); return -1;` | slirp/socket.c | exported | Unvalidated |
| `sorecvoob` | slirp/socket.c | left out | not translated yet |
| `struct tcpcb *tp = ((so)->so_tcpcb); ; ; soread(so); tp->snd_up = tp->snd_una + so->so_snd.sb_c…` | slirp/socket.c | exported | Unvalidated |
| `sosendoob` | slirp/socket.c | left out | not translated yet |
| `struct sbuf *sb = &so->so_rcv; char buff[2048]; int n, len; ; ; ;` | slirp/socket.c | exported | Unvalidated |
| `if (so->so_urgc > 2048)` | slirp/socket.c | exported | Unvalidated |
| `if (sb->sb_rptr < sb->sb_wptr) … else` | slirp/socket.c | exported | Unvalidated |
| `sb->sb_cc -= n; sb->sb_rptr += n;` | slirp/socket.c | exported | Unvalidated |
| `if (sb->sb_rptr >= (sb->sb_data + sb->sb_datalen)` | slirp/socket.c | exported | Unvalidated |
| `return n;` | slirp/socket.c | exported | Unvalidated |
| `sowrite` | slirp/socket.c | left out | not translated yet |
| `int n,nn; struct sbuf *sb = &so->so_rcv; int len = sb->sb_cc; struct iovec iov[2]; ; ;` | slirp/socket.c | exported | Unvalidated |
| `if (so->so_urgc)` | slirp/socket.c | exported | Unvalidated |
| `iov[0].iov_base = sb->sb_rptr; iov[1].iov_base = ((void *)0) ; iov[1].iov_len = 0;` | slirp/socket.c | exported | Unvalidated |
| `if (sb->sb_rptr < sb->sb_wptr) … else` | slirp/socket.c | exported | Unvalidated |
| `nn = slirp_send(so, iov[0].iov_base, iov[0].iov_len,0);` | slirp/socket.c | exported | Unvalidated |
| `if (nn < 0 && ( (*__errno_location ()) == 11 || (*__errno_location ()) == 4)` | slirp/socket.c | exported | Unvalidated |
| `if (nn <= 0)` | slirp/socket.c | exported | Unvalidated |
| `if (n == 2 && nn == iov[0].iov_len)` | slirp/socket.c | exported | Unvalidated |
| `; sb->sb_cc -= nn; sb->sb_rptr += nn;` | slirp/socket.c | exported | Unvalidated |
| `if (sb->sb_rptr >= (sb->sb_data + sb->sb_datalen)` | slirp/socket.c | exported | Unvalidated |
| `if (so->so_state & 0x040) && sb->sb_cc == 0)` | slirp/socket.c | exported | Unvalidated |
| `return nn;` | slirp/socket.c | exported | Unvalidated |
| `sorecvfrom` | slirp/socket.c | left out | not translated yet |
| `struct sockaddr_in addr; socklen_t addrlen = sizeof(struct sockaddr_in); ; ;` | slirp/socket.c | exported | Unvalidated |
| `if (so->so_type == IPPROTO_ICMP) … else` | slirp/socket.c | exported | Unvalidated |
| `sosendto` | slirp/socket.c | left out | not translated yet |
| `Slirp *slirp = so->slirp; int ret; struct sockaddr_in addr; ; ; ; addr.sin_family = 2 ;` | slirp/socket.c | exported | Unvalidated |
| `if (so->so_faddr.s_addr & slirp->vnetwork_mask.s_addr) == slirp->vnetwork_addr.s_addr) … else` | slirp/socket.c | exported | Unvalidated |
| `addr.sin_port = so->so_fport; ; ret = sendto(so->s, m->m_hdr.mh_data, m->m_hdr.mh_len, 0, (stru…` | slirp/socket.c | exported | Unvalidated |
| `if (ret < 0)` | slirp/socket.c | exported | Unvalidated |
| `if (so->so_expire)` | slirp/socket.c | exported | Unvalidated |
| `so->so_state &= 0xf000; so->so_state |= 0x004; return 0;` | slirp/socket.c | exported | Unvalidated |
| `tcp_listen` | slirp/socket.c | left out | not translated yet |
| `struct sockaddr_in addr; struct socket *so; int s, opt = 1; socklen_t addrlen = sizeof(addr); m…` | slirp/socket.c | exported | Unvalidated |
| `if (!so)` | slirp/socket.c | exported | Unvalidated |
| `if (so->so_tcpcb = tcp_newtcpcb(so)) == ((void *)0)` | slirp/socket.c | exported | Unvalidated |
| `slirp_insque(so, &slirp->tcb);` | slirp/socket.c | exported | Unvalidated |
| `if (flags & 0x200)` | slirp/socket.c | exported | Unvalidated |
| `so->so_state &= 0xf000; so->so_state |= (0x100 | flags); so->so_lport = lport; so->so_laddr.s_a…` | slirp/socket.c | exported | Unvalidated |
| `if (s = os_socket( 2 , SOCK_STREAM ,0)) < 0) || (setsockopt(s, 1 , 2 ,(char *)&opt,sizeof(int))…` | slirp/socket.c | exported | Unvalidated |
| `setsockopt(s, 1 , 10 ,(char *)&opt,sizeof(int)); getsockname(s,(struct sockaddr *)&addr,&addrle…` | slirp/socket.c | exported | Unvalidated |
| `if (addr.sin_addr.s_addr == 0 || addr.sin_addr.s_addr == loopback_addr.s_addr) … else` | slirp/socket.c | exported | Unvalidated |
| `so->s = s; return so;` | slirp/socket.c | exported | Unvalidated |
| `soisfconnecting` | slirp/socket.c | left out | not translated yet |
| `so->so_state &= ~(0x001|0x004|0x008| 0x010|0x040); so->so_state |= 0x002;` | slirp/socket.c | exported | Unvalidated |
| `soisfconnected` | slirp/socket.c | left out | not translated yet |
| `so->so_state &= ~(0x002|0x040|0x001); so->so_state |= 0x004;` | slirp/socket.c | exported | Unvalidated |
| `sofcantrcvmore` | slirp/socket.c | left out | not translated yet |
| `if (so->so_state & 0x001) == 0)` | slirp/socket.c | exported | Unvalidated |
| `so->so_state &= ~(0x002);` | slirp/socket.c | exported | Unvalidated |
| `if (so->so_state & 0x010) … else` | slirp/socket.c | exported | Unvalidated |
| `sofcantsendmore` | slirp/socket.c | left out | not translated yet |
| `if (so->so_state & 0x001) == 0)` | slirp/socket.c | exported | Unvalidated |
| `so->so_state &= ~(0x002);` | slirp/socket.c | exported | Unvalidated |
| `if (so->so_state & 0x008) … else` | slirp/socket.c | exported | Unvalidated |
| `sofwdrain` | slirp/socket.c | left out | not translated yet |
| `if (so->so_rcv.sb_cc) … else` | slirp/socket.c | exported | Unvalidated |
| `tcp_reass` | slirp/tcp_input.c | left out | not translated yet |
| `if (ti == ((void *)0)` | slirp/tcp_input.c | exported | Unvalidated |
| `for (q = ((struct tcpiphdr*)(((char*)((tp)->seg_next)) + sizeof(struct qlink); !(((struct qlink…` | slirp/tcp_input.c | exported | Unvalidated |
| `if (!(((struct qlink*)(((char*)(((struct tcpiphdr*)(((char*)(((struct qlink*)(((char*)(q)) - si…` | slirp/tcp_input.c | exported | Unvalidated |
| `ti->ti_i.ih_mbuf.mptr = m;` | slirp/tcp_input.c | exported | Unvalidated |
| `while (!(((struct qlink*)(((char*)(q)) - sizeof(struct qlink))) == (struct qlink*)(tp)` | slirp/tcp_input.c | exported | Unvalidated |
| `slirp_insque(((struct qlink*)(((char*)(ti)) - sizeof(struct qlink))), ((struct qlink*)(((char*)…` | slirp/tcp_input.c | exported | Unvalidated |
| `present: if (!((tp->t_state) >= 4)) return (0);` | slirp/tcp_input.c | exported | Unvalidated |
| `ti = ((struct tcpiphdr*)(((char*)((tp)->seg_next)) + sizeof(struct qlink)));` | slirp/tcp_input.c | exported | Unvalidated |
| `if ((struct qlink*)(((char*)(ti)) - sizeof(struct qlink))) == (struct qlink*)(tp)) || ti->ti_t.…` | slirp/tcp_input.c | exported | Unvalidated |
| `if (tp->t_state == 3 && ti->ti_i.ih_len)` | slirp/tcp_input.c | exported | Unvalidated |
| `do … while (ti != (struct tcpiphdr *)tp && ti->ti_t.th_seq == tp->rcv_nxt)` | slirp/tcp_input.c | exported | Unvalidated |
| `return (flags);` | slirp/tcp_input.c | exported | Unvalidated |
| `tcp_input` | slirp/tcp_input.c | left out | not translated yet |
| `struct ip save_ip, *ip; register struct tcpiphdr *ti; caddr_t optp = ((void *)0) ; int optlen =…` | slirp/tcp_input.c | exported | Unvalidated |
| `if (m == ((void *)0)` | slirp/tcp_input.c | exported | Unvalidated |
| `slirp = m->slirp; ti = ((struct tcpiphdr *)(m)->m_hdr.mh_data);` | slirp/tcp_input.c | exported | Unvalidated |
| `if (iphlen > sizeof(struct ip ))` | slirp/tcp_input.c | exported | Unvalidated |
| `ip=((struct ip *)(m)->m_hdr.mh_data); save_ip = *ip; save_ip.ip_len+= iphlen; tlen = ((struct i…` | slirp/tcp_input.c | exported | Unvalidated |
| `if (cksum(m, len))` | slirp/tcp_input.c | exported | Unvalidated |
| `off = ti->ti_t.th_off << 2;` | slirp/tcp_input.c | exported | Unvalidated |
| `if (off < sizeof (struct tcphdr) || off > tlen)` | slirp/tcp_input.c | exported | Unvalidated |
| `tlen -= off; ti->ti_i.ih_len = tlen;` | slirp/tcp_input.c | exported | Unvalidated |
| `if (off > sizeof (struct tcphdr))` | slirp/tcp_input.c | exported | Unvalidated |
| `tiflags = ti->ti_t.th_flags; ((ti->ti_t.th_seq) = ntohl((ti->ti_t.th_seq))); ((ti->ti_t.th_ack)…` | slirp/tcp_input.c | exported | Unvalidated |
| `if (slirp->restricted)` | slirp/tcp_input.c | exported | Unvalidated |
| `findso: so = slirp->tcp_last_so;` | slirp/tcp_input.c | exported | Unvalidated |
| `if (so->so_fport != ti->ti_t.th_dport || so->so_lport != ti->ti_t.th_sport || so->so_laddr.s_ad…` | slirp/tcp_input.c | exported | Unvalidated |
| `if (so == ((void *)0)` | slirp/tcp_input.c | exported | Unvalidated |
| `if (so->so_state & 0x002)` | slirp/tcp_input.c | exported | Unvalidated |
| `tp = ((so)->so_tcpcb);` | slirp/tcp_input.c | exported | Unvalidated |
| `if (tp == ((void *)0)` | slirp/tcp_input.c | exported | Unvalidated |
| `if (tp->t_state == 0)` | slirp/tcp_input.c | exported | Unvalidated |
| `tiwin = ti->ti_t.th_win; tp->t_idle = 0;` | slirp/tcp_input.c | exported | Unvalidated |
| `if (0) … else` | slirp/tcp_input.c | exported | Unvalidated |
| `if (optp && tp->t_state != 1)` | slirp/tcp_input.c | exported | Unvalidated |
| `if (tp->t_state == 4 && (tiflags & (0x02|0x01|0x04|0x20|0x10)) == 0x10 && ti->ti_t.th_seq == tp…` | slirp/tcp_input.c | exported | Unvalidated |
| `{ … }` | slirp/tcp_input.c | exported | Unvalidated |
| `switch (tp->t_state)` | slirp/tcp_input.c | left out | still being translated |
| `todrop = tp->rcv_nxt - ti->ti_t.th_seq;` | slirp/tcp_input.c | exported | Unvalidated |
| `if (todrop > 0)` | slirp/tcp_input.c | exported | Unvalidated |
| `if (so->so_state & 0x001) && tp->t_state > 5 && ti->ti_i.ih_len)` | slirp/tcp_input.c | exported | Unvalidated |
| `todrop = (ti->ti_t.th_seq+ti->ti_i.ih_len) - (tp->rcv_nxt+tp->rcv_wnd);` | slirp/tcp_input.c | exported | Unvalidated |
| `if (todrop > 0)` | slirp/tcp_input.c | exported | Unvalidated |
| `if (tiflags&0x04)` | slirp/tcp_input.c | exported | Unvalidated |
| `if (tiflags & 0x02)` | slirp/tcp_input.c | exported | Unvalidated |
| `if (tiflags & 0x10) == 0)` | slirp/tcp_input.c | exported | Unvalidated |
| `switch (tp->t_state)` | slirp/tcp_input.c | left out | still being translated |
| `step6: if ((tiflags & 0x10) && (((int)((tp->snd_wl1)-(ti->ti_t.th_seq)) < 0) || (tp->snd_wl1 ==…` | slirp/tcp_input.c | exported | Unvalidated |
| `if (tiflags & 0x20) && ti->ti_t.th_urp && ((tp->t_state) >= 10) == 0) … else` | slirp/tcp_input.c | exported | Unvalidated |
| `dodata: if ((ti->ti_i.ih_len || (tiflags&0x01)) && ((tp->t_state) >= 10) == 0) { { if ((ti)->ti…` | slirp/tcp_input.c | left out | still being translated |
| `if (tiflags & 0x01)` | slirp/tcp_input.c | exported | Unvalidated |
| `if (ti->ti_i.ih_len && (unsigned)ti->ti_i.ih_len <= 5 && ((struct tcpiphdr_2 *)ti)->first_char…` | slirp/tcp_input.c | exported | Unvalidated |
| `if (needoutput || (tp->t_flags & 0x0001)` | slirp/tcp_input.c | exported | Unvalidated |
| `return;` | slirp/tcp_input.c | exported | Unvalidated |
| `dropafterack: if (tiflags & 0x04) goto drop;` | slirp/tcp_input.c | exported | Unvalidated |
| `m_free(m); tp->t_flags |= 0x0001; (void) tcp_output(tp); return;` | slirp/tcp_input.c | exported | Unvalidated |
| `dropwithreset: if (tiflags & 0x10) tcp_respond(tp, ti, m, (tcp_seq)0, ti->ti_t.th_ack, 0x04); e…` | slirp/tcp_input.c | exported | Unvalidated |
| `return;` | slirp/tcp_input.c | exported | Unvalidated |
| `drop: m_free(m);` | slirp/tcp_input.c | exported | Unvalidated |
| `return;` | slirp/tcp_input.c | exported | Unvalidated |
| `tcp_dooptions` | slirp/tcp_input.c | left out | not translated yet |
| `uint16_t mss; int opt, optlen; ; ;` | slirp/tcp_input.c | exported | Unvalidated |
| `for (; cnt > 0; cnt -= optlen, cp += optlen)` | slirp/tcp_input.c | exported | Unvalidated |
| `tcp_xmit_timer` | slirp/tcp_input.c | left out | not translated yet |
| `register short delta; ; ; ;` | slirp/tcp_input.c | exported | Unvalidated |
| `if (tp->t_srtt != 0) … else` | slirp/tcp_input.c | exported | Unvalidated |
| `tp->t_rtt = 0; tp->t_rxtshift = 0;` | slirp/tcp_input.c | exported | Unvalidated |
| `{ … }` | slirp/tcp_input.c | exported | Unvalidated |
| `; tp->t_softerror = 0;` | slirp/tcp_input.c | exported | Unvalidated |
| `tcp_mss` | slirp/tcp_input.c | left out | not translated yet |
| `struct socket *so = tp->t_socket; int mss; ; ; ; mss = ((1500) < (1500) ? (1500) : (1500)) - si…` | slirp/tcp_input.c | exported | Unvalidated |
| `if (offer)` | slirp/tcp_input.c | exported | Unvalidated |
| `mss = ((mss) > (32) ? (mss) : (32));` | slirp/tcp_input.c | exported | Unvalidated |
| `if (mss < tp->t_maxseg || offer != 0)` | slirp/tcp_input.c | exported | Unvalidated |
| `tp->snd_cwnd = mss; sbreserve(&so->so_snd, 8192 + ((8192 % mss) ? (mss - (8192 % mss)) : 0)); s…` | slirp/tcp_input.c | exported | Unvalidated |
| `tcp_output` | slirp/tcp_output.c | left out | not translated yet |
| `register struct socket *so = tp->t_socket; register long len, win; int off, flags, error; regis…` | slirp/tcp_output.c | exported | Unvalidated |
| `if (idle && tp->t_idle >= tp->t_rxtcur)` | slirp/tcp_output.c | exported | Unvalidated |
| `again: sendalot = 0;` | slirp/tcp_output.c | exported | Unvalidated |
| `off = tp->snd_nxt - tp->snd_una; win = ((tp->snd_wnd) < (tp->snd_cwnd) ? (tp->snd_wnd) : (tp->s…` | slirp/tcp_output.c | exported | Unvalidated |
| `if (tp->t_force)` | slirp/tcp_output.c | exported | Unvalidated |
| `len = ((so->so_snd.sb_cc) < (win) ? (so->so_snd.sb_cc) : (win)) - off;` | slirp/tcp_output.c | exported | Unvalidated |
| `if (len < 0)` | slirp/tcp_output.c | exported | Unvalidated |
| `if (len > tp->t_maxseg)` | slirp/tcp_output.c | exported | Unvalidated |
| `if ((int)((tp->snd_nxt + len)-(tp->snd_una + so->so_snd.sb_cc)) < 0)` | slirp/tcp_output.c | exported | Unvalidated |
| `win = ((&so->so_rcv)->sb_datalen - (&so->so_rcv)->sb_cc);` | slirp/tcp_output.c | exported | Unvalidated |
| `if (len)` | slirp/tcp_output.c | exported | Unvalidated |
| `if (win > 0)` | slirp/tcp_output.c | exported | Unvalidated |
| `if (tp->t_flags & 0x0001)` | slirp/tcp_output.c | exported | Unvalidated |
| `if (flags & (0x02|0x04)` | slirp/tcp_output.c | exported | Unvalidated |
| `if ((int)((tp->snd_up)-(tp->snd_una)) > 0)` | slirp/tcp_output.c | exported | Unvalidated |
| `if (flags & 0x01 && ((tp->t_flags & 0x0010) == 0 || tp->snd_nxt == tp->snd_una)` | slirp/tcp_output.c | exported | Unvalidated |
| `if (so->so_snd.sb_cc && tp->t_timer[0] == 0 && tp->t_timer[1] == 0)` | slirp/tcp_output.c | exported | Unvalidated |
| `return (0);` | slirp/tcp_output.c | exported | Unvalidated |
| `send: optlen = 0;` | slirp/tcp_output.c | exported | Unvalidated |
| `hdrlen = sizeof (struct tcpiphdr);` | slirp/tcp_output.c | exported | Unvalidated |
| `if (flags & 0x02)` | slirp/tcp_output.c | exported | Unvalidated |
| `hdrlen += optlen;` | slirp/tcp_output.c | exported | Unvalidated |
| `if (len > tp->t_maxseg - optlen)` | slirp/tcp_output.c | exported | Unvalidated |
| `if (len) … else` | slirp/tcp_output.c | exported | Unvalidated |
| `ti = ((struct tcpiphdr *)(m)->m_hdr.mh_data); memcpy((caddr_t)ti, &tp->t_template, sizeof (stru…` | slirp/tcp_output.c | exported | Unvalidated |
| `if (flags & 0x01 && tp->t_flags & 0x0010 && tp->snd_nxt == tp->snd_max)` | slirp/tcp_output.c | exported | Unvalidated |
| `if (len || (flags & (0x02|0x01)) || tp->t_timer[1]) … else` | slirp/tcp_output.c | exported | Unvalidated |
| `ti->ti_t.th_ack = htonl(tp->rcv_nxt);` | slirp/tcp_output.c | exported | Unvalidated |
| `if (optlen)` | slirp/tcp_output.c | exported | Unvalidated |
| `ti->ti_t.th_flags = flags;` | slirp/tcp_output.c | exported | Unvalidated |
| `if (win < (long)(so->so_rcv.sb_datalen / 4) && win < (long)tp->t_maxseg)` | slirp/tcp_output.c | exported | Unvalidated |
| `if (win > (long)65535 << tp->rcv_scale)` | slirp/tcp_output.c | exported | Unvalidated |
| `if (win < (long)(tp->rcv_adv - tp->rcv_nxt)` | slirp/tcp_output.c | exported | Unvalidated |
| `ti->ti_t.th_win = htons((uint16_t) (win>>tp->rcv_scale));` | slirp/tcp_output.c | exported | Unvalidated |
| `if ((int)((tp->snd_up)-(tp->snd_una)) > 0) … else` | slirp/tcp_output.c | exported | Unvalidated |
| `if (len + optlen)` | slirp/tcp_output.c | exported | Unvalidated |
| `ti->ti_t.th_sum = cksum(m, (int)(hdrlen + len));` | slirp/tcp_output.c | exported | Unvalidated |
| `if (tp->t_force == 0 || tp->t_timer[1] == 0) … else` | slirp/tcp_output.c | exported | Unvalidated |
| `m->m_hdr.mh_len = hdrlen + len;` | slirp/tcp_output.c | exported | Unvalidated |
| `{ … }` | slirp/tcp_output.c | exported | Unvalidated |
| `if (error)` | slirp/tcp_output.c | exported | Unvalidated |
| `if (win > 0 && ((int)((tp->rcv_nxt+win)-(tp->rcv_adv)) > 0)` | slirp/tcp_output.c | exported | Unvalidated |
| `tp->last_ack_sent = tp->rcv_nxt; tp->t_flags &= ~(0x0001|0x0002);` | slirp/tcp_output.c | exported | Unvalidated |
| `if (sendalot)` | slirp/tcp_output.c | exported | Unvalidated |
| `return (0);` | slirp/tcp_output.c | exported | Unvalidated |
| `tcp_setpersist` | slirp/tcp_output.c | left out | not translated yet |
| `{ … }` | slirp/tcp_output.c | exported | Unvalidated |
| `;` | slirp/tcp_output.c | exported | Unvalidated |
| `if (tp->t_rxtshift < 12)` | slirp/tcp_output.c | exported | Unvalidated |
| `tcp_init` | slirp/tcp_subr.c | left out | not translated yet |
| `slirp->tcp_iss = 1; slirp->tcb.so_next = slirp->tcb.so_prev = &slirp->tcb; slirp->tcp_last_so =…` | slirp/tcp_subr.c | exported | Unvalidated |
| `tcp_template` | slirp/tcp_subr.c | left out | not translated yet |
| `struct socket *so = tp->t_socket; register struct tcpiphdr *n = &tp->t_template; n->ti_i.ih_mbu…` | slirp/tcp_subr.c | exported | Unvalidated |
| `tcp_respond` | slirp/tcp_subr.c | left out | not translated yet |
| `register int tlen; int win = 0; ; ; ; ; ; ; ;` | slirp/tcp_subr.c | exported | Unvalidated |
| `if (tp)` | slirp/tcp_subr.c | exported | Unvalidated |
| `if (m == ((void *)0) … else` | slirp/tcp_subr.c | exported | Unvalidated |
| `ti->ti_i.ih_len = htons((u_short)(sizeof (struct tcphdr) + tlen)); tlen += sizeof (struct tcpip…` | slirp/tcp_subr.c | exported | Unvalidated |
| `if (tp) … else` | slirp/tcp_subr.c | exported | Unvalidated |
| `ti->ti_t.th_urp = 0; ti->ti_t.th_sum = 0; ti->ti_t.th_sum = cksum(m, tlen); ((struct ip *)ti)->…` | slirp/tcp_subr.c | exported | Unvalidated |
| `if (flags & 0x04) … else` | slirp/tcp_subr.c | exported | Unvalidated |
| `(void) ip_output((struct socket *)0, m);` | slirp/tcp_subr.c | exported | Unvalidated |
| `tcp_newtcpcb` | slirp/tcp_subr.c | left out | not translated yet |
| `register struct tcpcb *tp; tp = (struct tcpcb *)malloc(sizeof(*tp));` | slirp/tcp_subr.c | exported | Unvalidated |
| `if (tp == ((void *)0)` | slirp/tcp_subr.c | exported | Unvalidated |
| `memset((char *) tp, 0, sizeof(struct tcpcb)); tp->seg_next = tp->seg_prev = (struct tcpiphdr*)t…` | slirp/tcp_subr.c | exported | Unvalidated |
| `{ … }` | slirp/tcp_subr.c | exported | Unvalidated |
| `; tp->snd_cwnd = 65535 << 14; tp->snd_ssthresh = 65535 << 14; tp->t_state = 0; so->so_tcpcb = t…` | slirp/tcp_subr.c | exported | Unvalidated |
| `tcp_drop` | slirp/tcp_subr.c | left out | not translated yet |
| `; ; ;` | slirp/tcp_subr.c | exported | Unvalidated |
| `if (tp->t_state) >= 3)` | slirp/tcp_subr.c | exported | Unvalidated |
| `return (tcp_close(tp));` | slirp/tcp_subr.c | exported | Unvalidated |
| `tcp_close` | slirp/tcp_subr.c | left out | not translated yet |
| `register struct tcpiphdr *t; struct socket *so = tp->t_socket; Slirp *slirp = so->slirp; regist…` | slirp/tcp_subr.c | exported | Unvalidated |
| `while (!(((struct qlink*)(((char*)(t)) - sizeof(struct qlink))) == (struct qlink*)(tp)` | slirp/tcp_subr.c | exported | Unvalidated |
| `free(tp); so->so_tcpcb = ((void *)0) ;` | slirp/tcp_subr.c | exported | Unvalidated |
| `if (so == slirp->tcp_last_so)` | slirp/tcp_subr.c | exported | Unvalidated |
| `close(so->s); sbfree(&so->so_rcv); sbfree(&so->so_snd); sofree(so); return ((struct tcpcb *)0);` | slirp/tcp_subr.c | exported | Unvalidated |
| `tcp_sockclosed` | slirp/tcp_subr.c | left out | not translated yet |
| `; ;` | slirp/tcp_subr.c | exported | Unvalidated |
| `switch (tp->t_state)` | slirp/tcp_subr.c | exported | Unvalidated |
| `if (tp)` | slirp/tcp_subr.c | exported | Unvalidated |
| `tcp_fconnect` | slirp/tcp_subr.c | left out | not translated yet |
| `Slirp *slirp = so->slirp; int ret=0; ; ;` | slirp/tcp_subr.c | exported | Unvalidated |
| `if (ret = so->s = os_socket( 2 , SOCK_STREAM ,0)) >= 0)` | slirp/tcp_subr.c | exported | Unvalidated |
| `return(ret);` | slirp/tcp_subr.c | exported | Unvalidated |
| `tcp_connect` | slirp/tcp_subr.c | left out | not translated yet |
| `Slirp *slirp = inso->slirp; struct socket *so; struct sockaddr_in addr; socklen_t addrlen = siz…` | slirp/tcp_subr.c | exported | Unvalidated |
| `if (inso->so_state & 0x200) … else` | slirp/tcp_subr.c | exported | Unvalidated |
| `(void) tcp_mss(((so)->so_tcpcb), 0);` | slirp/tcp_subr.c | exported | Unvalidated |
| `if (s = accept(inso->s,(struct sockaddr *)&addr,&addrlen)) < 0)` | slirp/tcp_subr.c | exported | Unvalidated |
| `fd_nonblock(s); opt = 1; setsockopt(s, 1 , 2 ,(char *)&opt,sizeof(int)); opt = 1; setsockopt(s,…` | slirp/tcp_subr.c | exported | Unvalidated |
| `if (so->so_faddr.s_addr == 0 || so->so_faddr.s_addr == loopback_addr.s_addr)` | slirp/tcp_subr.c | exported | Unvalidated |
| `if (inso->so_state & 0x200)` | slirp/tcp_subr.c | exported | Unvalidated |
| `so->s = s; so->so_state |= 0x2000; so->so_iptos = tcp_tos(so); tp = ((so)->so_tcpcb); tcp_templ…` | slirp/tcp_subr.c | exported | Unvalidated |
| `tcp_attach` | slirp/tcp_subr.c | left out | not translated yet |
| `if (so->so_tcpcb = tcp_newtcpcb(so)) == ((void *)0)` | slirp/tcp_subr.c | exported | Unvalidated |
| `slirp_insque(so, &so->slirp->tcb); return 0;` | slirp/tcp_subr.c | exported | Unvalidated |
| `tcp_tos` | slirp/tcp_subr.c | left out | not translated yet |
| `while (tcptos[i].tos)` | slirp/tcp_subr.c | exported | Unvalidated |
| `for (emup = tcpemu; emup; emup = emup->next)` | slirp/tcp_subr.c | exported | Unvalidated |
| `return 0;` | slirp/tcp_subr.c | exported | Unvalidated |
| `tcp_emu` | slirp/tcp_subr.c | left out | not translated yet |
| `Slirp *slirp = so->slirp; u_int n1, n2, n3, n4, n5, n6; char buff[257]; uint32_t laddr; u_int l…` | slirp/tcp_subr.c | exported | Unvalidated |
| `switch(so->so_emu) { int x, i; case 0x7: { struct socket *tmpso; struct sockaddr_in addr; sockl…` | slirp/tcp_subr.c | left out | still being translated |
| `tcp_ctl` | slirp/tcp_subr.c | left out | not translated yet |
| `Slirp *slirp = so->slirp; struct sbuf *sb = &so->so_snd; struct ex_list *ex_ptr; int do_pty; ; ;` | slirp/tcp_subr.c | exported | Unvalidated |
| `if (so->so_faddr.s_addr != slirp->vhost_addr.s_addr)` | slirp/tcp_subr.c | exported | Unvalidated |
| `sb->sb_cc = snprintf(sb->sb_wptr, sb->sb_datalen - (sb->sb_wptr - sb->sb_data), "Error: No appl…` | slirp/tcp_subr.c | exported | Unvalidated |
| `tcp_fasttimo` | slirp/tcp_timer.c | left out | not translated yet |
| `register struct socket *so; register struct tcpcb *tp; ; so = slirp->tcb.so_next;` | slirp/tcp_timer.c | exported | Unvalidated |
| `if (so)` | slirp/tcp_timer.c | exported | Unvalidated |
| `tcp_slowtimo` | slirp/tcp_timer.c | left out | not translated yet |
| `register struct socket *ip, *ipnxt; register struct tcpcb *tp; register int i; ; ip = slirp->tc…` | slirp/tcp_timer.c | exported | Unvalidated |
| `if (ip == ((void *)0)` | slirp/tcp_timer.c | exported | Unvalidated |
| `for (; ip != &slirp->tcb; ip = ipnxt)` | slirp/tcp_timer.c | exported | Unvalidated |
| `slirp->tcp_iss += (125*1024)/2; slirp->tcp_now++;` | slirp/tcp_timer.c | exported | Unvalidated |
| `tcp_canceltimers` | slirp/tcp_timer.c | left out | not translated yet |
| `for (i = 0; i < 4; i++)` | slirp/tcp_timer.c | exported | Unvalidated |
| `tcp_timers` | slirp/tcp_timer.c | left out | not translated yet |
| `register int rexmt; ;` | slirp/tcp_timer.c | exported | Unvalidated |
| `switch (timer)` | slirp/tcp_timer.c | exported | Unvalidated |
| `return (tp);` | slirp/tcp_timer.c | exported | Unvalidated |
| `udp_init` | slirp/udp.c | left out | not translated yet |
| `slirp->udb.so_next = slirp->udb.so_prev = &slirp->udb; slirp->udp_last_so = &slirp->udb;` | slirp/udp.c | exported | Unvalidated |
| `udp_input` | slirp/udp.c | left out | not translated yet |
| `Slirp *slirp = m->slirp; register struct ip *ip; register struct udphdr *uh; int len; struct ip…` | slirp/udp.c | exported | Unvalidated |
| `if (iphlen > sizeof(struct ip))` | slirp/udp.c | exported | Unvalidated |
| `ip = ((struct ip *)(m)->m_hdr.mh_data); uh = (struct udphdr *)((caddr_t)ip + iphlen); len = nto…` | slirp/udp.c | exported | Unvalidated |
| `if (ip->ip_len != len)` | slirp/udp.c | exported | Unvalidated |
| `save_ip = *ip; save_ip.ip_len+= iphlen;` | slirp/udp.c | exported | Unvalidated |
| `if (uh->uh_sum)` | slirp/udp.c | exported | Unvalidated |
| `if (ntohs(uh->uh_dport) == 67)` | slirp/udp.c | exported | Unvalidated |
| `if (slirp->restricted)` | slirp/udp.c | exported | Unvalidated |
| `so = slirp->udp_last_so;` | slirp/udp.c | exported | Unvalidated |
| `if (so->so_lport != uh->uh_sport || so->so_laddr.s_addr != ip->ip_src.s_addr)` | slirp/udp.c | exported | Unvalidated |
| `if (so == ((void *)0)` | slirp/udp.c | exported | Unvalidated |
| `so->so_faddr = ip->ip_dst; so->so_fport = uh->uh_dport; iphlen += sizeof(struct udphdr); m->m_h…` | slirp/udp.c | exported | Unvalidated |
| `if (sosendto(so,m) == -1)` | slirp/udp.c | exported | Unvalidated |
| `m_free(so->so_m); m->m_hdr.mh_len += iphlen; m->m_hdr.mh_data -= iphlen; *ip=save_ip; so->so_m=…` | slirp/udp.c | exported | Unvalidated |
| `bad: m_free(m);` | slirp/udp.c | exported | Unvalidated |
| `return;` | slirp/udp.c | exported | Unvalidated |
| `udp_output2` | slirp/udp.c | left out | not translated yet |
| `register struct udpiphdr *ui; int error = 0; ; ; ; ; ; m->m_hdr.mh_data -= sizeof(struct udpiph…` | slirp/udp.c | exported | Unvalidated |
| `if (ui->ui_u.uh_sum = cksum(m, m->m_hdr.mh_len)) == 0)` | slirp/udp.c | exported | Unvalidated |
| `((struct ip *)ui)->ip_len = m->m_hdr.mh_len; ((struct ip *)ui)->ip_ttl = 64; ((struct ip *)ui)-…` | slirp/udp.c | exported | Unvalidated |
| `udp_output` | slirp/udp.c | left out | not translated yet |
| `Slirp *slirp = so->slirp; struct sockaddr_in saddr, daddr; saddr = *addr;` | slirp/udp.c | exported | Unvalidated |
| `if (so->so_faddr.s_addr & slirp->vnetwork_mask.s_addr) == slirp->vnetwork_addr.s_addr)` | slirp/udp.c | exported | Unvalidated |
| `daddr.sin_addr = so->so_laddr; daddr.sin_port = so->so_lport; return udp_output2(so, m, &saddr,…` | slirp/udp.c | exported | Unvalidated |
| `udp_attach` | slirp/udp.c | left out | not translated yet |
| `if (so->s = os_socket( 2 , SOCK_DGRAM ,0)) != -1)` | slirp/udp.c | exported | Unvalidated |
| `return(so->s);` | slirp/udp.c | exported | Unvalidated |
| `udp_detach` | slirp/udp.c | left out | not translated yet |
| `close(so->s); sofree(so);` | slirp/udp.c | exported | Unvalidated |
| `udp_tos` | slirp/udp.c | left out | not translated yet |
| `while (udptos[i].tos)` | slirp/udp.c | exported | Unvalidated |
| `return 0;` | slirp/udp.c | exported | Unvalidated |
| `udp_listen` | slirp/udp.c | left out | not translated yet |
| `struct sockaddr_in addr; struct socket *so; socklen_t addrlen = sizeof(struct sockaddr_in), opt…` | slirp/udp.c | exported | Unvalidated |
| `if (!so)` | slirp/udp.c | exported | Unvalidated |
| `so->s = os_socket( 2 , SOCK_DGRAM ,0); so->so_expire = curtime + 240000; slirp_insque(so, &slir…` | slirp/udp.c | exported | Unvalidated |
| `if (bind(so->s,(struct sockaddr *)&addr, addrlen) < 0)` | slirp/udp.c | exported | Unvalidated |
| `setsockopt(so->s, 1 , 2 ,(char *)&opt,sizeof(int)); getsockname(so->s,(struct sockaddr *)&addr,…` | slirp/udp.c | exported | Unvalidated |
| `if (addr.sin_addr.s_addr == 0 || addr.sin_addr.s_addr == loopback_addr.s_addr) … else` | slirp/udp.c | exported | Unvalidated |
| `so->so_lport = lport; so->so_laddr.s_addr = laddr;` | slirp/udp.c | exported | Unvalidated |
| `if (flags != 0x200)` | slirp/udp.c | exported | Unvalidated |
| `so->so_state &= 0xf000; so->so_state |= 0x004 | flags; return so;` | slirp/udp.c | exported | Unvalidated |
| `clz32` | softfp.c | left out | not translated yet |
| `if (a == 0) … else` | softfp.c | exported | Unvalidated |
| `return r;` | softfp.c | exported | Unvalidated |
| `clz64` | softfp.c | left out | not translated yet |
| `if (a == 0) … else` | softfp.c | exported | Unvalidated |
| `return r;` | softfp.c | exported | Unvalidated |
| `clz128` | softfp.c | left out | not translated yet |
| `if (a == 0) … else` | softfp.c | exported | Unvalidated |
| `return r;` | softfp.c | exported | Unvalidated |
| `pack_sf32` | softfp_template.h | left out | not translated yet |
| `pack_sf64` | softfp_template.h | left out | not translated yet |
| `pack_sf128` | softfp_template.h | left out | not translated yet |
| `return ((uint32_t)a_sign << (32 - 1)) | ((uint32_t)a_exp << 23) | (a_mant & (((uint32_t)1 << 23…` | softfp_template.h | exported | Unvalidated |
| `return ((uint64_t)a_sign << (64 - 1)) | ((uint64_t)a_exp << 52) | (a_mant & (((uint64_t)1 << 52…` | softfp_template.h | exported | Unvalidated |
| `return ((uint128_t)a_sign << (128 - 1)) | ((uint128_t)a_exp << 112) | (a_mant & (((uint128_t)1…` | softfp_template.h | exported | Unvalidated |
| `unpack_sf32` | softfp_template.h | left out | not translated yet |
| `unpack_sf64` | softfp_template.h | left out | not translated yet |
| `unpack_sf128` | softfp_template.h | left out | not translated yet |
| `*pa_sign = a >> (32 - 1); *pa_exp = (a >> 23) & ((1 << 8) - 1); return a & (((uint32_t)1 << 23)…` | softfp_template.h | exported | Unvalidated |
| `*pa_sign = a >> (64 - 1); *pa_exp = (a >> 52) & ((1 << 11) - 1); return a & (((uint64_t)1 << 52…` | softfp_template.h | exported | Unvalidated |
| `*pa_sign = a >> (128 - 1); *pa_exp = (a >> 112) & ((1 << 15) - 1); return a & (((uint128_t)1 <<…` | softfp_template.h | exported | Unvalidated |
| `rshift_rnd32` | softfp_template.h | left out | not translated yet |
| `rshift_rnd64` | softfp_template.h | left out | not translated yet |
| `rshift_rnd128` | softfp_template.h | left out | not translated yet |
| `if (d != 0)` | softfp_template.h | exported | Unvalidated |
| `if (d != 0)` | softfp_template.h | exported | Unvalidated |
| `if (d != 0)` | softfp_template.h | exported | Unvalidated |
| `return a;` | softfp_template.h | exported | Unvalidated |
| `return a;` | softfp_template.h | exported | Unvalidated |
| `return a;` | softfp_template.h | exported | Unvalidated |
| `roundpack_sf32` | softfp_template.h | left out | not translated yet |
| `roundpack_sf64` | softfp_template.h | left out | not translated yet |
| `roundpack_sf128` | softfp_template.h | left out | not translated yet |
| `switch (rm)` | softfp_template.h | exported | Unvalidated |
| `switch (rm)` | softfp_template.h | exported | Unvalidated |
| `switch (rm)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp <= 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp <= 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp <= 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (rnd_bits != 0)` | softfp_template.h | exported | Unvalidated |
| `if (rnd_bits != 0)` | softfp_template.h | exported | Unvalidated |
| `if (rnd_bits != 0)` | softfp_template.h | exported | Unvalidated |
| `a_mant = (a_mant + addend) >> ((32 - 2) - 23);` | softfp_template.h | exported | Unvalidated |
| `a_mant = (a_mant + addend) >> ((64 - 2) - 52);` | softfp_template.h | exported | Unvalidated |
| `a_mant = (a_mant + addend) >> ((128 - 2) - 112);` | softfp_template.h | exported | Unvalidated |
| `if (rm == RM_RNE && rnd_bits == (1 << (((32 - 2) - 23) - 1)` | softfp_template.h | exported | Unvalidated |
| `if (rm == RM_RNE && rnd_bits == (1 << (((64 - 2) - 52) - 1)` | softfp_template.h | exported | Unvalidated |
| `if (rm == RM_RNE && rnd_bits == (1 << (((128 - 2) - 112) - 1)` | softfp_template.h | exported | Unvalidated |
| `a_exp += a_mant >> (23 + 1);` | softfp_template.h | exported | Unvalidated |
| `a_exp += a_mant >> (52 + 1);` | softfp_template.h | exported | Unvalidated |
| `a_exp += a_mant >> (112 + 1);` | softfp_template.h | exported | Unvalidated |
| `if (a_mant <= (((uint32_t)1 << 23) - 1) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_mant <= (((uint64_t)1 << 52) - 1) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_mant <= (((uint128_t)1 << 112) - 1) … else` | softfp_template.h | exported | Unvalidated |
| `return pack_sf32(a_sign, a_exp, a_mant);` | softfp_template.h | exported | Unvalidated |
| `return pack_sf64(a_sign, a_exp, a_mant);` | softfp_template.h | exported | Unvalidated |
| `return pack_sf128(a_sign, a_exp, a_mant);` | softfp_template.h | exported | Unvalidated |
| `normalize_sf32` | softfp_template.h | left out | not translated yet |
| `normalize_sf64` | softfp_template.h | left out | not translated yet |
| `normalize_sf128` | softfp_template.h | left out | not translated yet |
| `int shift; shift = clz32(a_mant) - (32 - 1 - (32 - 2)); ((void) sizeof (__assert_single_arg ( s…` | softfp_template.h | exported | Unvalidated |
| `int shift; shift = clz64(a_mant) - (64 - 1 - (64 - 2)); ((void) sizeof (__assert_single_arg ( s…` | softfp_template.h | exported | Unvalidated |
| `int shift; shift = clz128(a_mant) - (128 - 1 - (128 - 2)); ((void) sizeof (__assert_single_arg…` | softfp_template.h | exported | Unvalidated |
| `normalize2_sf32` | softfp_template.h | left out | not translated yet |
| `normalize2_sf64` | softfp_template.h | left out | not translated yet |
| `normalize2_sf128` | softfp_template.h | left out | not translated yet |
| `if (a_mant1 == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_mant1 == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_mant1 == 0) … else` | softfp_template.h | exported | Unvalidated |
| `shift = l - (32 - 1 - (32 - 2)); ((void) sizeof (__assert_single_arg ( shift >= 0 )), __extensi…` | softfp_template.h | exported | Unvalidated |
| `shift = l - (64 - 1 - (64 - 2)); ((void) sizeof (__assert_single_arg ( shift >= 0 )), __extensi…` | softfp_template.h | exported | Unvalidated |
| `shift = l - (128 - 1 - (128 - 2)); ((void) sizeof (__assert_single_arg ( shift >= 0 )), __exten…` | softfp_template.h | exported | Unvalidated |
| `if (shift == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (shift == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (shift == 0) … else` | softfp_template.h | exported | Unvalidated |
| `return roundpack_sf32(a_sign, a_exp, a_mant1, rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `return roundpack_sf64(a_sign, a_exp, a_mant1, rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `return roundpack_sf128(a_sign, a_exp, a_mant1, rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `issignan_sf32` | softfp_template.h | left out | not translated yet |
| `issignan_sf64` | softfp_template.h | left out | not translated yet |
| `issignan_sf128` | softfp_template.h | left out | not translated yet |
| `uint32_t a_exp1; uint32_t a_mant; a_exp1 = (a >> (23 - 1)) & ((1 << (8 + 1)) - 1); a_mant = a &…` | softfp_template.h | exported | Unvalidated |
| `uint32_t a_exp1; uint64_t a_mant; a_exp1 = (a >> (52 - 1)) & ((1 << (11 + 1)) - 1); a_mant = a…` | softfp_template.h | exported | Unvalidated |
| `uint32_t a_exp1; uint128_t a_mant; a_exp1 = (a >> (112 - 1)) & ((1 << (15 + 1)) - 1); a_mant =…` | softfp_template.h | exported | Unvalidated |
| `isnan_sf32` | softfp_template.h | left out | not translated yet |
| `isnan_sf64` | softfp_template.h | left out | not translated yet |
| `isnan_sf128` | softfp_template.h | left out | not translated yet |
| `uint32_t a_exp; uint32_t a_mant; a_exp = (a >> 23) & ((1 << 8) - 1); a_mant = a & (((uint32_t)1…` | softfp_template.h | exported | Unvalidated |
| `uint32_t a_exp; uint64_t a_mant; a_exp = (a >> 52) & ((1 << 11) - 1); a_mant = a & (((uint64_t)…` | softfp_template.h | exported | Unvalidated |
| `uint32_t a_exp; uint128_t a_mant; a_exp = (a >> 112) & ((1 << 15) - 1); a_mant = a & (((uint128…` | softfp_template.h | exported | Unvalidated |
| `add_sf32` | softfp_template.h | left out | not translated yet |
| `add_sf64` | softfp_template.h | left out | not translated yet |
| `add_sf128` | softfp_template.h | left out | not translated yet |
| `if (a & ~((uint32_t)1 << (32 - 1))) < (b & ~((uint32_t)1 << (32 - 1)` | softfp_template.h | exported | Unvalidated |
| `if (a & ~((uint64_t)1 << (64 - 1))) < (b & ~((uint64_t)1 << (64 - 1)` | softfp_template.h | exported | Unvalidated |
| `if (a & ~((uint128_t)1 << (128 - 1))) < (b & ~((uint128_t)1 << (128 - 1)` | softfp_template.h | exported | Unvalidated |
| `a_sign = a >> (32 - 1); b_sign = b >> (32 - 1); a_exp = (a >> 23) & ((1 << 8) - 1); b_exp = (b…` | softfp_template.h | exported | Unvalidated |
| `a_sign = a >> (64 - 1); b_sign = b >> (64 - 1); a_exp = (a >> 52) & ((1 << 11) - 1); b_exp = (b…` | softfp_template.h | exported | Unvalidated |
| `a_sign = a >> (128 - 1); b_sign = b >> (128 - 1); a_exp = (a >> 112) & ((1 << 15) - 1); b_exp =…` | softfp_template.h | exported | Unvalidated |
| `if (__builtin_expect(!!(a_exp == ((1 << 8) - 1)), 0))` | softfp_template.h | exported | Unvalidated |
| `if (__builtin_expect(!!(a_exp == ((1 << 11) - 1)), 0))` | softfp_template.h | exported | Unvalidated |
| `if (__builtin_expect(!!(a_exp == ((1 << 15) - 1)), 0))` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (b_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (b_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (b_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `b_mant = rshift_rnd32(b_mant, a_exp - b_exp);` | softfp_template.h | exported | Unvalidated |
| `b_mant = rshift_rnd64(b_mant, a_exp - b_exp);` | softfp_template.h | exported | Unvalidated |
| `b_mant = rshift_rnd128(b_mant, a_exp - b_exp);` | softfp_template.h | exported | Unvalidated |
| `if (a_sign == b_sign) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_sign == b_sign) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_sign == b_sign) … else` | softfp_template.h | exported | Unvalidated |
| `a_exp += (((32 - 2) - 23) - 3); return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `a_exp += (((64 - 2) - 52) - 3); return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `a_exp += (((128 - 2) - 112) - 3); return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `sub_sf32` | softfp_template.h | left out | not translated yet |
| `sub_sf64` | softfp_template.h | left out | not translated yet |
| `sub_sf128` | softfp_template.h | left out | not translated yet |
| `return add_sf32(a, b ^ ((uint32_t)1 << (32 - 1)), rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `return add_sf64(a, b ^ ((uint64_t)1 << (64 - 1)), rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `return add_sf128(a, b ^ ((uint128_t)1 << (128 - 1)), rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `normalize_subnormal_sf32` | softfp_template.h | left out | not translated yet |
| `normalize_subnormal_sf64` | softfp_template.h | left out | not translated yet |
| `normalize_subnormal_sf128` | softfp_template.h | left out | not translated yet |
| `int shift; shift = 23 - ((32 - 1 - clz32(a_mant))); *pa_exp = 1 - shift; return a_mant << shift;` | softfp_template.h | exported | Unvalidated |
| `int shift; shift = 52 - ((64 - 1 - clz64(a_mant))); *pa_exp = 1 - shift; return a_mant << shift;` | softfp_template.h | exported | Unvalidated |
| `int shift; shift = 112 - ((128 - 1 - clz128(a_mant))); *pa_exp = 1 - shift; return a_mant << sh…` | softfp_template.h | exported | Unvalidated |
| `mul_u32` | softfp_template.h | left out | not translated yet |
| `mul_u64` | softfp_template.h | left out | not translated yet |
| `uint64_t r; r = (uint64_t)a * (uint64_t)b; *plow = r; return r >> 32;` | softfp_template.h | exported | Unvalidated |
| `uint128_t r; r = (uint128_t)a * (uint128_t)b; *plow = r; return r >> 64;` | softfp_template.h | exported | Unvalidated |
| `mul_u128` | softfp_template.h | left out | not translated yet |
| `uint64_t a0, a1, b0, b1, r0, r1, r2, r3; uint128_t r00, r01, r10, r11, c; a0 = a; a1 = a >> (12…` | softfp_template.h | exported | Unvalidated |
| `mul_sf32` | softfp_template.h | left out | not translated yet |
| `mul_sf64` | softfp_template.h | left out | not translated yet |
| `mul_sf128` | softfp_template.h | left out | not translated yet |
| `uint32_t a_sign, b_sign, r_sign; int32_t a_exp, b_exp, r_exp; uint32_t a_mant, b_mant, r_mant,…` | softfp_template.h | exported | Unvalidated |
| `uint32_t a_sign, b_sign, r_sign; int32_t a_exp, b_exp, r_exp; uint64_t a_mant, b_mant, r_mant,…` | softfp_template.h | exported | Unvalidated |
| `uint32_t a_sign, b_sign, r_sign; int32_t a_exp, b_exp, r_exp; uint128_t a_mant, b_mant, r_mant,…` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 8) - 1) || b_exp == ((1 << 8) - 1)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 11) - 1) || b_exp == ((1 << 11) - 1)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 15) - 1) || b_exp == ((1 << 15) - 1)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (b_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (b_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (b_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `r_exp = a_exp + b_exp - (1 << (8 - 1)) + 2; r_mant = mul_u32(&r_mant_low,a_mant << ((32 - 2) -…` | softfp_template.h | exported | Unvalidated |
| `r_exp = a_exp + b_exp - (1 << (11 - 1)) + 2; r_mant = mul_u64(&r_mant_low,a_mant << ((64 - 2) -…` | softfp_template.h | exported | Unvalidated |
| `r_exp = a_exp + b_exp - (1 << (15 - 1)) + 2; r_mant = mul_u128(&r_mant_low,a_mant << ((128 - 2)…` | softfp_template.h | exported | Unvalidated |
| `fma_sf32` | softfp_template.h | left out | not translated yet |
| `fma_sf64` | softfp_template.h | left out | not translated yet |
| `fma_sf128` | softfp_template.h | left out | not translated yet |
| `uint32_t a_sign, b_sign, c_sign, r_sign; int32_t a_exp, b_exp, c_exp, r_exp, shift; uint32_t a_…` | softfp_template.h | exported | Unvalidated |
| `uint32_t a_sign, b_sign, c_sign, r_sign; int32_t a_exp, b_exp, c_exp, r_exp, shift; uint64_t a_…` | softfp_template.h | exported | Unvalidated |
| `uint32_t a_sign, b_sign, c_sign, r_sign; int32_t a_exp, b_exp, c_exp, r_exp, shift; uint128_t a…` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 8) - 1) || b_exp == ((1 << 8) - 1) || c_exp == ((1 << 8) - 1)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 11) - 1) || b_exp == ((1 << 11) - 1) || c_exp == ((1 << 11) - 1)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 15) - 1) || b_exp == ((1 << 15) - 1) || c_exp == ((1 << 15) - 1)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (b_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (b_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (b_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `r_exp = a_exp + b_exp - (1 << (8 - 1)) + 3; r_mant1 = mul_u32(&r_mant0, a_mant << ((32 - 2) - 2…` | softfp_template.h | exported | Unvalidated |
| `r_exp = a_exp + b_exp - (1 << (11 - 1)) + 3; r_mant1 = mul_u64(&r_mant0, a_mant << ((64 - 2) -…` | softfp_template.h | exported | Unvalidated |
| `r_exp = a_exp + b_exp - (1 << (15 - 1)) + 3; r_mant1 = mul_u128(&r_mant0, a_mant << ((128 - 2)…` | softfp_template.h | exported | Unvalidated |
| `if (r_mant1 < ((uint32_t)1 << (32 - 3)` | softfp_template.h | exported | Unvalidated |
| `if (r_mant1 < ((uint64_t)1 << (64 - 3)` | softfp_template.h | exported | Unvalidated |
| `if (r_mant1 < ((uint128_t)1 << (128 - 3)` | softfp_template.h | exported | Unvalidated |
| `if (c_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (c_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (c_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `c_exp++; c_mant1 = c_mant << (((32 - 2) - 23) - 1); c_mant0 = 0;` | softfp_template.h | exported | Unvalidated |
| `c_exp++; c_mant1 = c_mant << (((64 - 2) - 52) - 1); c_mant0 = 0;` | softfp_template.h | exported | Unvalidated |
| `c_exp++; c_mant1 = c_mant << (((128 - 2) - 112) - 1); c_mant0 = 0;` | softfp_template.h | exported | Unvalidated |
| `if (!(r_exp > c_exp || (r_exp == c_exp && r_mant1 >= c_mant1)` | softfp_template.h | exported | Unvalidated |
| `if (!(r_exp > c_exp || (r_exp == c_exp && r_mant1 >= c_mant1)` | softfp_template.h | exported | Unvalidated |
| `if (!(r_exp > c_exp || (r_exp == c_exp && r_mant1 >= c_mant1)` | softfp_template.h | exported | Unvalidated |
| `shift = r_exp - c_exp;` | softfp_template.h | exported | Unvalidated |
| `shift = r_exp - c_exp;` | softfp_template.h | exported | Unvalidated |
| `shift = r_exp - c_exp;` | softfp_template.h | exported | Unvalidated |
| `if (shift >= 2 * 32) … else` | softfp_template.h | exported | Unvalidated |
| `if (shift >= 2 * 64) … else` | softfp_template.h | exported | Unvalidated |
| `if (shift >= 2 * 128) … else` | softfp_template.h | exported | Unvalidated |
| `if (r_sign == c_sign) … else` | softfp_template.h | exported | Unvalidated |
| `if (r_sign == c_sign) … else` | softfp_template.h | exported | Unvalidated |
| `if (r_sign == c_sign) … else` | softfp_template.h | exported | Unvalidated |
| `return normalize2_sf32(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `return normalize2_sf64(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `return normalize2_sf128(r_sign, r_exp, r_mant1, r_mant0, rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `divrem_u32` | softfp_template.h | left out | not translated yet |
| `divrem_u64` | softfp_template.h | left out | not translated yet |
| `uint64_t a; a = ((uint64_t)ah << 32) | al; *pr = a % b; return a / b;` | softfp_template.h | exported | Unvalidated |
| `uint128_t a; a = ((uint128_t)ah << 64) | al; *pr = a % b; return a / b;` | softfp_template.h | exported | Unvalidated |
| `divrem_u128` | softfp_template.h | left out | not translated yet |
| `int i, qb, ab; ((void) sizeof (__assert_single_arg ( a1 < b )), __extension__ ({ if ( a1 < b )…` | softfp_template.h | exported | Unvalidated |
| `for (i = 0; i < 128; i++)` | softfp_template.h | exported | Unvalidated |
| `*pr = a1; return a0;` | softfp_template.h | exported | Unvalidated |
| `div_sf32` | softfp_template.h | left out | not translated yet |
| `div_sf64` | softfp_template.h | left out | not translated yet |
| `div_sf128` | softfp_template.h | left out | not translated yet |
| `uint32_t a_sign, b_sign, r_sign; int32_t a_exp, b_exp, r_exp; uint32_t a_mant, b_mant, r_mant,…` | softfp_template.h | exported | Unvalidated |
| `uint32_t a_sign, b_sign, r_sign; int32_t a_exp, b_exp, r_exp; uint64_t a_mant, b_mant, r_mant,…` | softfp_template.h | exported | Unvalidated |
| `uint32_t a_sign, b_sign, r_sign; int32_t a_exp, b_exp, r_exp; uint128_t a_mant, b_mant, r_mant,…` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 8) - 1) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 11) - 1) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 15) - 1) … else` | softfp_template.h | exported | Unvalidated |
| `if (b_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (b_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (b_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `r_exp = a_exp - b_exp + (1 << (8 - 1)) - 1; r_mant = divrem_u32(&r, a_mant, 0, b_mant << 2);` | softfp_template.h | exported | Unvalidated |
| `r_exp = a_exp - b_exp + (1 << (11 - 1)) - 1; r_mant = divrem_u64(&r, a_mant, 0, b_mant << 2);` | softfp_template.h | exported | Unvalidated |
| `r_exp = a_exp - b_exp + (1 << (15 - 1)) - 1; r_mant = divrem_u128(&r, a_mant, 0, b_mant << 2);` | softfp_template.h | exported | Unvalidated |
| `if (r != 0)` | softfp_template.h | exported | Unvalidated |
| `if (r != 0)` | softfp_template.h | exported | Unvalidated |
| `if (r != 0)` | softfp_template.h | exported | Unvalidated |
| `return normalize_sf32(r_sign, r_exp, r_mant, rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `return normalize_sf64(r_sign, r_exp, r_mant, rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `return normalize_sf128(r_sign, r_exp, r_mant, rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `sqrtrem_u32` | softfp_template.h | left out | not translated yet |
| `sqrtrem_u64` | softfp_template.h | left out | not translated yet |
| `if (ah != 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (ah != 0) … else` | softfp_template.h | exported | Unvalidated |
| `a = ((uint64_t)ah << 32) | al; u = (uint64_t)1 << ((l + 1) / 2);` | softfp_template.h | exported | Unvalidated |
| `a = ((uint128_t)ah << 64) | al; u = (uint128_t)1 << ((l + 1) / 2);` | softfp_template.h | exported | Unvalidated |
| `for (; ; )` | softfp_template.h | exported | Unvalidated |
| `for (; ; )` | softfp_template.h | exported | Unvalidated |
| `inexact = (a - s * s) != 0; *pr = s; return inexact;` | softfp_template.h | exported | Unvalidated |
| `inexact = (a - s * s) != 0; *pr = s; return inexact;` | softfp_template.h | exported | Unvalidated |
| `sqrtrem_u128` | softfp_template.h | left out | not translated yet |
| `if (a1 != 0) … else` | softfp_template.h | exported | Unvalidated |
| `u = (uint128_t)1 << ((l + 1) / 2);` | softfp_template.h | exported | Unvalidated |
| `for (; ; )` | softfp_template.h | exported | Unvalidated |
| `sq1 = mul_u128(&sq0, s, s); inexact = (sq0 != a0 || sq1 != a1); *pr = s; return inexact;` | softfp_template.h | exported | Unvalidated |
| `sqrt_sf32` | softfp_template.h | left out | not translated yet |
| `sqrt_sf64` | softfp_template.h | left out | not translated yet |
| `sqrt_sf128` | softfp_template.h | left out | not translated yet |
| `uint32_t a_sign; int32_t a_exp; uint32_t a_mant; a_sign = a >> (32 - 1); a_exp = (a >> 23) & ((…` | softfp_template.h | exported | Unvalidated |
| `uint32_t a_sign; int32_t a_exp; uint64_t a_mant; a_sign = a >> (64 - 1); a_exp = (a >> 52) & ((…` | softfp_template.h | exported | Unvalidated |
| `uint32_t a_sign; int32_t a_exp; uint128_t a_mant; a_sign = a >> (128 - 1); a_exp = (a >> 112) &…` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 8) - 1)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 11) - 1)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 15) - 1)` | softfp_template.h | exported | Unvalidated |
| `if (a_sign)` | softfp_template.h | exported | Unvalidated |
| `if (a_sign)` | softfp_template.h | exported | Unvalidated |
| `if (a_sign)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `a_exp -= ((1 << 8) - 1) / 2;` | softfp_template.h | exported | Unvalidated |
| `a_exp -= ((1 << 11) - 1) / 2;` | softfp_template.h | exported | Unvalidated |
| `a_exp -= ((1 << 15) - 1) / 2;` | softfp_template.h | exported | Unvalidated |
| `if (a_exp & 1)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp & 1)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp & 1)` | softfp_template.h | exported | Unvalidated |
| `a_exp = (a_exp >> 1) + ((1 << 8) - 1) / 2; a_mant <<= (32 - 4 - 23);` | softfp_template.h | exported | Unvalidated |
| `a_exp = (a_exp >> 1) + ((1 << 11) - 1) / 2; a_mant <<= (64 - 4 - 52);` | softfp_template.h | exported | Unvalidated |
| `a_exp = (a_exp >> 1) + ((1 << 15) - 1) / 2; a_mant <<= (128 - 4 - 112);` | softfp_template.h | exported | Unvalidated |
| `if (sqrtrem_u32(&a_mant, a_mant, 0))` | softfp_template.h | exported | Unvalidated |
| `if (sqrtrem_u64(&a_mant, a_mant, 0))` | softfp_template.h | exported | Unvalidated |
| `if (sqrtrem_u128(&a_mant, a_mant, 0))` | softfp_template.h | exported | Unvalidated |
| `return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);` | softfp_template.h | exported | Unvalidated |
| `min_sf32` | softfp_template.h | left out | not translated yet |
| `min_sf64` | softfp_template.h | left out | not translated yet |
| `min_sf128` | softfp_template.h | left out | not translated yet |
| `if (isnan_sf32(a) || isnan_sf32(b))` | softfp_template.h | exported | Unvalidated |
| `if (isnan_sf64(a) || isnan_sf64(b))` | softfp_template.h | exported | Unvalidated |
| `if (isnan_sf128(a) || isnan_sf128(b))` | softfp_template.h | exported | Unvalidated |
| `a_sign = a >> (32 - 1); b_sign = b >> (32 - 1);` | softfp_template.h | exported | Unvalidated |
| `a_sign = a >> (64 - 1); b_sign = b >> (64 - 1);` | softfp_template.h | exported | Unvalidated |
| `a_sign = a >> (128 - 1); b_sign = b >> (128 - 1);` | softfp_template.h | exported | Unvalidated |
| `if (a_sign != b_sign) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_sign != b_sign) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_sign != b_sign) … else` | softfp_template.h | exported | Unvalidated |
| `max_sf32` | softfp_template.h | left out | not translated yet |
| `max_sf64` | softfp_template.h | left out | not translated yet |
| `max_sf128` | softfp_template.h | left out | not translated yet |
| `if (isnan_sf32(a) || isnan_sf32(b))` | softfp_template.h | exported | Unvalidated |
| `if (isnan_sf64(a) || isnan_sf64(b))` | softfp_template.h | exported | Unvalidated |
| `if (isnan_sf128(a) || isnan_sf128(b))` | softfp_template.h | exported | Unvalidated |
| `a_sign = a >> (32 - 1); b_sign = b >> (32 - 1);` | softfp_template.h | exported | Unvalidated |
| `a_sign = a >> (64 - 1); b_sign = b >> (64 - 1);` | softfp_template.h | exported | Unvalidated |
| `a_sign = a >> (128 - 1); b_sign = b >> (128 - 1);` | softfp_template.h | exported | Unvalidated |
| `if (a_sign != b_sign) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_sign != b_sign) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_sign != b_sign) … else` | softfp_template.h | exported | Unvalidated |
| `eq_quiet_sf32` | softfp_template.h | left out | not translated yet |
| `eq_quiet_sf64` | softfp_template.h | left out | not translated yet |
| `eq_quiet_sf128` | softfp_template.h | left out | not translated yet |
| `if (isnan_sf32(a) || isnan_sf32(b))` | softfp_template.h | exported | Unvalidated |
| `if (isnan_sf64(a) || isnan_sf64(b))` | softfp_template.h | exported | Unvalidated |
| `if (isnan_sf128(a) || isnan_sf128(b))` | softfp_template.h | exported | Unvalidated |
| `if ((uint32_t)((a | b) << 1) == 0)` | softfp_template.h | exported | Unvalidated |
| `if ((uint64_t)((a | b) << 1) == 0)` | softfp_template.h | exported | Unvalidated |
| `if ((uint128_t)((a | b) << 1) == 0)` | softfp_template.h | exported | Unvalidated |
| `return (a == b);` | softfp_template.h | exported | Unvalidated |
| `return (a == b);` | softfp_template.h | exported | Unvalidated |
| `return (a == b);` | softfp_template.h | exported | Unvalidated |
| `le_sf32` | softfp_template.h | left out | not translated yet |
| `le_sf64` | softfp_template.h | left out | not translated yet |
| `le_sf128` | softfp_template.h | left out | not translated yet |
| `if (isnan_sf32(a) || isnan_sf32(b))` | softfp_template.h | exported | Unvalidated |
| `if (isnan_sf64(a) || isnan_sf64(b))` | softfp_template.h | exported | Unvalidated |
| `if (isnan_sf128(a) || isnan_sf128(b))` | softfp_template.h | exported | Unvalidated |
| `a_sign = a >> (32 - 1); b_sign = b >> (32 - 1);` | softfp_template.h | exported | Unvalidated |
| `a_sign = a >> (64 - 1); b_sign = b >> (64 - 1);` | softfp_template.h | exported | Unvalidated |
| `a_sign = a >> (128 - 1); b_sign = b >> (128 - 1);` | softfp_template.h | exported | Unvalidated |
| `if (a_sign != b_sign) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_sign != b_sign) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_sign != b_sign) … else` | softfp_template.h | exported | Unvalidated |
| `lt_sf32` | softfp_template.h | left out | not translated yet |
| `lt_sf64` | softfp_template.h | left out | not translated yet |
| `lt_sf128` | softfp_template.h | left out | not translated yet |
| `if (isnan_sf32(a) || isnan_sf32(b))` | softfp_template.h | exported | Unvalidated |
| `if (isnan_sf64(a) || isnan_sf64(b))` | softfp_template.h | exported | Unvalidated |
| `if (isnan_sf128(a) || isnan_sf128(b))` | softfp_template.h | exported | Unvalidated |
| `a_sign = a >> (32 - 1); b_sign = b >> (32 - 1);` | softfp_template.h | exported | Unvalidated |
| `a_sign = a >> (64 - 1); b_sign = b >> (64 - 1);` | softfp_template.h | exported | Unvalidated |
| `a_sign = a >> (128 - 1); b_sign = b >> (128 - 1);` | softfp_template.h | exported | Unvalidated |
| `if (a_sign != b_sign) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_sign != b_sign) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_sign != b_sign) … else` | softfp_template.h | exported | Unvalidated |
| `fclass_sf32` | softfp_template.h | left out | not translated yet |
| `fclass_sf64` | softfp_template.h | left out | not translated yet |
| `fclass_sf128` | softfp_template.h | left out | not translated yet |
| `uint32_t a_sign; int32_t a_exp; uint32_t a_mant; uint32_t ret; a_sign = a >> (32 - 1); a_exp =…` | softfp_template.h | exported | Unvalidated |
| `uint32_t a_sign; int32_t a_exp; uint64_t a_mant; uint32_t ret; a_sign = a >> (64 - 1); a_exp =…` | softfp_template.h | exported | Unvalidated |
| `uint32_t a_sign; int32_t a_exp; uint128_t a_mant; uint32_t ret; a_sign = a >> (128 - 1); a_exp…` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 8) - 1) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 11) - 1) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 15) - 1) … else` | softfp_template.h | exported | Unvalidated |
| `return ret;` | softfp_template.h | exported | Unvalidated |
| `return ret;` | softfp_template.h | exported | Unvalidated |
| `return ret;` | softfp_template.h | exported | Unvalidated |
| `cvt_sf32_sf64` | softfp_template.h | left out | not translated yet |
| `cvt_sf32_sf128` | softfp_template.h | left out | not translated yet |
| `uint32_t a_sign; int32_t a_exp; uint64_t a_mant; a_mant = unpack_sf32(&a_sign, &a_exp, a);` | softfp_template.h | exported | Unvalidated |
| `uint32_t a_sign; int32_t a_exp; uint128_t a_mant; a_mant = unpack_sf32(&a_sign, &a_exp, a);` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0xff)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0xff)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0)` | softfp_template.h | exported | Unvalidated |
| `a_exp = a_exp - 0x7f + (((1 << 11) - 1) / 2); a_mant <<= (52 - 23); return pack_sf64(a_sign, a_…` | softfp_template.h | exported | Unvalidated |
| `a_exp = a_exp - 0x7f + (((1 << 15) - 1) / 2); a_mant <<= (112 - 23); return pack_sf128(a_sign,…` | softfp_template.h | exported | Unvalidated |
| `cvt_sf64_sf32` | softfp_template.h | left out | not translated yet |
| `cvt_sf128_sf32` | softfp_template.h | left out | not translated yet |
| `uint32_t a_sign; int32_t a_exp; uint64_t a_mant; a_mant = unpack_sf64(&a_sign, &a_exp, a);` | softfp_template.h | exported | Unvalidated |
| `uint32_t a_sign; int32_t a_exp; uint128_t a_mant; a_mant = unpack_sf128(&a_sign, &a_exp, a);` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 11) - 1)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 15) - 1)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `a_exp = a_exp - (((1 << 11) - 1) / 2) + 0x7f; a_mant = rshift_rnd64(a_mant, 52 - (32 - 2)); ret…` | softfp_template.h | exported | Unvalidated |
| `a_exp = a_exp - (((1 << 15) - 1) / 2) + 0x7f; a_mant = rshift_rnd128(a_mant, 112 - (32 - 2)); r…` | softfp_template.h | exported | Unvalidated |
| `cvt_sf64_sf128` | softfp_template.h | left out | not translated yet |
| `uint32_t a_sign; int32_t a_exp; uint128_t a_mant; a_mant = unpack_sf64(&a_sign, &a_exp, a);` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0x7ff)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0)` | softfp_template.h | exported | Unvalidated |
| `a_exp = a_exp - 0x3ff + (((1 << 15) - 1) / 2); a_mant <<= (112 - 52); return pack_sf128(a_sign,…` | softfp_template.h | exported | Unvalidated |
| `cvt_sf128_sf64` | softfp_template.h | left out | not translated yet |
| `uint32_t a_sign; int32_t a_exp; uint128_t a_mant; a_mant = unpack_sf128(&a_sign, &a_exp, a);` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == ((1 << 15) - 1)` | softfp_template.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template.h | exported | Unvalidated |
| `a_exp = a_exp - (((1 << 15) - 1) / 2) + 0x3ff; a_mant = rshift_rnd128(a_mant, 112 - (64 - 2));…` | softfp_template.h | exported | Unvalidated |
| `internal_cvt_sf32_i32` | softfp_template_icvt.h | left out | not translated yet |
| `internal_cvt_sf32_i64` | softfp_template_icvt.h | left out | not translated yet |
| `internal_cvt_sf32_i128` | softfp_template_icvt.h | left out | not translated yet |
| `internal_cvt_sf64_i32` | softfp_template_icvt.h | left out | not translated yet |
| `internal_cvt_sf64_i64` | softfp_template_icvt.h | left out | not translated yet |
| `internal_cvt_sf64_i128` | softfp_template_icvt.h | left out | not translated yet |
| `internal_cvt_sf128_i32` | softfp_template_icvt.h | left out | not translated yet |
| `internal_cvt_sf128_i64` | softfp_template_icvt.h | left out | not translated yet |
| `internal_cvt_sf128_i128` | softfp_template_icvt.h | left out | not translated yet |
| `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint32_t a_mant; uint32_t r, r_max; a_sign =…` | softfp_template_icvt.h | exported | Unvalidated |
| `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint32_t a_mant; uint64_t r, r_max; a_sign =…` | softfp_template_icvt.h | exported | Unvalidated |
| `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint32_t a_mant; uint128_t r, r_max; a_sign =…` | softfp_template_icvt.h | exported | Unvalidated |
| `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint64_t a_mant; uint32_t r, r_max; a_sign =…` | softfp_template_icvt.h | exported | Unvalidated |
| `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint64_t a_mant; uint64_t r, r_max; a_sign =…` | softfp_template_icvt.h | exported | Unvalidated |
| `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint64_t a_mant; uint128_t r, r_max; a_sign =…` | softfp_template_icvt.h | exported | Unvalidated |
| `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint128_t a_mant; uint32_t r, r_max; a_sign =…` | softfp_template_icvt.h | exported | Unvalidated |
| `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint128_t a_mant; uint64_t r, r_max; a_sign =…` | softfp_template_icvt.h | exported | Unvalidated |
| `uint32_t a_sign, addend, rnd_bits; int32_t a_exp; uint128_t a_mant; uint128_t r, r_max; a_sign…` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == ((1 << 8) - 1) && a_mant != 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == ((1 << 8) - 1) && a_mant != 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == ((1 << 8) - 1) && a_mant != 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == ((1 << 11) - 1) && a_mant != 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == ((1 << 11) - 1) && a_mant != 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == ((1 << 11) - 1) && a_mant != 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == ((1 << 15) - 1) && a_mant != 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == ((1 << 15) - 1) && a_mant != 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == ((1 << 15) - 1) && a_mant != 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp == 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant <<= ((32 - 2) - 23); a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant <<= ((32 - 2) - 23); a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant <<= ((32 - 2) - 23); a_exp = a_exp - (((1 << 8) - 1) / 2) - 23;` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant <<= ((64 - 2) - 52); a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant <<= ((64 - 2) - 52); a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant <<= ((64 - 2) - 52); a_exp = a_exp - (((1 << 11) - 1) / 2) - 52;` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant <<= ((128 - 2) - 112); a_exp = a_exp - (((1 << 15) - 1) / 2) - 112;` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant <<= ((128 - 2) - 112); a_exp = a_exp - (((1 << 15) - 1) / 2) - 112;` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant <<= ((128 - 2) - 112); a_exp = a_exp - (((1 << 15) - 1) / 2) - 112;` | softfp_template_icvt.h | exported | Unvalidated |
| `if (is_unsigned) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (is_unsigned) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (is_unsigned) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (is_unsigned) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (is_unsigned) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (is_unsigned) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (is_unsigned) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (is_unsigned) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (is_unsigned) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp >= 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp >= 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp >= 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp >= 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp >= 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp >= 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp >= 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp >= 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_exp >= 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_sign)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_sign)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_sign)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_sign)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_sign)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_sign)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_sign)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_sign)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (a_sign)` | softfp_template_icvt.h | exported | Unvalidated |
| `return r;` | softfp_template_icvt.h | exported | Unvalidated |
| `return r;` | softfp_template_icvt.h | exported | Unvalidated |
| `return r;` | softfp_template_icvt.h | exported | Unvalidated |
| `return r;` | softfp_template_icvt.h | exported | Unvalidated |
| `return r;` | softfp_template_icvt.h | exported | Unvalidated |
| `return r;` | softfp_template_icvt.h | exported | Unvalidated |
| `return r;` | softfp_template_icvt.h | exported | Unvalidated |
| `return r;` | softfp_template_icvt.h | exported | Unvalidated |
| `return r;` | softfp_template_icvt.h | exported | Unvalidated |
| `cvt_sf32_i32` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_sf32_i64` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_sf32_i128` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_sf64_i32` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_sf64_i64` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_sf64_i128` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_sf128_i32` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_sf128_i64` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_sf128_i128` | softfp_template_icvt.h | left out | not translated yet |
| `return internal_cvt_sf32_i32(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_sf32_i64(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_sf32_i128(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_sf64_i32(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_sf64_i64(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_sf64_i128(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_sf128_i32(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_sf128_i64(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_sf128_i128(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `cvt_sf32_u32` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_sf32_u64` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_sf32_u128` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_sf64_u32` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_sf64_u64` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_sf64_u128` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_sf128_u32` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_sf128_u64` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_sf128_u128` | softfp_template_icvt.h | left out | not translated yet |
| `return internal_cvt_sf32_i32 (a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_sf32_i64 (a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_sf32_i128 (a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_sf64_i32 (a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_sf64_i64 (a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_sf64_i128 (a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_sf128_i32 (a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_sf128_i64 (a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_sf128_i128 (a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `internal_cvt_i32_sf32` | softfp_template_icvt.h | left out | not translated yet |
| `internal_cvt_i64_sf32` | softfp_template_icvt.h | left out | not translated yet |
| `internal_cvt_i128_sf32` | softfp_template_icvt.h | left out | not translated yet |
| `internal_cvt_i32_sf64` | softfp_template_icvt.h | left out | not translated yet |
| `internal_cvt_i64_sf64` | softfp_template_icvt.h | left out | not translated yet |
| `internal_cvt_i128_sf64` | softfp_template_icvt.h | left out | not translated yet |
| `internal_cvt_i32_sf128` | softfp_template_icvt.h | left out | not translated yet |
| `internal_cvt_i64_sf128` | softfp_template_icvt.h | left out | not translated yet |
| `internal_cvt_i128_sf128` | softfp_template_icvt.h | left out | not translated yet |
| `if (!is_unsigned && a < 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (!is_unsigned && a < 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (!is_unsigned && a < 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (!is_unsigned && a < 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (!is_unsigned && a < 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (!is_unsigned && a < 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (!is_unsigned && a < 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (!is_unsigned && a < 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `if (!is_unsigned && a < 0) … else` | softfp_template_icvt.h | exported | Unvalidated |
| `a_exp = (((1 << 8) - 1) / 2) + 32 - 2; l = 32 - clz32(r) - (32 - 1);` | softfp_template_icvt.h | exported | Unvalidated |
| `a_exp = (((1 << 8) - 1) / 2) + 32 - 2; l = 64 - clz64(r) - (32 - 1);` | softfp_template_icvt.h | exported | Unvalidated |
| `a_exp = (((1 << 8) - 1) / 2) + 32 - 2; l = 128 - clz128(r) - (32 - 1);` | softfp_template_icvt.h | exported | Unvalidated |
| `a_exp = (((1 << 11) - 1) / 2) + 64 - 2; l = 32 - clz32(r) - (64 - 1);` | softfp_template_icvt.h | exported | Unvalidated |
| `a_exp = (((1 << 11) - 1) / 2) + 64 - 2; l = 64 - clz64(r) - (64 - 1);` | softfp_template_icvt.h | exported | Unvalidated |
| `a_exp = (((1 << 11) - 1) / 2) + 64 - 2; l = 128 - clz128(r) - (64 - 1);` | softfp_template_icvt.h | exported | Unvalidated |
| `a_exp = (((1 << 15) - 1) / 2) + 128 - 2; l = 32 - clz32(r) - (128 - 1);` | softfp_template_icvt.h | exported | Unvalidated |
| `a_exp = (((1 << 15) - 1) / 2) + 128 - 2; l = 64 - clz64(r) - (128 - 1);` | softfp_template_icvt.h | exported | Unvalidated |
| `a_exp = (((1 << 15) - 1) / 2) + 128 - 2; l = 128 - clz128(r) - (128 - 1);` | softfp_template_icvt.h | exported | Unvalidated |
| `if (l > 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (l > 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (l > 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (l > 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (l > 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (l > 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (l > 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (l > 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `if (l > 0)` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant = r; return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant = r; return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant = r; return normalize_sf32(a_sign, a_exp, a_mant, rm, pfflags);` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant = r; return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant = r; return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant = r; return normalize_sf64(a_sign, a_exp, a_mant, rm, pfflags);` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant = r; return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant = r; return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);` | softfp_template_icvt.h | exported | Unvalidated |
| `a_mant = r; return normalize_sf128(a_sign, a_exp, a_mant, rm, pfflags);` | softfp_template_icvt.h | exported | Unvalidated |
| `cvt_i32_sf32` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_i64_sf32` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_i128_sf32` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_i32_sf64` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_i64_sf64` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_i128_sf64` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_i32_sf128` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_i64_sf128` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_i128_sf128` | softfp_template_icvt.h | left out | not translated yet |
| `return internal_cvt_i32_sf32(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_i64_sf32(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_i128_sf32(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_i32_sf64(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_i64_sf64(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_i128_sf64(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_i32_sf128(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_i64_sf128(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_i128_sf128(a, rm, pfflags, FALSE);` | softfp_template_icvt.h | exported | Unvalidated |
| `cvt_u32_sf32` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_u64_sf32` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_u128_sf32` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_u32_sf64` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_u64_sf64` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_u128_sf64` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_u32_sf128` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_u64_sf128` | softfp_template_icvt.h | left out | not translated yet |
| `cvt_u128_sf128` | softfp_template_icvt.h | left out | not translated yet |
| `return internal_cvt_i32_sf32(a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_i64_sf32(a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_i128_sf32(a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_i32_sf64(a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_i64_sf64(a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_i128_sf64(a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_i32_sf128(a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_i64_sf128(a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `return internal_cvt_i128_sf128(a, rm, pfflags, TRUE);` | softfp_template_icvt.h | exported | Unvalidated |
| `main` | splitimg.c | left out | not translated yet |
| `if (optind + 1) >= argc)` | splitimg.c | exported | Unvalidated |
| `infilename = argv[optind++]; outpath = argv[optind++]; blocksize = 256;` | splitimg.c | exported | Unvalidated |
| `if (optind < argc)` | splitimg.c | exported | Unvalidated |
| `blocksize *= 1024; buf = malloc(blocksize); f = fopen(infilename, "rb");` | splitimg.c | exported | Unvalidated |
| `if (!f)` | splitimg.c | exported | Unvalidated |
| `i = 0;` | splitimg.c | exported | Unvalidated |
| `for (; ; )` | splitimg.c | exported | Unvalidated |
| `fclose(f); printf("%d blocks\n", i); snprintf(buf1, sizeof(buf1), "%s/blk.txt", outpath); fo =…` | splitimg.c | exported | Unvalidated |
| `if (!fo)` | splitimg.c | exported | Unvalidated |
| `fprintf(fo, "{\n"); fprintf(fo, " block_size: %d,\n", blocksize / 1024); fprintf(fo, " n_block:…` | splitimg.c | exported | Unvalidated |
| `term_exit` | temu.c | left out | not translated yet |
| `tcsetattr (0, 0 , &oldtty); fcntl(0, 4 , old_fd0_flags);` | temu.c | exported | Unvalidated |
| `term_init` | temu.c | left out | not translated yet |
| `struct termios tty; memset(&tty, 0, sizeof(tty)); tcgetattr (0, &tty); oldtty = tty; old_fd0_fl…` | temu.c | exported | Unvalidated |
| `if (!allow_ctrlc)` | temu.c | exported | Unvalidated |
| `tty.c_cflag &= ~( 0000060 | 0000400 ); tty.c_cflag |= 0000060 ; tty.c_cc[ 6 ] = 1; tty.c_cc[ 5…` | temu.c | exported | Unvalidated |
| `console_write` | temu.c | left out | not translated yet |
| `fwrite(buf, 1, len, stdout ); fflush( stdout );` | temu.c | exported | Unvalidated |
| `console_read` | temu.c | left out | not translated yet |
| `if (len <= 0)` | temu.c | exported | Unvalidated |
| `ret = read(s->stdin_fd, buf, len);` | temu.c | exported | Unvalidated |
| `if (ret < 0)` | temu.c | exported | Unvalidated |
| `if (ret == 0)` | temu.c | exported | Unvalidated |
| `j = 0;` | temu.c | exported | Unvalidated |
| `for (i = 0; i < ret; i++)` | temu.c | exported | Unvalidated |
| `return j;` | temu.c | exported | Unvalidated |
| `term_resize_handler` | temu.c | left out | not translated yet |
| `if (global_stdio_device)` | temu.c | exported | Unvalidated |
| `console_get_size` | temu.c | left out | not translated yet |
| `struct winsize ws; int width, height; width = 80; height = 25;` | temu.c | exported | Unvalidated |
| `if (ioctl(s->stdin_fd, 0x5413 , &ws) == 0 && ws.ws_col >= 4 && ws.ws_row >= 4)` | temu.c | exported | Unvalidated |
| `*pw = width; *ph = height;` | temu.c | exported | Unvalidated |
| `console_init` | temu.c | left out | not translated yet |
| `CharacterDevice *dev; STDIODevice *s; struct sigaction sig; term_init(allow_ctrlc); dev = mallo…` | temu.c | exported | Unvalidated |
| `bf_get_sector_count` | temu.c | left out | not translated yet |
| `BlockDeviceFile *bf = bs->opaque; return bf->nb_sectors;` | temu.c | exported | Unvalidated |
| `bf_read_async` | temu.c | left out | not translated yet |
| `if (!bf->f)` | temu.c | exported | Unvalidated |
| `if (bf->mode == BF_MODE_SNAPSHOT) … else` | temu.c | exported | Unvalidated |
| `return 0;` | temu.c | exported | Unvalidated |
| `bf_write_async` | temu.c | left out | not translated yet |
| `switch (bf->mode)` | temu.c | exported | Unvalidated |
| `return ret;` | temu.c | exported | Unvalidated |
| `block_device_init` | temu.c | left out | not translated yet |
| `if (mode == BF_MODE_RW) … else` | temu.c | exported | Unvalidated |
| `f = fopen(filename, mode_str);` | temu.c | exported | Unvalidated |
| `if (!f)` | temu.c | exported | Unvalidated |
| `fseek(f, 0, 2 ); file_size = ftello(f); bs = mallocz(sizeof(*bs)); bf = mallocz(sizeof(*bf)); b…` | temu.c | exported | Unvalidated |
| `if (mode == BF_MODE_SNAPSHOT)` | temu.c | exported | Unvalidated |
| `bs->opaque = bf; bs->get_sector_count = bf_get_sector_count; bs->read_async = bf_read_async; bs…` | temu.c | exported | Unvalidated |
| `tun_write_packet` | temu.c | left out | not translated yet |
| `TunState *s = net->opaque; write(s->fd, buf, len);` | temu.c | exported | Unvalidated |
| `tun_select_fill` | temu.c | left out | not translated yet |
| `TunState *s = net->opaque; int net_fd = s->fd; s->select_filled = net->device_can_write_packet(…` | temu.c | exported | Unvalidated |
| `if (s->select_filled)` | temu.c | exported | Unvalidated |
| `tun_select_poll` | temu.c | left out | not translated yet |
| `if (select_ret <= 0)` | temu.c | exported | Unvalidated |
| `if (s->select_filled && (((( rfds )->fds_bits)[(( net_fd ) / (8 * (int) sizeof (__fd_mask)))] &…` | temu.c | exported | Unvalidated |
| `tun_open` | temu.c | left out | not translated yet |
| `struct ifreq ifr; int fd, ret; EthernetDevice *net; TunState *s; fd = open("/dev/net/tun", 02 );` | temu.c | exported | Unvalidated |
| `if (fd < 0)` | temu.c | exported | Unvalidated |
| `memset(&ifr, 0, sizeof(ifr)); ifr. ifr_ifru.ifru_flags = 0x0002 | 0x1000 ; pstrcpy(ifr. ifr_ifr…` | temu.c | exported | Unvalidated |
| `if (ret != 0)` | temu.c | exported | Unvalidated |
| `fcntl(fd, 4 , 04000 ); net = mallocz(sizeof(*net)); net->mac_addr[0] = 0x02; net->mac_addr[1] =…` | temu.c | exported | Unvalidated |
| `slirp_write_packet` | temu.c | left out | not translated yet |
| `Slirp *slirp_state = net->opaque; slirp_input(slirp_state, buf, len);` | temu.c | exported | Unvalidated |
| `slirp_can_output` | temu.c | left out | not translated yet |
| `EthernetDevice *net = opaque; return net->device_can_write_packet(net);` | temu.c | exported | Unvalidated |
| `slirp_output` | temu.c | left out | not translated yet |
| `EthernetDevice *net = opaque; return net->device_write_packet(net, pkt, pkt_len);` | temu.c | exported | Unvalidated |
| `slirp_select_fill1` | temu.c | left out | not translated yet |
| `Slirp *slirp_state = net->opaque; slirp_select_fill(slirp_state, pfd_max, rfds, wfds, efds);` | temu.c | exported | Unvalidated |
| `slirp_select_poll1` | temu.c | left out | not translated yet |
| `Slirp *slirp_state = net->opaque; slirp_select_poll(slirp_state, rfds, wfds, efds, (select_ret…` | temu.c | exported | Unvalidated |
| `slirp_open` | temu.c | left out | not translated yet |
| `if (slirp_state)` | temu.c | exported | Unvalidated |
| `net = mallocz(sizeof(*net)); slirp_state = slirp_init(restricted, net_addr, mask, host, vhostna…` | temu.c | exported | Unvalidated |
| `virt_machine_run` | temu.c | left out | not translated yet |
| `fd_set rfds, wfds, efds; int fd_max, ret, delay; struct timeval tv; int stdin_fd; delay = virt_…` | temu.c | exported | Unvalidated |
| `do … while (0)` | temu.c | exported | Unvalidated |
| `do … while (0)` | temu.c | exported | Unvalidated |
| `do … while (0)` | temu.c | exported | Unvalidated |
| `fd_max = -1;` | temu.c | exported | Unvalidated |
| `if (m->console_dev && virtio_console_can_write_data(m->console_dev))` | temu.c | exported | Unvalidated |
| `if (m->net)` | temu.c | exported | Unvalidated |
| `fs_net_set_fdset(&fd_max, &rfds, &wfds, &efds, &delay); tv.tv_sec = delay / 1000; tv.tv_usec =…` | temu.c | exported | Unvalidated |
| `if (m->net)` | temu.c | exported | Unvalidated |
| `if (ret > 0)` | temu.c | exported | Unvalidated |
| `sdl_refresh(m); virt_machine_interp(m, 500000);` | temu.c | exported | Unvalidated |
| `help` | temu.c | left out | not translated yet |
| `printf("temu version " "2019-02-10" ", Copyright (c) 2016-2018 Fabrice Bellard\n" "usage: temu…` | temu.c | exported | Unvalidated |
| `net_start_cb` | temu.c | left out | not translated yet |
| `net_completed = TRUE;` | temu.c | exported | Unvalidated |
| `net_poll_cb` | temu.c | left out | not translated yet |
| `return net_completed;` | temu.c | exported | Unvalidated |
| `main` | temu.c | left out | not translated yet |
| `VirtMachine *s; const char *path, *cmdline, *build_preload_file; int c, option_index, i, ram_si…` | temu.c | exported | Unvalidated |
| `for (; ; )` | temu.c | exported | Unvalidated |
| `if (optind >= argc)` | temu.c | exported | Unvalidated |
| `path = argv[optind++]; virt_machine_set_defaults(p); fs_wget_init(); virt_machine_load_config_f…` | temu.c | exported | Unvalidated |
| `if (ram_size > 0)` | temu.c | exported | Unvalidated |
| `if (accel_enable != -1)` | temu.c | exported | Unvalidated |
| `if (cmdline)` | temu.c | exported | Unvalidated |
| `for (i = 0; i < p->drive_count; i++)` | temu.c | exported | Unvalidated |
| `for (i = 0; i < p->fs_count; i++)` | temu.c | exported | Unvalidated |
| `for (i = 0; i < p->eth_count; i++)` | temu.c | left out | still being translated |
| `if (p->display_device) … else` | temu.c | exported | Unvalidated |
| `p->rtc_real_time = TRUE; s = virt_machine_init(p);` | temu.c | exported | Unvalidated |
| `if (!s)` | temu.c | exported | Unvalidated |
| `virt_machine_free_config(p);` | temu.c | exported | Unvalidated |
| `if (s->net)` | temu.c | exported | Unvalidated |
| `for (; ; )` | temu.c | exported | Unvalidated |
| `virt_machine_end(s); return 0;` | temu.c | exported | Unvalidated |
| `vga_draw_glyph8` | vga.c | left out | not translated yet |
| `uint32_t font_data, xorcol; xorcol = bgcol ^ fgcol;` | vga.c | exported | Unvalidated |
| `do … while (--h)` | vga.c | exported | Unvalidated |
| `vga_draw_glyph9` | vga.c | left out | not translated yet |
| `uint32_t font_data, xorcol, v; xorcol = bgcol ^ fgcol;` | vga.c | exported | Unvalidated |
| `do … while (--h)` | vga.c | exported | Unvalidated |
| `c6_to_8` | vga.c | left out | not translated yet |
| `int b; v &= 0x3f; b = v & 1; return (v << 2) | (b << 1) | b;` | vga.c | exported | Unvalidated |
| `update_palette16` | vga.c | left out | not translated yet |
| `int full_update, i; uint32_t v, col; full_update = 0;` | vga.c | exported | Unvalidated |
| `for (i = 0; i < 16; i++)` | vga.c | exported | Unvalidated |
| `return full_update;` | vga.c | exported | Unvalidated |
| `vga_text_refresh` | vga.c | left out | not translated yet |
| `FBDevice *fb_dev = s->fb_dev; int width, height, cwidth, cheight, cy, cx, x1, y1, width1, heigh…` | vga.c | exported | Unvalidated |
| `if (!(s->sr[1] & 0x01)` | vga.c | exported | Unvalidated |
| `width = (s->cr[0x01] + 1); height = s->cr[0x12] | ((s->cr[0x07] & 0x02) << 7) | ((s->cr[0x07] &…` | vga.c | exported | Unvalidated |
| `if (fb_dev->width < width1 || fb_dev->height < height1 || width > 132 || height > 60)` | vga.c | exported | Unvalidated |
| `if (s->last_line_offset != line_offset || s->last_start_addr != start_addr || s->last_width !=…` | vga.c | exported | Unvalidated |
| `cursor_offset = ((s->cr[0x0e] << 8) | s->cr[0x0f]) - start_addr; cursor_start = s->cr[0xa]; cur…` | vga.c | exported | Unvalidated |
| `if (cursor_offset != s->last_cursor_offset || cursor_start != s->last_cursor_start || cursor_en…` | vga.c | exported | Unvalidated |
| `ch_addr1 = 0x18000 + (start_addr * 2); cursor_offset = 0x18000 + (start_addr + cursor_offset) *…` | vga.c | exported | Unvalidated |
| `for (cy = 0; cy < height; cy++)` | vga.c | exported | Unvalidated |
| `vga_refresh` | vga.c | left out | not translated yet |
| `if (!(s->ar_index & 0x20) … else` | vga.c | exported | Unvalidated |
| `vga_ioport_read` | vga.c | left out | not translated yet |
| `if (addr >= 0x3b0 && addr <= 0x3bf && (s->msr & 0x01)) || (addr >= 0x3d0 && addr <= 0x3df && !(… … else` | vga.c | exported | Unvalidated |
| `return val;` | vga.c | exported | Unvalidated |
| `vga_ioport_write` | vga.c | left out | not translated yet |
| `if (addr >= 0x3b0 && addr <= 0x3bf && (s->msr & 0x01)) || (addr >= 0x3d0 && addr <= 0x3df && !(…` | vga.c | exported | Unvalidated |
| `switch (addr)` | vga.c | exported | Unvalidated |
| `vga_read_0x3c0` | vga.c | left out | not translated yet |
| `return vga_ioport_read(opaque, 0x3c0 + addr);` | vga.c | exported | Unvalidated |
| `vga_write_0x3c0` | vga.c | left out | not translated yet |
| `return vga_ioport_write(opaque, 0x3c0 + addr, val);` | vga.c | exported | Unvalidated |
| `vga_read_0x3b4` | vga.c | left out | not translated yet |
| `return vga_ioport_read(opaque, 0x3b4 + addr);` | vga.c | exported | Unvalidated |
| `vga_write_0x3b4` | vga.c | left out | not translated yet |
| `return vga_ioport_write(opaque, 0x3b4 + addr, val);` | vga.c | exported | Unvalidated |
| `vga_read_0x3d4` | vga.c | left out | not translated yet |
| `return vga_ioport_read(opaque, 0x3d4 + addr);` | vga.c | exported | Unvalidated |
| `vga_write_0x3d4` | vga.c | left out | not translated yet |
| `return vga_ioport_write(opaque, 0x3d4 + addr, val);` | vga.c | exported | Unvalidated |
| `vga_read_0x3ba` | vga.c | left out | not translated yet |
| `return vga_ioport_read(opaque, 0x3ba + addr);` | vga.c | exported | Unvalidated |
| `vga_write_0x3ba` | vga.c | left out | not translated yet |
| `return vga_ioport_write(opaque, 0x3ba + addr, val);` | vga.c | exported | Unvalidated |
| `vga_read_0x3da` | vga.c | left out | not translated yet |
| `return vga_ioport_read(opaque, 0x3da + addr);` | vga.c | exported | Unvalidated |
| `vga_write_0x3da` | vga.c | left out | not translated yet |
| `return vga_ioport_write(opaque, 0x3da + addr, val);` | vga.c | exported | Unvalidated |
| `vbe_write` | vga.c | left out | not translated yet |
| `if (offset == 0) … else` | vga.c | exported | Unvalidated |
| `vbe_read` | vga.c | left out | not translated yet |
| `if (offset == 0) … else` | vga.c | exported | Unvalidated |
| `return val;` | vga.c | exported | Unvalidated |
| `simplefb_bar_set` | vga.c | left out | not translated yet |
| `if (bar_num == 0) … else` | vga.c | exported | Unvalidated |
| `pci_vga_init` | vga.c | left out | not translated yet |
| `VGAState *s; PCIDevice *d; uint32_t bar_size; PhysMemoryMap *mem_map, *port_map; d = pci_regist…` | vga.c | exported | Unvalidated |
| `while (bar_size < fb_dev->fb_size)` | vga.c | exported | Unvalidated |
| `pci_register_bar(d, 0, bar_size, 0x00, s, simplefb_bar_set);` | vga.c | exported | Unvalidated |
| `if (vga_rom_size > 0)` | vga.c | exported | Unvalidated |
| `s->mem_range2 = cpu_register_ram(mem_map, 0xa0000, 0x20000, 0); s->vga_ram = s->mem_range2->phy…` | vga.c | left out | still being translated |
| `virtio_reset` | virtio.c | left out | not translated yet |
| `int i; s->status = 0; s->queue_sel = 0; s->device_features_sel = 0; s->int_status = 0;` | virtio.c | exported | Unvalidated |
| `for (i = 0; i < 8; i++)` | virtio.c | exported | Unvalidated |
| `virtio_pci_get_ram_ptr` | virtio.c | left out | not translated yet |
| `return pci_device_get_dma_ptr(s->pci_dev, paddr, is_rw);` | virtio.c | exported | Unvalidated |
| `virtio_mmio_get_ram_ptr` | virtio.c | left out | not translated yet |
| `return phys_mem_get_ram_ptr(s->mem_map, paddr, is_rw);` | virtio.c | exported | Unvalidated |
| `virtio_add_pci_capability` | virtio.c | left out | not translated yet |
| `if (cfg_type == 2) … else` | virtio.c | exported | Unvalidated |
| `memset(cap, 0, cap_len); cap[0] = 0x09; cap[2] = cap_len; cap[3] = cfg_type; cap[4] = bar; put_…` | virtio.c | exported | Unvalidated |
| `if (cfg_type == 2)` | virtio.c | exported | Unvalidated |
| `pci_add_capability(s->pci_dev, cap, cap_len);` | virtio.c | exported | Unvalidated |
| `virtio_pci_bar_set` | virtio.c | left out | not translated yet |
| `VIRTIODevice *s = opaque; phys_mem_set_addr(s->mem_range, addr, enabled);` | virtio.c | exported | Unvalidated |
| `virtio_init` | virtio.c | left out | not translated yet |
| `memset(s, 0, sizeof(*s));` | virtio.c | exported | Unvalidated |
| `if (bus->pci_bus) … else` | virtio.c | left out | still being translated |
| `s->device_id = device_id; s->vendor_id = 0xffff; s->config_space_size = config_space_size; s->d…` | virtio.c | exported | Unvalidated |
| `virtio_read16` | virtio.c | left out | not translated yet |
| `if (addr & 1)` | virtio.c | exported | Unvalidated |
| `ptr = s->get_ram_ptr(s, addr, FALSE);` | virtio.c | exported | Unvalidated |
| `if (!ptr)` | virtio.c | exported | Unvalidated |
| `return *(uint16_t *)ptr;` | virtio.c | exported | Unvalidated |
| `virtio_write16` | virtio.c | left out | not translated yet |
| `if (addr & 1)` | virtio.c | exported | Unvalidated |
| `ptr = s->get_ram_ptr(s, addr, TRUE);` | virtio.c | exported | Unvalidated |
| `if (!ptr)` | virtio.c | exported | Unvalidated |
| `*(uint16_t *)ptr = val;` | virtio.c | exported | Unvalidated |
| `virtio_write32` | virtio.c | left out | not translated yet |
| `if (addr & 3)` | virtio.c | exported | Unvalidated |
| `ptr = s->get_ram_ptr(s, addr, TRUE);` | virtio.c | exported | Unvalidated |
| `if (!ptr)` | virtio.c | exported | Unvalidated |
| `*(uint32_t *)ptr = val;` | virtio.c | exported | Unvalidated |
| `virtio_memcpy_from_ram` | virtio.c | left out | not translated yet |
| `while (count > 0)` | virtio.c | exported | Unvalidated |
| `return 0;` | virtio.c | exported | Unvalidated |
| `virtio_memcpy_to_ram` | virtio.c | left out | not translated yet |
| `while (count > 0)` | virtio.c | exported | Unvalidated |
| `return 0;` | virtio.c | exported | Unvalidated |
| `get_desc` | virtio.c | left out | not translated yet |
| `QueueState *qs = &s->queue[queue_idx]; return virtio_memcpy_from_ram(s, (void *)desc, qs->desc_…` | virtio.c | exported | Unvalidated |
| `memcpy_to_from_queue` | virtio.c | left out | not translated yet |
| `if (count == 0)` | virtio.c | exported | Unvalidated |
| `get_desc(s, &desc, queue_idx, desc_idx);` | virtio.c | exported | Unvalidated |
| `if (to_queue) … else` | virtio.c | exported | Unvalidated |
| `for (; ; )` | virtio.c | exported | Unvalidated |
| `for (; ; )` | virtio.c | left out | still being translated |
| `return 0;` | virtio.c | exported | Unvalidated |
| `memcpy_from_queue` | virtio.c | left out | not translated yet |
| `return memcpy_to_from_queue(s, buf, queue_idx, desc_idx, offset, count, FALSE);` | virtio.c | exported | Unvalidated |
| `memcpy_to_queue` | virtio.c | left out | not translated yet |
| `return memcpy_to_from_queue(s, (void *)buf, queue_idx, desc_idx, offset, count, TRUE);` | virtio.c | exported | Unvalidated |
| `virtio_consume_desc` | virtio.c | left out | not translated yet |
| `QueueState *qs = &s->queue[queue_idx]; virtio_phys_addr_t addr; uint32_t index; addr = qs->used…` | virtio.c | exported | Unvalidated |
| `get_desc_rw_size` | virtio.c | left out | not translated yet |
| `VIRTIODesc desc; int read_size, write_size; read_size = 0; write_size = 0; get_desc(s, &desc, q…` | virtio.c | exported | Unvalidated |
| `for (; ; )` | virtio.c | exported | Unvalidated |
| `for (; ; )` | virtio.c | exported | Unvalidated |
| `done: *pread_size = read_size;` | virtio.c | exported | Unvalidated |
| `*pwrite_size = write_size; return 0;` | virtio.c | exported | Unvalidated |
| `queue_notify` | virtio.c | left out | not translated yet |
| `if (qs->manual_recv)` | virtio.c | exported | Unvalidated |
| `avail_idx = virtio_read16(s, qs->avail_addr + 2);` | virtio.c | exported | Unvalidated |
| `while (qs->last_avail_idx != avail_idx)` | virtio.c | exported | Unvalidated |
| `virtio_config_read` | virtio.c | left out | not translated yet |
| `switch (size_log2)` | virtio.c | exported | Unvalidated |
| `return val;` | virtio.c | exported | Unvalidated |
| `virtio_config_write` | virtio.c | left out | not translated yet |
| `switch (size_log2)` | virtio.c | exported | Unvalidated |
| `virtio_mmio_read` | virtio.c | left out | not translated yet |
| `if (offset >= 0x100)` | virtio.c | exported | Unvalidated |
| `if (size_log2 == 2) … else` | virtio.c | exported | Unvalidated |
| `return val;` | virtio.c | exported | Unvalidated |
| `set_low32` | virtio.c | left out | not translated yet |
| `*paddr = (*paddr & ~(virtio_phys_addr_t)0xffffffff) | val;` | virtio.c | exported | Unvalidated |
| `set_high32` | virtio.c | left out | not translated yet |
| `*paddr = (*paddr & 0xffffffff) | ((virtio_phys_addr_t)val << 32);` | virtio.c | exported | Unvalidated |
| `virtio_mmio_write` | virtio.c | left out | not translated yet |
| `if (offset >= 0x100)` | virtio.c | exported | Unvalidated |
| `if (size_log2 == 2)` | virtio.c | exported | Unvalidated |
| `virtio_pci_read` | virtio.c | left out | not translated yet |
| `VIRTIODevice *s = opaque; uint32_t offset; uint32_t val = 0; offset = offset1 & 0xfff;` | virtio.c | exported | Unvalidated |
| `switch (offset1 >> 12)` | virtio.c | exported | Unvalidated |
| `return val;` | virtio.c | exported | Unvalidated |
| `virtio_pci_write` | virtio.c | left out | not translated yet |
| `VIRTIODevice *s = opaque; uint32_t offset; offset = offset1 & 0xfff;` | virtio.c | exported | Unvalidated |
| `switch (offset1 >> 12)` | virtio.c | left out | still being translated |
| `virtio_set_debug` | virtio.c | left out | not translated yet |
| `s->debug = debug;` | virtio.c | exported | Unvalidated |
| `virtio_config_change_notify` | virtio.c | left out | not translated yet |
| `s->int_status |= 2; set_irq(s->irq, 1);` | virtio.c | exported | Unvalidated |
| `virtio_block_req_end` | virtio.c | left out | not translated yet |
| `switch (s1->req.type)` | virtio.c | left out | still being translated |
| `virtio_block_req_cb` | virtio.c | left out | not translated yet |
| `VIRTIODevice *s = opaque; VIRTIOBlockDevice *s1 = (VIRTIOBlockDevice *)s; virtio_block_req_end(…` | virtio.c | exported | Unvalidated |
| `virtio_block_recv_request` | virtio.c | left out | not translated yet |
| `if (s1->req_in_progress)` | virtio.c | exported | Unvalidated |
| `if (memcpy_from_queue(s, &h, queue_idx, desc_idx, 0, sizeof(h)) < 0)` | virtio.c | exported | Unvalidated |
| `s1->req.type = h.type; s1->req.queue_idx = queue_idx; s1->req.desc_idx = desc_idx;` | virtio.c | exported | Unvalidated |
| `switch (h.type)` | virtio.c | left out | still being translated |
| `return 0;` | virtio.c | exported | Unvalidated |
| `virtio_block_init` | virtio.c | left out | not translated yet |
| `VIRTIOBlockDevice *s; uint64_t nb_sectors; s = mallocz(sizeof(*s)); virtio_init(&s->common, bus…` | virtio.c | left out | still being translated |
| `virtio_net_recv_request` | virtio.c | left out | not translated yet |
| `if (queue_idx == 1)` | virtio.c | exported | Unvalidated |
| `return 0;` | virtio.c | exported | Unvalidated |
| `virtio_net_can_write_packet` | virtio.c | left out | not translated yet |
| `if (!qs->ready)` | virtio.c | exported | Unvalidated |
| `avail_idx = virtio_read16(s, qs->avail_addr + 2); return qs->last_avail_idx != avail_idx;` | virtio.c | exported | Unvalidated |
| `virtio_net_write_packet` | virtio.c | left out | not translated yet |
| `if (!qs->ready)` | virtio.c | exported | Unvalidated |
| `avail_idx = virtio_read16(s, qs->avail_addr + 2);` | virtio.c | exported | Unvalidated |
| `if (qs->last_avail_idx == avail_idx)` | virtio.c | exported | Unvalidated |
| `desc_idx = virtio_read16(s, qs->avail_addr + 4 + (qs->last_avail_idx & (qs->num - 1)) * 2);` | virtio.c | exported | Unvalidated |
| `if (get_desc_rw_size(s, &read_size, &write_size, queue_idx, desc_idx))` | virtio.c | exported | Unvalidated |
| `len = s1->header_size + buf_len;` | virtio.c | exported | Unvalidated |
| `if (len > write_size)` | virtio.c | exported | Unvalidated |
| `memset(&h, 0, s1->header_size); memcpy_to_queue(s, queue_idx, desc_idx, 0, &h, s1->header_size)…` | virtio.c | left out | still being translated |
| `virtio_net_set_carrier` | virtio.c | left out | not translated yet |
| `virtio_net_init` | virtio.c | left out | not translated yet |
| `VIRTIONetDevice *s; s = mallocz(sizeof(*s)); virtio_init(&s->common, bus, 1, 6 + 2, virtio_net_…` | virtio.c | left out | still being translated |
| `virtio_console_recv_request` | virtio.c | left out | not translated yet |
| `if (queue_idx == 1)` | virtio.c | exported | Unvalidated |
| `return 0;` | virtio.c | exported | Unvalidated |
| `virtio_console_can_write_data` | virtio.c | left out | not translated yet |
| `if (!qs->ready)` | virtio.c | exported | Unvalidated |
| `avail_idx = virtio_read16(s, qs->avail_addr + 2); return qs->last_avail_idx != avail_idx;` | virtio.c | exported | Unvalidated |
| `virtio_console_get_write_len` | virtio.c | left out | not translated yet |
| `if (!qs->ready)` | virtio.c | exported | Unvalidated |
| `avail_idx = virtio_read16(s, qs->avail_addr + 2);` | virtio.c | exported | Unvalidated |
| `if (qs->last_avail_idx == avail_idx)` | virtio.c | exported | Unvalidated |
| `desc_idx = virtio_read16(s, qs->avail_addr + 4 + (qs->last_avail_idx & (qs->num - 1)) * 2);` | virtio.c | exported | Unvalidated |
| `if (get_desc_rw_size(s, &read_size, &write_size, queue_idx, desc_idx))` | virtio.c | exported | Unvalidated |
| `return write_size;` | virtio.c | exported | Unvalidated |
| `virtio_console_write_data` | virtio.c | left out | not translated yet |
| `if (!qs->ready)` | virtio.c | exported | Unvalidated |
| `avail_idx = virtio_read16(s, qs->avail_addr + 2);` | virtio.c | exported | Unvalidated |
| `if (qs->last_avail_idx == avail_idx)` | virtio.c | exported | Unvalidated |
| `desc_idx = virtio_read16(s, qs->avail_addr + 4 + (qs->last_avail_idx & (qs->num - 1)) * 2); mem…` | virtio.c | left out | still being translated |
| `virtio_console_resize_event` | virtio.c | left out | not translated yet |
| `put_le16(s->config_space + 0, width); put_le16(s->config_space + 2, height); virtio_config_chan…` | virtio.c | exported | Unvalidated |
| `virtio_console_init` | virtio.c | left out | not translated yet |
| `VIRTIOConsoleDevice *s; s = mallocz(sizeof(*s)); virtio_init(&s->common, bus, 3, 4, virtio_cons…` | virtio.c | left out | still being translated |
| `virtio_input_recv_request` | virtio.c | left out | not translated yet |
| `if (queue_idx == 1)` | virtio.c | exported | Unvalidated |
| `return 0;` | virtio.c | exported | Unvalidated |
| `virtio_input_queue_event` | virtio.c | left out | not translated yet |
| `if (!qs->ready)` | virtio.c | exported | Unvalidated |
| `put_le16(buf, type); put_le16(buf + 2, code); put_le32(buf + 4, value); buf_len = 8; avail_idx…` | virtio.c | exported | Unvalidated |
| `if (qs->last_avail_idx == avail_idx)` | virtio.c | exported | Unvalidated |
| `desc_idx = virtio_read16(s, qs->avail_addr + 4 + (qs->last_avail_idx & (qs->num - 1)) * 2); mem…` | virtio.c | exported | Unvalidated |
| `virtio_input_send_key_event` | virtio.c | left out | not translated yet |
| `if (s1->type != VIRTIO_INPUT_TYPE_KEYBOARD)` | virtio.c | exported | Unvalidated |
| `ret = virtio_input_queue_event(s, 0x01, key_code, is_down);` | virtio.c | left out | still being translated |
| `if (ret)` | virtio.c | exported | Unvalidated |
| `return virtio_input_queue_event(s, 0x00, 0, 0);` | virtio.c | exported | Unvalidated |
| `virtio_input_send_mouse_event` | virtio.c | left out | not translated yet |
| `if (s1->type != VIRTIO_INPUT_TYPE_MOUSE && s1->type != VIRTIO_INPUT_TYPE_TABLET)` | virtio.c | exported | Unvalidated |
| `if (s1->type == VIRTIO_INPUT_TYPE_MOUSE) … else` | virtio.c | exported | Unvalidated |
| `if (dz != 0)` | virtio.c | exported | Unvalidated |
| `if (buttons != s1->buttons_state)` | virtio.c | left out | still being translated |
| `return virtio_input_queue_event(s, 0x00, 0, 0);` | virtio.c | left out | still being translated |
| `set_bit` | virtio.c | left out | not translated yet |
| `tab[k >> 3] |= 1 << (k & 7);` | virtio.c | exported | Unvalidated |
| `virtio_input_config_write` | virtio.c | left out | not translated yet |
| `switch (config[0])` | virtio.c | exported | Unvalidated |
| `virtio_input_init` | virtio.c | left out | not translated yet |
| `VIRTIOInputDevice *s; s = mallocz(sizeof(*s)); virtio_init(&s->common, bus, 18, 256, virtio_inp…` | virtio.c | left out | still being translated |
| `fid_find1` | virtio.c | left out | not translated yet |
| `for (el = (&s->fid_list)->next; el != (&s->fid_list; el = el->next)` | virtio.c | exported | Unvalidated |
| `return ((void *)0) ;` | virtio.c | exported | Unvalidated |
| `fid_find` | virtio.c | left out | not translated yet |
| `FIDDesc *f; f = fid_find1(s, fid);` | virtio.c | exported | Unvalidated |
| `if (!f)` | virtio.c | exported | Unvalidated |
| `return f->fd;` | virtio.c | exported | Unvalidated |
| `fid_delete` | virtio.c | left out | not translated yet |
| `FIDDesc *f; f = fid_find1(s, fid);` | virtio.c | exported | Unvalidated |
| `if (f)` | virtio.c | exported | Unvalidated |
| `fid_set` | virtio.c | left out | not translated yet |
| `FIDDesc *f; f = fid_find1(s, fid);` | virtio.c | exported | Unvalidated |
| `if (f) … else` | virtio.c | exported | Unvalidated |
| `virtio_9p_send_reply` | virtio.c | left out | not translated yet |
| `uint8_t *buf1; int len; len = buf_len + 7; buf1 = malloc(len); put_le32(buf1, len); buf1[4] = i…` | virtio.c | left out | still being translated |
| `virtio_9p_send_error` | virtio.c | left out | not translated yet |
| `uint8_t buf[4]; int buf_len; buf_len = marshall(s, buf, sizeof(buf), "w", -error); virtio_9p_se…` | virtio.c | left out | still being translated |
| `virtio_9p_open_reply` | virtio.c | left out | not translated yet |
| `if (err < 0) … else` | virtio.c | left out | still being translated |
| `free(oi);` | virtio.c | exported | Unvalidated |
| `virtio_9p_open_cb` | virtio.c | left out | not translated yet |
| `P9OpenInfo *oi = opaque; VIRTIO9PDevice *s = oi->dev; int queue_idx = oi->queue_idx; virtio_9p_…` | virtio.c | left out | still being translated |
| `virtio_9p_recv_request` | virtio.c | left out | not translated yet |
| `if (queue_idx != 0)` | virtio.c | exported | Unvalidated |
| `if (s->req_in_progress)` | virtio.c | exported | Unvalidated |
| `offset = 0; header_len = 4 + 1 + 2;` | virtio.c | exported | Unvalidated |
| `if (memcpy_from_queue(s1, buf, queue_idx, desc_idx, offset, header_len))` | virtio.c | left out | still being translated |
| `id = buf[4]; tag = get_le16(buf + 5); offset += header_len;` | virtio.c | exported | Unvalidated |
| `switch (id)` | virtio.c | left out | still being translated |
| `return 0;` | virtio.c | exported | Unvalidated |
| `error: virtio_9p_send_error(s, queue_idx, desc_idx, tag, err);` | virtio.c | left out | still being translated |
| `return 0;` | virtio.c | exported | Unvalidated |
| `protocol_error: fid_not_found: err = -71;` | virtio.c | exported | Unvalidated |
| `goto error;` | virtio.c | exported | Unvalidated |
| `virtio_9p_init` | virtio.c | left out | not translated yet |
| `VIRTIO9PDevice *s; int len; uint8_t *cfg; len = strlen(mount_tag); s = mallocz(sizeof(*s)); vir…` | virtio.c | left out | still being translated |
| `put_queue` | vmmouse.c | left out | not translated yet |
| `if (s->fifo_count >= (4 * 16)` | vmmouse.c | exported | Unvalidated |
| `s->fifo_buf[s->fifo_windex] = val;` | vmmouse.c | exported | Unvalidated |
| `if (++s->fifo_windex == (4 * 16)` | vmmouse.c | exported | Unvalidated |
| `s->fifo_count++;` | vmmouse.c | exported | Unvalidated |
| `read_data` | vmmouse.c | left out | not translated yet |
| `if (size > 6 || size > s->fifo_count)` | vmmouse.c | exported | Unvalidated |
| `for (i = 0; i < size; i++)` | vmmouse.c | exported | Unvalidated |
| `s->fifo_count -= size;` | vmmouse.c | exported | Unvalidated |
| `vmmouse_send_mouse_event` | vmmouse.c | left out | not translated yet |
| `if (!s->enabled)` | vmmouse.c | exported | Unvalidated |
| `if (s->fifo_count + 4) > (4 * 16)` | vmmouse.c | exported | Unvalidated |
| `state = 0;` | vmmouse.c | exported | Unvalidated |
| `if (buttons & 1)` | vmmouse.c | exported | Unvalidated |
| `if (buttons & 2)` | vmmouse.c | exported | Unvalidated |
| `if (buttons & 4)` | vmmouse.c | exported | Unvalidated |
| `if (s->absolute)` | vmmouse.c | exported | Unvalidated |
| `put_queue(s, state); put_queue(s, x); put_queue(s, y); put_queue(s, -dz); ps2_mouse_event(s->ps…` | vmmouse.c | exported | Unvalidated |
| `vmmouse_handler` | vmmouse.c | left out | not translated yet |
| `uint32_t cmd; cmd = regs[2] & 0xff;` | vmmouse.c | exported | Unvalidated |
| `switch (cmd)` | vmmouse.c | exported | Unvalidated |
| `vmmouse_is_absolute` | vmmouse.c | left out | not translated yet |
| `return s->absolute;` | vmmouse.c | exported | Unvalidated |
| `vmmouse_init` | vmmouse.c | left out | not translated yet |
| `VMMouseState *s; s = mallocz(sizeof(*s)); s->ps2_mouse = ps2_mouse; return s;` | vmmouse.c | exported | Unvalidated |
| `x86_cpu_init` | x86_cpu.c | left out | not translated yet |
| `fprintf( stderr , "x86 emulator is not supported\n"); exit(1);` | x86_cpu.c | exported | Unvalidated |
| `x86_cpu_end` | x86_cpu.c | left out | not translated yet |
| `x86_cpu_interp` | x86_cpu.c | left out | not translated yet |
| `x86_cpu_set_irq` | x86_cpu.c | left out | not translated yet |
| `x86_cpu_set_reg` | x86_cpu.c | left out | not translated yet |
| `x86_cpu_get_reg` | x86_cpu.c | left out | not translated yet |
| `return 0;` | x86_cpu.c | exported | Unvalidated |
| `x86_cpu_set_seg` | x86_cpu.c | left out | not translated yet |
| `x86_cpu_set_get_hard_intno` | x86_cpu.c | left out | not translated yet |
| `x86_cpu_set_get_tsc` | x86_cpu.c | left out | not translated yet |
| `x86_cpu_set_port_io` | x86_cpu.c | left out | not translated yet |
| `x86_cpu_get_cycles` | x86_cpu.c | left out | not translated yet |
| `return 0;` | x86_cpu.c | exported | Unvalidated |
| `x86_cpu_get_power_down` | x86_cpu.c | left out | not translated yet |
| `return FALSE;` | x86_cpu.c | exported | Unvalidated |
| `x86_cpu_flush_tlb_write_range_ram` | x86_cpu.c | left out | not translated yet |
| `to_bcd` | x86_machine.c | left out | not translated yet |
| `if (s->cmos_data[11] & 0x04) … else` | x86_machine.c | exported | Unvalidated |
| `cmos_update_time` | x86_machine.c | left out | not translated yet |
| `struct timeval tv; struct tm tm; time_t ti; int val; gettimeofday(&tv, ((void *)0) ); ti = tv.t…` | x86_machine.c | exported | Unvalidated |
| `if (s->use_local_time) … else` | x86_machine.c | exported | Unvalidated |
| `s->cmos_data[0] = to_bcd(s, tm.tm_sec); s->cmos_data[2] = to_bcd(s, tm.tm_min);` | x86_machine.c | exported | Unvalidated |
| `if (s->cmos_data[11] & 0x02) … else` | x86_machine.c | exported | Unvalidated |
| `s->cmos_data[6] = to_bcd(s, tm.tm_wday); s->cmos_data[7] = to_bcd(s, tm.tm_mday); s->cmos_data[…` | x86_machine.c | exported | Unvalidated |
| `if (set_century)` | x86_machine.c | exported | Unvalidated |
| `if (tv.tv_usec < 244) … else` | x86_machine.c | exported | Unvalidated |
| `cmos_init` | x86_machine.c | left out | not translated yet |
| `CMOSState *s; s = mallocz(sizeof(*s)); s->use_local_time = use_local_time; s->cmos_index = 0; s…` | x86_machine.c | left out | still being translated |
| `cmos_get_timer` | x86_machine.c | left out | not translated yet |
| `struct timespec ts; clock_gettime( 1 , &ts); return (uint32_t)ts.tv_sec * 32768 + ((uint64_t)ts…` | x86_machine.c | exported | Unvalidated |
| `cmos_update_timer` | x86_machine.c | left out | not translated yet |
| `int period_code; period_code = s->cmos_data[10] & 0x0f;` | x86_machine.c | exported | Unvalidated |
| `if (s->cmos_data[11] & 0x40) && period_code != 0)` | x86_machine.c | exported | Unvalidated |
| `cmos_update_irq` | x86_machine.c | left out | not translated yet |
| `if (s->cmos_data[11] & 0x40)` | x86_machine.c | exported | Unvalidated |
| `cmos_write` | x86_machine.c | left out | not translated yet |
| `if (offset == 0) … else` | x86_machine.c | exported | Unvalidated |
| `cmos_read` | x86_machine.c | left out | not translated yet |
| `if (offset == 0) … else` | x86_machine.c | left out | still being translated |
| `pic_init` | x86_machine.c | left out | not translated yet |
| `PICState *s; s = mallocz(sizeof(*s)); s->elcr_mask = elcr_mask; s->update_irq = update_irq; s->…` | x86_machine.c | left out | still being translated |
| `pic_reset` | x86_machine.c | left out | not translated yet |
| `s->last_irr = 0; s->irr = 0; s->imr = 0; s->isr = 0; s->priority_add = 0; s->irq_base = 0; s->r…` | x86_machine.c | exported | Unvalidated |
| `pic_set_irq1` | x86_machine.c | left out | not translated yet |
| `int mask; mask = 1 << irq;` | x86_machine.c | exported | Unvalidated |
| `if (s->elcr & mask) … else` | x86_machine.c | exported | Unvalidated |
| `pic_get_priority` | x86_machine.c | left out | not translated yet |
| `if (mask == 0)` | x86_machine.c | exported | Unvalidated |
| `priority = 7;` | x86_machine.c | exported | Unvalidated |
| `while (mask & (1 << ((priority + s->priority_add) & 7))) == 0)` | x86_machine.c | exported | Unvalidated |
| `return priority;` | x86_machine.c | exported | Unvalidated |
| `pic_get_irq` | x86_machine.c | left out | not translated yet |
| `int mask, cur_priority, priority; mask = s->irr & ~s->imr; priority = pic_get_priority(s, mask);` | x86_machine.c | exported | Unvalidated |
| `if (priority < 0)` | x86_machine.c | exported | Unvalidated |
| `cur_priority = pic_get_priority(s, s->isr);` | x86_machine.c | exported | Unvalidated |
| `if (priority > cur_priority) … else` | x86_machine.c | exported | Unvalidated |
| `pic_intack` | x86_machine.c | left out | not translated yet |
| `if (s->auto_eoi) … else` | x86_machine.c | exported | Unvalidated |
| `if (!(s->elcr & (1 << irq)` | x86_machine.c | exported | Unvalidated |
| `pic_write` | x86_machine.c | left out | not translated yet |
| `PICState *s = opaque; int priority, addr; addr = offset & 1;` | x86_machine.c | exported | Unvalidated |
| `if (addr == 0) … else` | x86_machine.c | left out | still being translated |
| `pic_read` | x86_machine.c | left out | not translated yet |
| `PICState *s = opaque; int addr, ret; addr = offset & 1;` | x86_machine.c | exported | Unvalidated |
| `if (addr == 0) … else` | x86_machine.c | exported | Unvalidated |
| `return ret;` | x86_machine.c | exported | Unvalidated |
| `pic_elcr_write` | x86_machine.c | left out | not translated yet |
| `PICState *s = opaque; s->elcr = val & s->elcr_mask;` | x86_machine.c | exported | Unvalidated |
| `pic_elcr_read` | x86_machine.c | left out | not translated yet |
| `PICState *s = opaque; return s->elcr;` | x86_machine.c | exported | Unvalidated |
| `pic2_init` | x86_machine.c | left out | not translated yet |
| `PIC2State *s; int i; s = mallocz(sizeof(*s));` | x86_machine.c | exported | Unvalidated |
| `for (i = 0; i < 16; i++)` | x86_machine.c | left out | still being translated |
| `s->cpu_set_irq = cpu_set_irq; s->opaque = opaque; s->pics[0] = pic_init(port_map, addr0, elcr_a…` | x86_machine.c | left out | still being translated |
| `pic2_set_elcr` | x86_machine.c | left out | not translated yet |
| `for (i = 0; i < 2; i++)` | x86_machine.c | exported | Unvalidated |
| `pic2_update_irq` | x86_machine.c | left out | not translated yet |
| `PIC2State *s = opaque; int irq2, irq; irq2 = pic_get_irq(s->pics[1]);` | x86_machine.c | left out | still being translated |
| `if (irq2 >= 0)` | x86_machine.c | exported | Unvalidated |
| `irq = pic_get_irq(s->pics[0]);` | x86_machine.c | exported | Unvalidated |
| `if (irq >= 0) … else` | x86_machine.c | exported | Unvalidated |
| `pic2_set_irq` | x86_machine.c | left out | not translated yet |
| `PIC2State *s = opaque; pic_set_irq1(s->pics[irq >> 3], irq & 7, level); pic2_update_irq(s);` | x86_machine.c | exported | Unvalidated |
| `pic2_get_hard_intno` | x86_machine.c | left out | not translated yet |
| `int irq, irq2, intno; irq = pic_get_irq(s->pics[0]);` | x86_machine.c | exported | Unvalidated |
| `if (irq >= 0) … else` | x86_machine.c | exported | Unvalidated |
| `pic2_update_irq(s); return intno;` | x86_machine.c | left out | still being translated |
| `pit_init` | x86_machine.c | left out | not translated yet |
| `PITState *s; PITChannel *pc; int i; s = mallocz(sizeof(*s)); s->irq = irq; s->get_ticks = get_t…` | x86_machine.c | exported | Unvalidated |
| `for (i = 0; i < 3; i++)` | x86_machine.c | left out | still being translated |
| `s->speaker_data_on = 0; cpu_register_device(port_map, addr0, 4, s, pit_read, pit_write, (1 << 0…` | x86_machine.c | left out | still being translated |
| `pit_get_time` | x86_machine.c | left out | not translated yet |
| `PITState *s = pc->pit_state; return s->get_ticks(s->opaque);` | x86_machine.c | exported | Unvalidated |
| `pit_get_count` | x86_machine.c | left out | not translated yet |
| `uint32_t counter; uint64_t d; d = pit_get_time(pc) - pc->count_load_time;` | x86_machine.c | left out | still being translated |
| `switch (pc->mode)` | x86_machine.c | exported | Unvalidated |
| `return counter;` | x86_machine.c | exported | Unvalidated |
| `pit_get_out` | x86_machine.c | left out | not translated yet |
| `int out; int64_t d; d = pit_get_time(pc) - pc->count_load_time;` | x86_machine.c | exported | Unvalidated |
| `switch (pc->mode)` | x86_machine.c | exported | Unvalidated |
| `return out;` | x86_machine.c | exported | Unvalidated |
| `pit_load_count` | x86_machine.c | left out | not translated yet |
| `if (val == 0)` | x86_machine.c | exported | Unvalidated |
| `s->count_load_time = pit_get_time(s); s->last_irq_time = 0; s->count = val;` | x86_machine.c | exported | Unvalidated |
| `pit_write` | x86_machine.c | left out | not translated yet |
| `PITState *pit = opaque; int channel, access, addr; PITChannel *s; addr = offset & 3;` | x86_machine.c | exported | Unvalidated |
| `if (addr == 3) … else` | x86_machine.c | left out | still being translated |
| `pit_read` | x86_machine.c | left out | not translated yet |
| `PITState *pit = opaque; PITChannel *s; int ret, count, addr; addr = offset & 3;` | x86_machine.c | exported | Unvalidated |
| `if (addr == 3)` | x86_machine.c | exported | Unvalidated |
| `s = &pit->pit_channels[addr];` | x86_machine.c | exported | Unvalidated |
| `switch (s->rw_state)` | x86_machine.c | left out | still being translated |
| `return ret;` | x86_machine.c | exported | Unvalidated |
| `speaker_write` | x86_machine.c | left out | not translated yet |
| `PITState *pit = opaque; pit->speaker_data_on = (val >> 1) & 1; pit->pit_channels[2].gate = val…` | x86_machine.c | exported | Unvalidated |
| `speaker_read` | x86_machine.c | left out | not translated yet |
| `PITState *pit = opaque; PITChannel *s; int out, val; s = &pit->pit_channels[2]; out = pit_get_o…` | x86_machine.c | exported | Unvalidated |
| `pit_update_irq` | x86_machine.c | left out | not translated yet |
| `PITChannel *s; int64_t d, delay; s = &pit->pit_channels[0]; delay = 1193182; d = pit_get_time(s…` | x86_machine.c | exported | Unvalidated |
| `switch (s->mode)` | x86_machine.c | left out | still being translated |
| `if (delay <= 0) … else` | x86_machine.c | exported | Unvalidated |
| `serial_init` | x86_machine.c | left out | not translated yet |
| `SerialState *s; s = mallocz(sizeof(*s)); s->divider = 0; s->rbr = 0; s->ier = 0; s->iir = 0x01;…` | x86_machine.c | left out | still being translated |
| `serial_update_irq` | x86_machine.c | left out | not translated yet |
| `if (s->lsr & 0x01) && (s->ier & 0x01) … else` | x86_machine.c | exported | Unvalidated |
| `if (s->iir != 0x01) … else` | x86_machine.c | exported | Unvalidated |
| `serial_write` | x86_machine.c | left out | not translated yet |
| `SerialState *s = opaque; int addr; addr = offset & 7;` | x86_machine.c | exported | Unvalidated |
| `switch (addr)` | x86_machine.c | left out | still being translated |
| `serial_read` | x86_machine.c | left out | not translated yet |
| `SerialState *s = opaque; int ret, addr; addr = offset & 7;` | x86_machine.c | exported | Unvalidated |
| `switch (addr)` | x86_machine.c | left out | still being translated |
| `return ret;` | x86_machine.c | exported | Unvalidated |
| `serial_send_break` | x86_machine.c | left out | not translated yet |
| `s->rbr = 0; s->lsr |= 0x10 | 0x01; serial_update_irq(s);` | x86_machine.c | exported | Unvalidated |
| `port80_write` | x86_machine.c | left out | not translated yet |
| `port80_read` | x86_machine.c | left out | not translated yet |
| `return 0xff;` | x86_machine.c | exported | Unvalidated |
| `port92_write` | x86_machine.c | left out | not translated yet |
| `port92_read` | x86_machine.c | left out | not translated yet |
| `int a20 = 1; return a20 << 1;` | x86_machine.c | exported | Unvalidated |
| `vmport_read` | x86_machine.c | left out | not translated yet |
| `if (s->kvm_enabled) … else` | x86_machine.c | left out | still being translated |
| `return regs[0];` | x86_machine.c | exported | Unvalidated |
| `vmport_write` | x86_machine.c | left out | not translated yet |
| `pic_set_irq_cb` | x86_machine.c | left out | not translated yet |
| `PCMachine *s = opaque; x86_cpu_set_irq(s->cpu_state, level);` | x86_machine.c | left out | still being translated |
| `serial_write_cb` | x86_machine.c | left out | not translated yet |
| `if (s->common.console)` | x86_machine.c | left out | still being translated |
| `get_hard_intno_cb` | x86_machine.c | left out | not translated yet |
| `PCMachine *s = opaque; return pic2_get_hard_intno(s->pic_state);` | x86_machine.c | left out | still being translated |
| `pit_get_ticks_cb` | x86_machine.c | left out | not translated yet |
| `struct timespec ts; clock_gettime( 1 , &ts); return (uint64_t)ts.tv_sec * 1193182 + ((uint64_t)…` | x86_machine.c | left out | still being translated |
| `get_ram_ptr` | x86_machine.c | left out | not translated yet |
| `PhysMemoryRange *pr; pr = get_phys_mem_range(s->mem_map, paddr);` | x86_machine.c | left out | still being translated |
| `if (!pr || !pr->is_ram)` | x86_machine.c | left out | still being translated |
| `return pr->phys_mem + (uintptr_t)(paddr - pr->addr);` | x86_machine.c | left out | still being translated |
| `st_port` | x86_machine.c | left out | not translated yet |
| `PCMachine *s = opaque; PhysMemoryRange *pr; pr = get_phys_mem_range(s->port_map, port);` | x86_machine.c | left out | still being translated |
| `if (!pr)` | x86_machine.c | left out | still being translated |
| `port -= pr->addr;` | x86_machine.c | left out | still being translated |
| `if (pr->devio_flags >> size_log2) & 1) … else` | x86_machine.c | left out | still being translated |
| `ld_port` | x86_machine.c | left out | not translated yet |
| `PCMachine *s = opaque; PhysMemoryRange *pr; uint32_t val, port; port = port1; pr = get_phys_mem…` | x86_machine.c | left out | still being translated |
| `if (!pr) … else` | x86_machine.c | left out | still being translated |
| `return val;` | x86_machine.c | left out | still being translated |
| `pc_machine_set_defaults` | x86_machine.c | left out | not translated yet |
| `p->accel_enable = TRUE;` | x86_machine.c | left out | still being translated |
| `sigalrm_handler` | x86_machine.c | left out | not translated yet |
| `kvm_set_cpuid` | x86_machine.c | left out | not translated yet |
| `struct kvm_cpuid2 *kvm_cpuid; int n_ent_max, i; struct kvm_cpuid_entry2 *ent; n_ent_max = 128;…` | x86_machine.c | left out | still being translated |
| `if (ioctl(s->kvm_fd, (((2U|1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x05)) << 0) | (…` | x86_machine.c | left out | still being translated |
| `for (i = 0; i < kvm_cpuid->nent; i++)` | x86_machine.c | left out | still being translated |
| `if (ioctl(s->vcpu_fd, (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x90)) << 0) | (((…` | x86_machine.c | left out | not translated yet |
| `free(kvm_cpuid);` | x86_machine.c | left out | not translated yet |
| `kvm_map_ram` | x86_machine.c | left out | not translated yet |
| `PCMachine *s = mem_map->opaque; struct kvm_userspace_memory_region region; int flags; region.sl…` | x86_machine.c | left out | not translated yet |
| `if (pr->devram_flags & (1 << 0)` | x86_machine.c | left out | not translated yet |
| `if (pr->devram_flags & (1 << 1)` | x86_machine.c | left out | not translated yet |
| `region.flags = flags; region.guest_phys_addr = pr->addr; region.memory_size = pr->size; region.…` | x86_machine.c | left out | not translated yet |
| `if (ioctl(s->vm_fd, (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x46)) << 0) | ((((s…` | x86_machine.c | left out | not translated yet |
| `kvm_register_ram` | x86_machine.c | left out | not translated yet |
| `PhysMemoryRange *pr; uint8_t *phys_mem; pr = register_ram_entry(mem_map, addr, size, devram_fla…` | x86_machine.c | left out | not translated yet |
| `if (!phys_mem)` | x86_machine.c | left out | not translated yet |
| `pr->phys_mem = phys_mem;` | x86_machine.c | left out | not translated yet |
| `if (devram_flags & (1 << 1)` | x86_machine.c | left out | not translated yet |
| `if (pr->size != 0)` | x86_machine.c | left out | not translated yet |
| `return pr;` | x86_machine.c | left out | not translated yet |
| `kvm_set_ram_addr` | x86_machine.c | left out | not translated yet |
| `if (enabled) … else` | x86_machine.c | left out | not translated yet |
| `kvm_get_dirty_bits` | x86_machine.c | left out | not translated yet |
| `if (pr->size == 0) … else` | x86_machine.c | left out | not translated yet |
| `return pr->dirty_bits;` | x86_machine.c | left out | not translated yet |
| `kvm_free_ram` | x86_machine.c | left out | not translated yet |
| `munmap(pr->phys_mem, pr->org_size); free(pr->dirty_bits);` | x86_machine.c | left out | not translated yet |
| `kvm_pic_set_irq` | x86_machine.c | left out | not translated yet |
| `PCMachine *s = opaque; struct kvm_irq_level irq_level; irq_level.irq = irq_num; irq_level.level…` | x86_machine.c | left out | not translated yet |
| `if (ioctl(s->vm_fd, (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x61)) << 0) | ((((s…` | x86_machine.c | left out | not translated yet |
| `kvm_init` | x86_machine.c | left out | not translated yet |
| `int ret, i; struct sigaction act; struct kvm_pit_config pit_config; uint64_t base_addr; s->kvm_…` | x86_machine.c | left out | not translated yet |
| `if (s->kvm_fd < 0)` | x86_machine.c | left out | not translated yet |
| `ret = ioctl(s->kvm_fd, (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x00)) << 0) | ((…` | x86_machine.c | left out | not translated yet |
| `if (ret < 0)` | x86_machine.c | left out | not translated yet |
| `if (ret != 12)` | x86_machine.c | left out | not translated yet |
| `s->vm_fd = ioctl(s->kvm_fd, (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x01)) << 0)…` | x86_machine.c | left out | not translated yet |
| `if (s->vm_fd < 0)` | x86_machine.c | left out | not translated yet |
| `base_addr = 0xfffbc000;` | x86_machine.c | left out | not translated yet |
| `if (ioctl(s->vm_fd, (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x48)) << 0) | ((((s…` | x86_machine.c | left out | not translated yet |
| `if (ioctl(s->vm_fd, (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x47)) << 0) | ((0)…` | x86_machine.c | left out | not translated yet |
| `if (ioctl(s->vm_fd, (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x60)) << 0) | ((0)…` | x86_machine.c | left out | not translated yet |
| `memset(&pit_config, 0, sizeof(pit_config)); pit_config.flags = 1 ;` | x86_machine.c | left out | not translated yet |
| `if (ioctl(s->vm_fd, (((1U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x77)) << 0) | ((((s…` | x86_machine.c | left out | not translated yet |
| `s->vcpu_fd = ioctl(s->vm_fd, (((0U) << (((0 +8)+8)+14)) | (((0xAE)) << (0 +8)) | (((0x41)) << 0…` | x86_machine.c | left out | not translated yet |
| `if (s->vcpu_fd < 0)` | x86_machine.c | left out | not translated yet |
| `kvm_set_cpuid(s); s->kvm_run_size = ioctl(s->kvm_fd, (((0U) << (((0 +8)+8)+14)) | (((0xAE)) <<…` | x86_machine.c | left out | not translated yet |
| `if (s->kvm_run_size < 0)` | x86_machine.c | left out | not translated yet |
| `s->kvm_run = mmap( ((void *)0) , s->kvm_run_size, 0x1 | 0x2 , 0x01 , s->vcpu_fd, 0);` | x86_machine.c | left out | not translated yet |
| `if (!s->kvm_run)` | x86_machine.c | left out | not translated yet |
| `for (i = 0; i < 16; i++)` | x86_machine.c | left out | not translated yet |
| `act. __sigaction_handler.sa_handler = sigalrm_handler; sigemptyset(&act.sa_mask); act.sa_flags…` | x86_machine.c | left out | not translated yet |
| `kvm_exit_io` | x86_machine.c | left out | not translated yet |
| `uint8_t *ptr; int i; ptr = (uint8_t *)run + run->io.data_offset;` | x86_machine.c | left out | not translated yet |
| `for (i = 0; i < run->io.count; i++)` | x86_machine.c | left out | not translated yet |
| `kvm_exit_mmio` | x86_machine.c | left out | not translated yet |
| `uint8_t *data = run->mmio.data; PhysMemoryRange *pr; uint64_t addr; pr = get_phys_mem_range(s->…` | x86_machine.c | left out | not translated yet |
| `if (run->mmio.is_write) … else` | x86_machine.c | left out | not translated yet |
| `kvm_exec` | x86_machine.c | left out | not translated yet |
| `struct kvm_run *run = s->kvm_run; struct itimerval ival; int ret; ival.it_interval.tv_sec = 0;…` | x86_machine.c | left out | not translated yet |
| `if (ret < 0)` | x86_machine.c | left out | not translated yet |
| `switch (run->exit_reason)` | x86_machine.c | left out | not translated yet |
| `cpu_get_tsc` | x86_machine.c | left out | not translated yet |
| `struct timespec ts; clock_gettime( 1 , &ts); return (uint64_t)ts.tv_sec * 100000000 + (ts.tv_ns…` | x86_machine.c | left out | not translated yet |
| `pc_flush_tlb_write_range` | x86_machine.c | left out | not translated yet |
| `PCMachine *s = opaque; x86_cpu_flush_tlb_write_range_ram(s->cpu_state, ram_addr, ram_size);` | x86_machine.c | left out | not translated yet |
| `pc_machine_init` | x86_machine.c | left out | not translated yet |
| `if (strcmp(p->machine_name, "pc") != 0)` | x86_machine.c | left out | not translated yet |
| `((void) sizeof (__assert_single_arg ( p->ram_size >= (1 << 20) )), __extension__ ({ if ( p->ram…` | x86_machine.c | left out | not translated yet |
| `if (p->accel_enable)` | x86_machine.c | left out | not translated yet |
| `if (!s->kvm_enabled)` | x86_machine.c | left out | not translated yet |
| `cpu_register_ram(s->mem_map, 0xc0000, p->ram_size - 0xc0000, 0); cpu_register_ram(s->mem_map, 0…` | x86_machine.c | left out | not translated yet |
| `if (p->files[VM_FILE_BIOS].len > 0)` | x86_machine.c | left out | not translated yet |
| `if (!s->kvm_enabled)` | x86_machine.c | left out | not translated yet |
| `s->cmos_state = cmos_init(s->port_map, 0x70, &s->pic_irq[8], p->rtc_local_time);` | x86_machine.c | left out | not translated yet |
| `{ … }` | x86_machine.c | left out | not translated yet |
| `s->i440fx_state = i440fx_init(&pci_bus, &piix3_devfn, s->mem_map, s->port_map, s->pic_irq); s->…` | x86_machine.c | left out | not translated yet |
| `if (0)` | x86_machine.c | left out | not translated yet |
| `memset(vbus, 0, sizeof(*vbus)); vbus->pci_bus = pci_bus;` | x86_machine.c | left out | not translated yet |
| `if (p->console)` | x86_machine.c | left out | not translated yet |
| `for (i = 0; i < p->drive_count; )` | x86_machine.c | left out | not translated yet |
| `for (i = 0; i < p->fs_count; i++)` | x86_machine.c | left out | not translated yet |
| `if (p->display_device)` | x86_machine.c | left out | not translated yet |
| `if (p->input_device)` | x86_machine.c | left out | not translated yet |
| `for (i = 0; i < p->eth_count; i++)` | x86_machine.c | left out | not translated yet |
| `if (p->files[VM_FILE_KERNEL].buf)` | x86_machine.c | left out | not translated yet |
| `return (VirtMachine *)s;` | x86_machine.c | left out | not translated yet |
| `pc_machine_end` | x86_machine.c | left out | not translated yet |
| `if (s->cpu_state)` | x86_machine.c | left out | not translated yet |
| `phys_mem_map_end(s->mem_map); phys_mem_map_end(s->port_map); free(s);` | x86_machine.c | left out | not translated yet |
| `pc_vm_send_key_event` | x86_machine.c | left out | not translated yet |
| `if (s->keyboard_dev) … else` | x86_machine.c | left out | not translated yet |
| `pc_vm_mouse_is_absolute` | x86_machine.c | left out | not translated yet |
| `if (s->mouse_dev) … else` | x86_machine.c | left out | not translated yet |
| `pc_vm_send_mouse_event` | x86_machine.c | left out | not translated yet |
| `if (s->mouse_dev) … else` | x86_machine.c | left out | not translated yet |
| `copy_kernel` | x86_machine.c | left out | not translated yet |
| `if (buf_len < 1024)` | x86_machine.c | left out | not translated yet |
| `if (buf[0x1fe] != 0x55 || buf[0x1ff] != 0xaa)` | x86_machine.c | left out | not translated yet |
| `setup_sects = buf[0x1f1];` | x86_machine.c | left out | not translated yet |
| `if (setup_sects == 0)` | x86_machine.c | left out | not translated yet |
| `header_len = (setup_sects + 1) * 512;` | x86_machine.c | left out | not translated yet |
| `if (buf_len < header_len)` | x86_machine.c | left out | not translated yet |
| `if (memcmp(buf + 0x202, "HdrS", 4) != 0)` | x86_machine.c | left out | not translated yet |
| `load_address = 0x100000; ram_ptr = get_ram_ptr(s, load_address); copy_len = buf_len - header_le…` | x86_machine.c | left out | not translated yet |
| `if (copy_len > (s->ram_size - load_address)` | x86_machine.c | left out | not translated yet |
| `memcpy(ram_ptr, buf + header_len, copy_len); params = (void *)get_ram_ptr(s, 0x00090000); memse…` | x86_machine.c | left out | not translated yet |
| `if (fb_dev)` | x86_machine.c | left out | not translated yet |
| `params->gdt_table[2] = 0x00cf9b000000ffffLL; params->gdt_table[3] = 0x00cf93000000ffffLL;` | x86_machine.c | left out | not translated yet |
| `if (s->kvm_enabled) … else` | x86_machine.c | left out | not translated yet |
| `{ … }` | x86_machine.c | left out | not translated yet |
| `pc_machine_get_sleep_duration` | x86_machine.c | left out | not translated yet |
| `if (s->kvm_enabled) … else` | x86_machine.c | left out | not translated yet |
| `return delay;` | x86_machine.c | left out | not translated yet |
| `pc_machine_interp` | x86_machine.c | left out | not translated yet |
| `if (s->kvm_enabled) … else` | x86_machine.c | left out | not translated yet |

