//! From `sha256.c`.

// ── `sha256_compress` · sha256.c:60-176 · chunk 2151 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     static void sha256_compress(SHA256_CTX *s, unsigned char *buf)
//     {
//         uint32_t S[8], W[64], t0, t1;
//     
//     
//     
//         int i;
//     
//     
//         for (i = 0; i < 8; i++) {
//             S[i] = s->state[i];
//         }
//     
//     
//         for (i = 0; i < 16; i++) {
//             W[i] = get_be32(buf + (4*i));
//         }
//     
//     
//         for (i = 16; i < 64; i++) {
//             W[i] = (( ((((uint32_t)((W[i - 2]))&0xFFFFFFFFUL)>>(uint32_t)(((17))&31)) | ((uint32_t)((W[i - 2]))<<(uint32_t)(32-(((17))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((W[i - 2]))&0xFFFFFFFFUL)>>(uint32_t)(((19))&31)) | ((uint32_t)((W[i - 2]))<<(uint32_t)(32-(((19))&31)))) & 0xFFFFFFFFUL) ^ (((W[i - 2])&0xFFFFFFFFUL)>>(10))) + W[i - 7] + (( ((((uint32_t)((W[i - 15]))&0xFFFFFFFFUL)>>(uint32_t)(((7))&31)) | ((uint32_t)((W[i - 15]))<<(uint32_t)(32-(((7))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((W[i - 15]))&0xFFFFFFFFUL)>>(uint32_t)(((18))&31)) | ((uint32_t)((W[i - 15]))<<(uint32_t)(32-(((18))&31)))) & 0xFFFFFFFFUL) ^ (((W[i - 15])&0xFFFFFFFFUL)>>(3))) + W[i - 16];
//         }
//         t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[6] ^ (S[4] & (S[5] ^ S[6]))) + 0x428a2f98 + W[0]; t1 = (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[0] | S[1]) & S[2]) | (S[0] & S[1])); S[3] += t0; S[7] = t0 + t1;;
//         t0 = S[6] + (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[5] ^ (S[3] & (S[4] ^ S[5]))) + 0x71374491 + W[1]; t1 = (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[7] | S[0]) & S[1]) | (S[7] & S[0])); S[2] += t0; S[6] = t0 + t1;;
//         t0 = S[5] + (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[4] ^ (S[2] & (S[3] ^ S[4]))) + 0xb5c0fbcf + W[2]; t1 = (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[6] | S[7]) & S[0]) | (S[6] & S[7])); S[1] += t0; S[5] = t0 + t1;;
//         t0 = S[4] + (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[3] ^ (S[1] & (S[2] ^ S[3]))) + 0xe9b5dba5 + W[3]; t1 = (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[5] | S[6]) & S[7]) | (S[5] & S[6])); S[0] += t0; S[4] = t0 + t1;;
//         t0 = S[3] + (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[2] ^ (S[0] & (S[1] ^ S[2]))) + 0x3956c25b + W[4]; t1 = (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[4] | S[5]) & S[6]) | (S[4] & S[5])); S[7] += t0; S[3] = t0 + t1;;
//         t0 = S[2] + (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[1] ^ (S[7] & (S[0] ^ S[1]))) + 0x59f111f1 + W[5]; t1 = (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[3] | S[4]) & S[5]) | (S[3] & S[4])); S[6] += t0; S[2] = t0 + t1;;
//         t0 = S[1] + (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[0] ^ (S[6] & (S[7] ^ S[0]))) + 0x923f82a4 + W[6]; t1 = (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[2] | S[3]) & S[4]) | (S[2] & S[3])); S[5] += t0; S[1] = t0 + t1;;
//         t0 = S[0] + (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[7] ^ (S[5] & (S[6] ^ S[7]))) + 0xab1c5ed5 + W[7]; t1 = (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[1] | S[2]) & S[3]) | (S[1] & S[2])); S[4] += t0; S[0] = t0 + t1;;
//         t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[6] ^ (S[4] & (S[5] ^ S[6]))) + 0xd807aa98 + W[8]; t1 = (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[0] | S[1]) & S[2]) | (S[0] & S[1])); S[3] += t0; S[7] = t0 + t1;;
//         t0 = S[6] + (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[5] ^ (S[3] & (S[4] ^ S[5]))) + 0x12835b01 + W[9]; t1 = (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[7] | S[0]) & S[1]) | (S[7] & S[0])); S[2] += t0; S[6] = t0 + t1;;
//         t0 = S[5] + (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[4] ^ (S[2] & (S[3] ^ S[4]))) + 0x243185be + W[10]; t1 = (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[6] | S[7]) & S[0]) | (S[6] & S[7])); S[1] += t0; S[5] = t0 + t1;;
//         t0 = S[4] + (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[3] ^ (S[1] & (S[2] ^ S[3]))) + 0x550c7dc3 + W[11]; t1 = (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[5] | S[6]) & S[7]) | (S[5] & S[6])); S[0] += t0; S[4] = t0 + t1;;
//         t0 = S[3] + (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[2] ^ (S[0] & (S[1] ^ S[2]))) + 0x72be5d74 + W[12]; t1 = (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[4] | S[5]) & S[6]) | (S[4] & S[5])); S[7] += t0; S[3] = t0 + t1;;
//         t0 = S[2] + (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[1] ^ (S[7] & (S[0] ^ S[1]))) + 0x80deb1fe + W[13]; t1 = (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[3] | S[4]) & S[5]) | (S[3] & S[4])); S[6] += t0; S[2] = t0 + t1;;
//         t0 = S[1] + (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[0] ^ (S[6] & (S[7] ^ S[0]))) + 0x9bdc06a7 + W[14]; t1 = (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[2] | S[3]) & S[4]) | (S[2] & S[3])); S[5] += t0; S[1] = t0 + t1;;
//         t0 = S[0] + (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[7] ^ (S[5] & (S[6] ^ S[7]))) + 0xc19bf174 + W[15]; t1 = (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[1] | S[2]) & S[3]) | (S[1] & S[2])); S[4] += t0; S[0] = t0 + t1;;
//         t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[6] ^ (S[4] & (S[5] ^ S[6]))) + 0xe49b69c1 + W[16]; t1 = (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[0] | S[1]) & S[2]) | (S[0] & S[1])); S[3] += t0; S[7] = t0 + t1;;
//         t0 = S[6] + (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[5] ^ (S[3] & (S[4] ^ S[5]))) + 0xefbe4786 + W[17]; t1 = (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[7] | S[0]) & S[1]) | (S[7] & S[0])); S[2] += t0; S[6] = t0 + t1;;
//         t0 = S[5] + (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[4] ^ (S[2] & (S[3] ^ S[4]))) + 0x0fc19dc6 + W[18]; t1 = (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[6] | S[7]) & S[0]) | (S[6] & S[7])); S[1] += t0; S[5] = t0 + t1;;
//         t0 = S[4] + (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[3] ^ (S[1] & (S[2] ^ S[3]))) + 0x240ca1cc + W[19]; t1 = (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[5] | S[6]) & S[7]) | (S[5] & S[6])); S[0] += t0; S[4] = t0 + t1;;
//         t0 = S[3] + (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[2] ^ (S[0] & (S[1] ^ S[2]))) + 0x2de92c6f + W[20]; t1 = (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[4] | S[5]) & S[6]) | (S[4] & S[5])); S[7] += t0; S[3] = t0 + t1;;
//         t0 = S[2] + (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[1] ^ (S[7] & (S[0] ^ S[1]))) + 0x4a7484aa + W[21]; t1 = (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[3] | S[4]) & S[5]) | (S[3] & S[4])); S[6] += t0; S[2] = t0 + t1;;
//         t0 = S[1] + (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[0] ^ (S[6] & (S[7] ^ S[0]))) + 0x5cb0a9dc + W[22]; t1 = (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[2] | S[3]) & S[4]) | (S[2] & S[3])); S[5] += t0; S[1] = t0 + t1;;
//         t0 = S[0] + (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[7] ^ (S[5] & (S[6] ^ S[7]))) + 0x76f988da + W[23]; t1 = (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[1] | S[2]) & S[3]) | (S[1] & S[2])); S[4] += t0; S[0] = t0 + t1;;
//         t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[6] ^ (S[4] & (S[5] ^ S[6]))) + 0x983e5152 + W[24]; t1 = (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[0] | S[1]) & S[2]) | (S[0] & S[1])); S[3] += t0; S[7] = t0 + t1;;
//         t0 = S[6] + (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[5] ^ (S[3] & (S[4] ^ S[5]))) + 0xa831c66d + W[25]; t1 = (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[7] | S[0]) & S[1]) | (S[7] & S[0])); S[2] += t0; S[6] = t0 + t1;;
//         t0 = S[5] + (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[4] ^ (S[2] & (S[3] ^ S[4]))) + 0xb00327c8 + W[26]; t1 = (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[6] | S[7]) & S[0]) | (S[6] & S[7])); S[1] += t0; S[5] = t0 + t1;;
//         t0 = S[4] + (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[3] ^ (S[1] & (S[2] ^ S[3]))) + 0xbf597fc7 + W[27]; t1 = (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[5] | S[6]) & S[7]) | (S[5] & S[6])); S[0] += t0; S[4] = t0 + t1;;
//         t0 = S[3] + (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[2] ^ (S[0] & (S[1] ^ S[2]))) + 0xc6e00bf3 + W[28]; t1 = (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[4] | S[5]) & S[6]) | (S[4] & S[5])); S[7] += t0; S[3] = t0 + t1;;
//         t0 = S[2] + (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[1] ^ (S[7] & (S[0] ^ S[1]))) + 0xd5a79147 + W[29]; t1 = (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[3] | S[4]) & S[5]) | (S[3] & S[4])); S[6] += t0; S[2] = t0 + t1;;
//         t0 = S[1] + (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[0] ^ (S[6] & (S[7] ^ S[0]))) + 0x06ca6351 + W[30]; t1 = (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[2] | S[3]) & S[4]) | (S[2] & S[3])); S[5] += t0; S[1] = t0 + t1;;
//         t0 = S[0] + (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[7] ^ (S[5] & (S[6] ^ S[7]))) + 0x14292967 + W[31]; t1 = (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[1] | S[2]) & S[3]) | (S[1] & S[2])); S[4] += t0; S[0] = t0 + t1;;
//         t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[6] ^ (S[4] & (S[5] ^ S[6]))) + 0x27b70a85 + W[32]; t1 = (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[0] | S[1]) & S[2]) | (S[0] & S[1])); S[3] += t0; S[7] = t0 + t1;;
//         t0 = S[6] + (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[5] ^ (S[3] & (S[4] ^ S[5]))) + 0x2e1b2138 + W[33]; t1 = (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[7] | S[0]) & S[1]) | (S[7] & S[0])); S[2] += t0; S[6] = t0 + t1;;
//         t0 = S[5] + (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[4] ^ (S[2] & (S[3] ^ S[4]))) + 0x4d2c6dfc + W[34]; t1 = (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[6] | S[7]) & S[0]) | (S[6] & S[7])); S[1] += t0; S[5] = t0 + t1;;
//         t0 = S[4] + (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[3] ^ (S[1] & (S[2] ^ S[3]))) + 0x53380d13 + W[35]; t1 = (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[5] | S[6]) & S[7]) | (S[5] & S[6])); S[0] += t0; S[4] = t0 + t1;;
//         t0 = S[3] + (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[2] ^ (S[0] & (S[1] ^ S[2]))) + 0x650a7354 + W[36]; t1 = (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[4] | S[5]) & S[6]) | (S[4] & S[5])); S[7] += t0; S[3] = t0 + t1;;
//         t0 = S[2] + (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[1] ^ (S[7] & (S[0] ^ S[1]))) + 0x766a0abb + W[37]; t1 = (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[3] | S[4]) & S[5]) | (S[3] & S[4])); S[6] += t0; S[2] = t0 + t1;;
//         t0 = S[1] + (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[0] ^ (S[6] & (S[7] ^ S[0]))) + 0x81c2c92e + W[38]; t1 = (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[2] | S[3]) & S[4]) | (S[2] & S[3])); S[5] += t0; S[1] = t0 + t1;;
//         t0 = S[0] + (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[7] ^ (S[5] & (S[6] ^ S[7]))) + 0x92722c85 + W[39]; t1 = (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[1] | S[2]) & S[3]) | (S[1] & S[2])); S[4] += t0; S[0] = t0 + t1;;
//         t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[6] ^ (S[4] & (S[5] ^ S[6]))) + 0xa2bfe8a1 + W[40]; t1 = (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[0] | S[1]) & S[2]) | (S[0] & S[1])); S[3] += t0; S[7] = t0 + t1;;
//         t0 = S[6] + (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[5] ^ (S[3] & (S[4] ^ S[5]))) + 0xa81a664b + W[41]; t1 = (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[7] | S[0]) & S[1]) | (S[7] & S[0])); S[2] += t0; S[6] = t0 + t1;;
//         t0 = S[5] + (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[4] ^ (S[2] & (S[3] ^ S[4]))) + 0xc24b8b70 + W[42]; t1 = (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[6] | S[7]) & S[0]) | (S[6] & S[7])); S[1] += t0; S[5] = t0 + t1;;
//         t0 = S[4] + (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[3] ^ (S[1] & (S[2] ^ S[3]))) + 0xc76c51a3 + W[43]; t1 = (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[5] | S[6]) & S[7]) | (S[5] & S[6])); S[0] += t0; S[4] = t0 + t1;;
//         t0 = S[3] + (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[2] ^ (S[0] & (S[1] ^ S[2]))) + 0xd192e819 + W[44]; t1 = (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[4] | S[5]) & S[6]) | (S[4] & S[5])); S[7] += t0; S[3] = t0 + t1;;
//         t0 = S[2] + (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[1] ^ (S[7] & (S[0] ^ S[1]))) + 0xd6990624 + W[45]; t1 = (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[3] | S[4]) & S[5]) | (S[3] & S[4])); S[6] += t0; S[2] = t0 + t1;;
//         t0 = S[1] + (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[0] ^ (S[6] & (S[7] ^ S[0]))) + 0xf40e3585 + W[46]; t1 = (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[2] | S[3]) & S[4]) | (S[2] & S[3])); S[5] += t0; S[1] = t0 + t1;;
//         t0 = S[0] + (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[7] ^ (S[5] & (S[6] ^ S[7]))) + 0x106aa070 + W[47]; t1 = (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[1] | S[2]) & S[3]) | (S[1] & S[2])); S[4] += t0; S[0] = t0 + t1;;
//         t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[6] ^ (S[4] & (S[5] ^ S[6]))) + 0x19a4c116 + W[48]; t1 = (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[0] | S[1]) & S[2]) | (S[0] & S[1])); S[3] += t0; S[7] = t0 + t1;;
//         t0 = S[6] + (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[5] ^ (S[3] & (S[4] ^ S[5]))) + 0x1e376c08 + W[49]; t1 = (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[7] | S[0]) & S[1]) | (S[7] & S[0])); S[2] += t0; S[6] = t0 + t1;;
//         t0 = S[5] + (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[4] ^ (S[2] & (S[3] ^ S[4]))) + 0x2748774c + W[50]; t1 = (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[6] | S[7]) & S[0]) | (S[6] & S[7])); S[1] += t0; S[5] = t0 + t1;;
//         t0 = S[4] + (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[3] ^ (S[1] & (S[2] ^ S[3]))) + 0x34b0bcb5 + W[51]; t1 = (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[5] | S[6]) & S[7]) | (S[5] & S[6])); S[0] += t0; S[4] = t0 + t1;;
//         t0 = S[3] + (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[2] ^ (S[0] & (S[1] ^ S[2]))) + 0x391c0cb3 + W[52]; t1 = (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[4] | S[5]) & S[6]) | (S[4] & S[5])); S[7] += t0; S[3] = t0 + t1;;
//         t0 = S[2] + (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[1] ^ (S[7] & (S[0] ^ S[1]))) + 0x4ed8aa4a + W[53]; t1 = (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[3] | S[4]) & S[5]) | (S[3] & S[4])); S[6] += t0; S[2] = t0 + t1;;
//         t0 = S[1] + (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[0] ^ (S[6] & (S[7] ^ S[0]))) + 0x5b9cca4f + W[54]; t1 = (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[2] | S[3]) & S[4]) | (S[2] & S[3])); S[5] += t0; S[1] = t0 + t1;;
//         t0 = S[0] + (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[7] ^ (S[5] & (S[6] ^ S[7]))) + 0x682e6ff3 + W[55]; t1 = (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[1] | S[2]) & S[3]) | (S[1] & S[2])); S[4] += t0; S[0] = t0 + t1;;
//         t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[6] ^ (S[4] & (S[5] ^ S[6]))) + 0x748f82ee + W[56]; t1 = (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[0] | S[1]) & S[2]) | (S[0] & S[1])); S[3] += t0; S[7] = t0 + t1;;
//         t0 = S[6] + (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[5] ^ (S[3] & (S[4] ^ S[5]))) + 0x78a5636f + W[57]; t1 = (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[7] | S[0]) & S[1]) | (S[7] & S[0])); S[2] += t0; S[6] = t0 + t1;;
//         t0 = S[5] + (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[4] ^ (S[2] & (S[3] ^ S[4]))) + 0x84c87814 + W[58]; t1 = (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[6] | S[7]) & S[0]) | (S[6] & S[7])); S[1] += t0; S[5] = t0 + t1;;
//         t0 = S[4] + (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[3] ^ (S[1] & (S[2] ^ S[3]))) + 0x8cc70208 + W[59]; t1 = (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[5] | S[6]) & S[7]) | (S[5] & S[6])); S[0] += t0; S[4] = t0 + t1;;
//         t0 = S[3] + (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[2] ^ (S[0] & (S[1] ^ S[2]))) + 0x90befffa + W[60]; t1 = (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[4] | S[5]) & S[6]) | (S[4] & S[5])); S[7] += t0; S[3] = t0 + t1;;
//         t0 = S[2] + (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[1] ^ (S[7] & (S[0] ^ S[1]))) + 0xa4506ceb + W[61]; t1 = (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[3] | S[4]) & S[5]) | (S[3] & S[4])); S[6] += t0; S[2] = t0 + t1;;
//         t0 = S[1] + (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[0] ^ (S[6] & (S[7] ^ S[0]))) + 0xbef9a3f7 + W[62]; t1 = (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[2] | S[3]) & S[4]) | (S[2] & S[3])); S[5] += t0; S[1] = t0 + t1;;
//         t0 = S[0] + (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[7] ^ (S[5] & (S[6] ^ S[7]))) + 0xc67178f2 + W[63]; t1 = (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[1] | S[2]) & S[3]) | (S[1] & S[2])); S[4] += t0; S[0] = t0 + t1;;
//     
//     
//     
//     
//     
//     
//         for (i = 0; i < 8; i++) {
//             s->state[i] = s->state[i] + S[i];
//         }
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2151 {
// Rust port of sha256.c: static void sha256_compress(SHA256_CTX *s, unsigned char *buf)

use core::ffi::c_uchar;

#[repr(C)]
pub struct SHA256_CTX {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

// Port of the static inline get_be32(const uint8_t *d)
#[inline]
unsafe fn get_be32(d: *const u8) -> u32 {
    ((*d as u32) << 24)
        | ((*d.add(1) as u32) << 16)
        | ((*d.add(2) as u32) << 8)
        | (*d.add(3) as u32)
}

#[inline]
fn ch(x: u32, y: u32, z: u32) -> u32 {
    z ^ (x & (y ^ z))
}

#[inline]
fn maj(x: u32, y: u32, z: u32) -> u32 {
    ((x | y) & z) | (x & y)
}

#[inline]
fn sigma0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

#[inline]
fn sigma1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

#[inline]
fn gamma0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

#[inline]
fn gamma1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

#[no_mangle]
pub extern "C" fn sha256_compress(s: *mut SHA256_CTX, buf: *mut c_uchar) {
    let mut st = [0u32; 8];
    let mut w = [0u32; 64];
    let mut t0: u32;
    let mut t1: u32;

    unsafe {
        let state = core::ptr::addr_of_mut!((*s).state) as *mut u32;
        let buf = buf as *const u8;

        // copy state into st
        let mut i: usize = 0;
        while i < 8 {
            st[i] = *state.add(i);
            i += 1;
        }

        // copy the 512-bit block into W[0..15] (big endian)
        i = 0;
        while i < 16 {
            w[i] = get_be32(buf.add(4 * i));
            i += 1;
        }

        // fill W[16..63]
        i = 16;
        while i < 64 {
            let a = w[i - 2];
            let b = w[i - 15];
            w[i] = gamma1(a)
                .wrapping_add(w[i - 7])
                .wrapping_add(gamma0(b))
                .wrapping_add(w[i - 16]);
            i += 1;
        }

        // compress
        i = 0;
        while i < 64 {
            t0 = st[7]
                .wrapping_add(sigma1(st[4]))
                .wrapping_add(ch(st[4], st[5], st[6]))
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            t1 = sigma0(st[0]).wrapping_add(maj(st[0], st[1], st[2]));
            st[3] = st[3].wrapping_add(t0);
            st[7] = t0.wrapping_add(t1);

            // rotate the working state by one word
            let t = st[7];
            st[7] = st[6];
            st[6] = st[5];
            st[5] = st[4];
            st[4] = st[3];
            st[3] = st[2];
            st[2] = st[1];
            st[1] = st[0];
            st[0] = t;

            i += 1;
        }

        // feedback
        i = 0;
        while i < 8 {
            let cur = *state.add(i);
            *state.add(i) = cur.wrapping_add(st[i]);
            i += 1;
        }
    }
}
}

// ── `for (i = 0; i < 8; i++)` · sha256.c:69-71 · chunk 2152 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (i = 0; i < 8; i++) {
//             S[i] = s->state[i];
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2152 {
// Port of the "copy state into S" loop from sha256_compress.
use core::ffi::c_int;

#[repr(C)]
pub struct SHA256_CTX {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

#[no_mangle]
pub unsafe extern "C" fn sha256_compress__part2152(
    s: *mut SHA256_CTX,
    big_s: *mut u32,
    i: *mut c_int,
) {
    let mut idx: c_int = 0;
    while idx < 8 {
        let v: u32 = unsafe { *(*s).state.as_ptr().add(idx as usize) };
        unsafe {
            *big_s.offset(idx as isize) = v;
        }
        idx += 1;
    }
    // C loop leaves i == 8 after finishing.
    if !i.is_null() {
        unsafe {
            *i = idx;
        }
    }
}
}

// ── `for (i = 0; i < 16; i++)` · sha256.c:74-76 · chunk 2153 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (i = 0; i < 16; i++) {
//             W[i] = get_be32(buf + (4*i));
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2153 {
// Port of the message-schedule initialisation loop of sha256_compress:
//   for (i = 0; i < 16; i++) W[i] = get_be32(buf + 4*i);
// get_be32 is a `static inline` C helper, so its logic lives here privately.

#[no_mangle]
pub extern "C" fn sha256_compress__part2153(buf: *const u8, w: *mut u32) {
    let mut i: i32 = 0;
    while i < 16 {
        unsafe {
            let d = buf.add((4 * i) as usize);
            let v = ((*d as u32) << 24)
                | ((*d.add(1) as u32) << 16)
                | ((*d.add(2) as u32) << 8)
                | (*d.add(3) as u32);
            *w.add(i as usize) = v;
        }
        i += 1;
    }
}
}

// ── `for (i = 16; i < 64; i++)` · sha256.c:79-81 · chunk 2154 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (i = 16; i < 64; i++) {
//             W[i] = (( ((((uint32_t)((W[i - 2]))&0xFFFFFFFFUL)>>(uint32_t)(((17))&31)) | ((uint32_t)((W[i - 2]))<<(uint32_t)(32-(((17))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((W[i - 2]))&0xFFFFFFFFUL)>>(uint32_t)(((19))&31)) | ((uint32_t)((W[i - 2]))<<(uint32_t)(32-(((19))&31)))) & 0xFFFFFFFFUL) ^ (((W[i - 2])&0xFFFFFFFFUL)>>(10))) + W[i - 7] + (( ((((uint32_t)((W[i - 15]))&0xFFFFFFFFUL)>>(uint32_t)(((7))&31)) | ((uint32_t)((W[i - 15]))<<(uint32_t)(32-(((7))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((W[i - 15]))&0xFFFFFFFFUL)>>(uint32_t)(((18))&31)) | ((uint32_t)((W[i - 15]))<<(uint32_t)(32-(((18))&31)))) & 0xFFFFFFFFUL) ^ (((W[i - 15])&0xFFFFFFFFUL)>>(3))) + W[i - 16];
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2154 {
// Expansion of the SHA-256 message schedule: W[16..64].
// Mirrors the C loop bit-for-bit (32-bit unsigned wrapping arithmetic).
#[no_mangle]
pub extern "C" fn sha256_compress__part2154(w: *mut u32, i: *mut i32) {
    let mut k: i32 = unsafe { *i };
    while k < 64 {
        unsafe {
            let w2: u32 = *w.offset(k as isize - 2);
            let w15: u32 = *w.offset(k as isize - 15);
            // sigma0(x) = ROTR17(x) ^ ROTR19(x) ^ SHR10(x)
            let s0: u32 = w2.rotate_right(17) ^ w2.rotate_right(19) ^ (w2 >> 10);
            // sigma1(x) = ROTR7(x) ^ ROTR18(x) ^ SHR3(x)
            let s1: u32 = w15.rotate_right(7) ^ w15.rotate_right(18) ^ (w15 >> 3);
            let val: u32 = s0
                .wrapping_add(*w.offset(k as isize - 7))
                .wrapping_add(s1)
                .wrapping_add(*w.offset(k as isize - 16));
            *w.offset(k as isize) = val;
        }
        k += 1;
    }
    unsafe {
        *i = k;
    }
}
}

// ── `t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))…` · sha256.c:103-166 · chunk 2155 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[6] ^ (S[4] & (S[5] ^ S[6]))) + 0x428a2f98 + W[0]; t1 = (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[0] | S[1]) & S[2]) | (S[0] & S[1])); S[3] += t0; S[7] = t0 + t1;;
//         t0 = S[6] + (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[5] ^ (S[3] & (S[4] ^ S[5]))) + 0x71374491 + W[1]; t1 = (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[7] | S[0]) & S[1]) | (S[7] & S[0])); S[2] += t0; S[6] = t0 + t1;;
//         t0 = S[5] + (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[4] ^ (S[2] & (S[3] ^ S[4]))) + 0xb5c0fbcf + W[2]; t1 = (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[6] | S[7]) & S[0]) | (S[6] & S[7])); S[1] += t0; S[5] = t0 + t1;;
//         t0 = S[4] + (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[3] ^ (S[1] & (S[2] ^ S[3]))) + 0xe9b5dba5 + W[3]; t1 = (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[5] | S[6]) & S[7]) | (S[5] & S[6])); S[0] += t0; S[4] = t0 + t1;;
//         t0 = S[3] + (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[2] ^ (S[0] & (S[1] ^ S[2]))) + 0x3956c25b + W[4]; t1 = (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[4] | S[5]) & S[6]) | (S[4] & S[5])); S[7] += t0; S[3] = t0 + t1;;
//         t0 = S[2] + (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[1] ^ (S[7] & (S[0] ^ S[1]))) + 0x59f111f1 + W[5]; t1 = (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[3] | S[4]) & S[5]) | (S[3] & S[4])); S[6] += t0; S[2] = t0 + t1;;
//         t0 = S[1] + (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[0] ^ (S[6] & (S[7] ^ S[0]))) + 0x923f82a4 + W[6]; t1 = (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[2] | S[3]) & S[4]) | (S[2] & S[3])); S[5] += t0; S[1] = t0 + t1;;
//         t0 = S[0] + (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[7] ^ (S[5] & (S[6] ^ S[7]))) + 0xab1c5ed5 + W[7]; t1 = (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[1] | S[2]) & S[3]) | (S[1] & S[2])); S[4] += t0; S[0] = t0 + t1;;
//         t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[6] ^ (S[4] & (S[5] ^ S[6]))) + 0xd807aa98 + W[8]; t1 = (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[0] | S[1]) & S[2]) | (S[0] & S[1])); S[3] += t0; S[7] = t0 + t1;;
//         t0 = S[6] + (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[5] ^ (S[3] & (S[4] ^ S[5]))) + 0x12835b01 + W[9]; t1 = (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[7] | S[0]) & S[1]) | (S[7] & S[0])); S[2] += t0; S[6] = t0 + t1;;
//         t0 = S[5] + (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[4] ^ (S[2] & (S[3] ^ S[4]))) + 0x243185be + W[10]; t1 = (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[6] | S[7]) & S[0]) | (S[6] & S[7])); S[1] += t0; S[5] = t0 + t1;;
//         t0 = S[4] + (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[3] ^ (S[1] & (S[2] ^ S[3]))) + 0x550c7dc3 + W[11]; t1 = (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[5] | S[6]) & S[7]) | (S[5] & S[6])); S[0] += t0; S[4] = t0 + t1;;
//         t0 = S[3] + (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[2] ^ (S[0] & (S[1] ^ S[2]))) + 0x72be5d74 + W[12]; t1 = (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[4] | S[5]) & S[6]) | (S[4] & S[5])); S[7] += t0; S[3] = t0 + t1;;
//         t0 = S[2] + (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[1] ^ (S[7] & (S[0] ^ S[1]))) + 0x80deb1fe + W[13]; t1 = (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[3] | S[4]) & S[5]) | (S[3] & S[4])); S[6] += t0; S[2] = t0 + t1;;
//         t0 = S[1] + (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[0] ^ (S[6] & (S[7] ^ S[0]))) + 0x9bdc06a7 + W[14]; t1 = (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[2] | S[3]) & S[4]) | (S[2] & S[3])); S[5] += t0; S[1] = t0 + t1;;
//         t0 = S[0] + (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[7] ^ (S[5] & (S[6] ^ S[7]))) + 0xc19bf174 + W[15]; t1 = (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[1] | S[2]) & S[3]) | (S[1] & S[2])); S[4] += t0; S[0] = t0 + t1;;
//         t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[6] ^ (S[4] & (S[5] ^ S[6]))) + 0xe49b69c1 + W[16]; t1 = (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[0] | S[1]) & S[2]) | (S[0] & S[1])); S[3] += t0; S[7] = t0 + t1;;
//         t0 = S[6] + (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[5] ^ (S[3] & (S[4] ^ S[5]))) + 0xefbe4786 + W[17]; t1 = (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[7] | S[0]) & S[1]) | (S[7] & S[0])); S[2] += t0; S[6] = t0 + t1;;
//         t0 = S[5] + (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[4] ^ (S[2] & (S[3] ^ S[4]))) + 0x0fc19dc6 + W[18]; t1 = (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[6] | S[7]) & S[0]) | (S[6] & S[7])); S[1] += t0; S[5] = t0 + t1;;
//         t0 = S[4] + (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[3] ^ (S[1] & (S[2] ^ S[3]))) + 0x240ca1cc + W[19]; t1 = (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[5] | S[6]) & S[7]) | (S[5] & S[6])); S[0] += t0; S[4] = t0 + t1;;
//         t0 = S[3] + (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[2] ^ (S[0] & (S[1] ^ S[2]))) + 0x2de92c6f + W[20]; t1 = (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[4] | S[5]) & S[6]) | (S[4] & S[5])); S[7] += t0; S[3] = t0 + t1;;
//         t0 = S[2] + (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[1] ^ (S[7] & (S[0] ^ S[1]))) + 0x4a7484aa + W[21]; t1 = (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[3] | S[4]) & S[5]) | (S[3] & S[4])); S[6] += t0; S[2] = t0 + t1;;
//         t0 = S[1] + (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[0] ^ (S[6] & (S[7] ^ S[0]))) + 0x5cb0a9dc + W[22]; t1 = (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[2] | S[3]) & S[4]) | (S[2] & S[3])); S[5] += t0; S[1] = t0 + t1;;
//         t0 = S[0] + (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[7] ^ (S[5] & (S[6] ^ S[7]))) + 0x76f988da + W[23]; t1 = (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[1] | S[2]) & S[3]) | (S[1] & S[2])); S[4] += t0; S[0] = t0 + t1;;
//         t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[6] ^ (S[4] & (S[5] ^ S[6]))) + 0x983e5152 + W[24]; t1 = (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[0] | S[1]) & S[2]) | (S[0] & S[1])); S[3] += t0; S[7] = t0 + t1;;
//         t0 = S[6] + (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[5] ^ (S[3] & (S[4] ^ S[5]))) + 0xa831c66d + W[25]; t1 = (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[7] | S[0]) & S[1]) | (S[7] & S[0])); S[2] += t0; S[6] = t0 + t1;;
//         t0 = S[5] + (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[4] ^ (S[2] & (S[3] ^ S[4]))) + 0xb00327c8 + W[26]; t1 = (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[6] | S[7]) & S[0]) | (S[6] & S[7])); S[1] += t0; S[5] = t0 + t1;;
//         t0 = S[4] + (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[3] ^ (S[1] & (S[2] ^ S[3]))) + 0xbf597fc7 + W[27]; t1 = (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[5] | S[6]) & S[7]) | (S[5] & S[6])); S[0] += t0; S[4] = t0 + t1;;
//         t0 = S[3] + (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[2] ^ (S[0] & (S[1] ^ S[2]))) + 0xc6e00bf3 + W[28]; t1 = (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[4] | S[5]) & S[6]) | (S[4] & S[5])); S[7] += t0; S[3] = t0 + t1;;
//         t0 = S[2] + (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[1] ^ (S[7] & (S[0] ^ S[1]))) + 0xd5a79147 + W[29]; t1 = (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[3] | S[4]) & S[5]) | (S[3] & S[4])); S[6] += t0; S[2] = t0 + t1;;
//         t0 = S[1] + (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[0] ^ (S[6] & (S[7] ^ S[0]))) + 0x06ca6351 + W[30]; t1 = (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[2] | S[3]) & S[4]) | (S[2] & S[3])); S[5] += t0; S[1] = t0 + t1;;
//         t0 = S[0] + (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[7] ^ (S[5] & (S[6] ^ S[7]))) + 0x14292967 + W[31]; t1 = (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[1] | S[2]) & S[3]) | (S[1] & S[2])); S[4] += t0; S[0] = t0 + t1;;
//         t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[6] ^ (S[4] & (S[5] ^ S[6]))) + 0x27b70a85 + W[32]; t1 = (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[0] | S[1]) & S[2]) | (S[0] & S[1])); S[3] += t0; S[7] = t0 + t1;;
//         t0 = S[6] + (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[5] ^ (S[3] & (S[4] ^ S[5]))) + 0x2e1b2138 + W[33]; t1 = (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[7] | S[0]) & S[1]) | (S[7] & S[0])); S[2] += t0; S[6] = t0 + t1;;
//         t0 = S[5] + (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[4] ^ (S[2] & (S[3] ^ S[4]))) + 0x4d2c6dfc + W[34]; t1 = (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[6] | S[7]) & S[0]) | (S[6] & S[7])); S[1] += t0; S[5] = t0 + t1;;
//         t0 = S[4] + (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[3] ^ (S[1] & (S[2] ^ S[3]))) + 0x53380d13 + W[35]; t1 = (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[5] | S[6]) & S[7]) | (S[5] & S[6])); S[0] += t0; S[4] = t0 + t1;;
//         t0 = S[3] + (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[2] ^ (S[0] & (S[1] ^ S[2]))) + 0x650a7354 + W[36]; t1 = (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[4] | S[5]) & S[6]) | (S[4] & S[5])); S[7] += t0; S[3] = t0 + t1;;
//         t0 = S[2] + (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[1] ^ (S[7] & (S[0] ^ S[1]))) + 0x766a0abb + W[37]; t1 = (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[3] | S[4]) & S[5]) | (S[3] & S[4])); S[6] += t0; S[2] = t0 + t1;;
//         t0 = S[1] + (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[0] ^ (S[6] & (S[7] ^ S[0]))) + 0x81c2c92e + W[38]; t1 = (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[2] | S[3]) & S[4]) | (S[2] & S[3])); S[5] += t0; S[1] = t0 + t1;;
//         t0 = S[0] + (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[7] ^ (S[5] & (S[6] ^ S[7]))) + 0x92722c85 + W[39]; t1 = (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[1] | S[2]) & S[3]) | (S[1] & S[2])); S[4] += t0; S[0] = t0 + t1;;
//         t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[6] ^ (S[4] & (S[5] ^ S[6]))) + 0xa2bfe8a1 + W[40]; t1 = (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[0] | S[1]) & S[2]) | (S[0] & S[1])); S[3] += t0; S[7] = t0 + t1;;
//         t0 = S[6] + (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[5] ^ (S[3] & (S[4] ^ S[5]))) + 0xa81a664b + W[41]; t1 = (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[7] | S[0]) & S[1]) | (S[7] & S[0])); S[2] += t0; S[6] = t0 + t1;;
//         t0 = S[5] + (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[4] ^ (S[2] & (S[3] ^ S[4]))) + 0xc24b8b70 + W[42]; t1 = (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[6] | S[7]) & S[0]) | (S[6] & S[7])); S[1] += t0; S[5] = t0 + t1;;
//         t0 = S[4] + (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[3] ^ (S[1] & (S[2] ^ S[3]))) + 0xc76c51a3 + W[43]; t1 = (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[5] | S[6]) & S[7]) | (S[5] & S[6])); S[0] += t0; S[4] = t0 + t1;;
//         t0 = S[3] + (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[2] ^ (S[0] & (S[1] ^ S[2]))) + 0xd192e819 + W[44]; t1 = (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[4] | S[5]) & S[6]) | (S[4] & S[5])); S[7] += t0; S[3] = t0 + t1;;
//         t0 = S[2] + (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[1] ^ (S[7] & (S[0] ^ S[1]))) + 0xd6990624 + W[45]; t1 = (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[3] | S[4]) & S[5]) | (S[3] & S[4])); S[6] += t0; S[2] = t0 + t1;;
//         t0 = S[1] + (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[0] ^ (S[6] & (S[7] ^ S[0]))) + 0xf40e3585 + W[46]; t1 = (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[2] | S[3]) & S[4]) | (S[2] & S[3])); S[5] += t0; S[1] = t0 + t1;;
//         t0 = S[0] + (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[7] ^ (S[5] & (S[6] ^ S[7]))) + 0x106aa070 + W[47]; t1 = (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[1] | S[2]) & S[3]) | (S[1] & S[2])); S[4] += t0; S[0] = t0 + t1;;
//         t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[6] ^ (S[4] & (S[5] ^ S[6]))) + 0x19a4c116 + W[48]; t1 = (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[0] | S[1]) & S[2]) | (S[0] & S[1])); S[3] += t0; S[7] = t0 + t1;;
//         t0 = S[6] + (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[5] ^ (S[3] & (S[4] ^ S[5]))) + 0x1e376c08 + W[49]; t1 = (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[7] | S[0]) & S[1]) | (S[7] & S[0])); S[2] += t0; S[6] = t0 + t1;;
//         t0 = S[5] + (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[4] ^ (S[2] & (S[3] ^ S[4]))) + 0x2748774c + W[50]; t1 = (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[6] | S[7]) & S[0]) | (S[6] & S[7])); S[1] += t0; S[5] = t0 + t1;;
//         t0 = S[4] + (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[3] ^ (S[1] & (S[2] ^ S[3]))) + 0x34b0bcb5 + W[51]; t1 = (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[5] | S[6]) & S[7]) | (S[5] & S[6])); S[0] += t0; S[4] = t0 + t1;;
//         t0 = S[3] + (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[2] ^ (S[0] & (S[1] ^ S[2]))) + 0x391c0cb3 + W[52]; t1 = (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[4] | S[5]) & S[6]) | (S[4] & S[5])); S[7] += t0; S[3] = t0 + t1;;
//         t0 = S[2] + (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[1] ^ (S[7] & (S[0] ^ S[1]))) + 0x4ed8aa4a + W[53]; t1 = (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[3] | S[4]) & S[5]) | (S[3] & S[4])); S[6] += t0; S[2] = t0 + t1;;
//         t0 = S[1] + (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[0] ^ (S[6] & (S[7] ^ S[0]))) + 0x5b9cca4f + W[54]; t1 = (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[2] | S[3]) & S[4]) | (S[2] & S[3])); S[5] += t0; S[1] = t0 + t1;;
//         t0 = S[0] + (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[7] ^ (S[5] & (S[6] ^ S[7]))) + 0x682e6ff3 + W[55]; t1 = (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[1] | S[2]) & S[3]) | (S[1] & S[2])); S[4] += t0; S[0] = t0 + t1;;
//         t0 = S[7] + (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[6] ^ (S[4] & (S[5] ^ S[6]))) + 0x748f82ee + W[56]; t1 = (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[0] | S[1]) & S[2]) | (S[0] & S[1])); S[3] += t0; S[7] = t0 + t1;;
//         t0 = S[6] + (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[5] ^ (S[3] & (S[4] ^ S[5]))) + 0x78a5636f + W[57]; t1 = (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[7] | S[0]) & S[1]) | (S[7] & S[0])); S[2] += t0; S[6] = t0 + t1;;
//         t0 = S[5] + (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[4] ^ (S[2] & (S[3] ^ S[4]))) + 0x84c87814 + W[58]; t1 = (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[6] | S[7]) & S[0]) | (S[6] & S[7])); S[1] += t0; S[5] = t0 + t1;;
//         t0 = S[4] + (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[3] ^ (S[1] & (S[2] ^ S[3]))) + 0x8cc70208 + W[59]; t1 = (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[5] | S[6]) & S[7]) | (S[5] & S[6])); S[0] += t0; S[4] = t0 + t1;;
//         t0 = S[3] + (( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[0]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[0]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[2] ^ (S[0] & (S[1] ^ S[2]))) + 0x90befffa + W[60]; t1 = (( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[4]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[4]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[4] | S[5]) & S[6]) | (S[4] & S[5])); S[7] += t0; S[3] = t0 + t1;;
//         t0 = S[2] + (( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[7]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[7]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[1] ^ (S[7] & (S[0] ^ S[1]))) + 0xa4506ceb + W[61]; t1 = (( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[3]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[3]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[3] | S[4]) & S[5]) | (S[3] & S[4])); S[6] += t0; S[2] = t0 + t1;;
//         t0 = S[1] + (( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[6]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[6]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[0] ^ (S[6] & (S[7] ^ S[0]))) + 0xbef9a3f7 + W[62]; t1 = (( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[2]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[2]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[2] | S[3]) & S[4]) | (S[2] & S[3])); S[5] += t0; S[1] = t0 + t1;;
//         t0 = S[0] + (( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((6))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((6))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((11))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((11))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[5]))&0xFFFFFFFFUL)>>(uint32_t)(((25))&31)) | ((uint32_t)((S[5]))<<(uint32_t)(32-(((25))&31)))) & 0xFFFFFFFFUL)) + (S[7] ^ (S[5] & (S[6] ^ S[7]))) + 0xc67178f2 + W[63]; t1 = (( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((2))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((2))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((13))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((13))&31)))) & 0xFFFFFFFFUL) ^ ( ((((uint32_t)((S[1]))&0xFFFFFFFFUL)>>(uint32_t)(((22))&31)) | ((uint32_t)((S[1]))<<(uint32_t)(32-(((22))&31)))) & 0xFFFFFFFFUL)) + (((S[1] | S[2]) & S[3]) | (S[1] & S[2])); S[4] += t0; S[0] = t0 + t1;;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2155 {
// SHA-256 compression: rounds 0..64 of sha256_compress.
// Reads the working state S[8] and the expanded message schedule W[64]
// (both uint32_t arrays were produced by the C caller) and updates S in place.

#[inline(always)]
fn bsig0(x: u32) -> u32 {
    // rotate_right(2) ^ rotate_right(13) ^ rotate_right(22)
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

#[inline(always)]
fn bsig1(x: u32) -> u32 {
    // rotate_right(6) ^ rotate_right(11) ^ rotate_right(25)
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

static K: [u32; 64] = [
    0x428a2f98u32, 0x71374491u32, 0xb5c0fbcfu32, 0xe9b5dba5u32,
    0x3956c25bu32, 0x59f111f1u32, 0x923f82a4u32, 0xab1c5ed5u32,
    0xd807aa98u32, 0x12835b01u32, 0x243185beu32, 0x550c7dc3u32,
    0x72be5d74u32, 0x80deb1feu32, 0x9bdc06a7u32, 0xc19bf174u32,
    0xe49b69c1u32, 0xefbe4786u32, 0x0fc19dc6u32, 0x240ca1ccu32,
    0x2de92c6fu32, 0x4a7484aau32, 0x5cb0a9dcu32, 0x76f988dau32,
    0x983e5152u32, 0xa831c66du32, 0xb00327c8u32, 0xbf597fc7u32,
    0xc6e00bf3u32, 0xd5a79147u32, 0x06ca6351u32, 0x14292967u32,
    0x27b70a85u32, 0x2e1b2138u32, 0x4d2c6dfcu32, 0x53380d13u32,
    0x650a7354u32, 0x766a0abbu32, 0x81c2c92eu32, 0x92722c85u32,
    0xa2bfe8a1u32, 0xa81a664bu32, 0xc24b8b70u32, 0xc76c51a3u32,
    0xd192e819u32, 0xd6990624u32, 0xf40e3585u32, 0x106aa070u32,
    0x19a4c116u32, 0x1e376c08u32, 0x2748774cu32, 0x34b0bcb5u32,
    0x391c0cb3u32, 0x4ed8aa4au32, 0x5b9cca4fu32, 0x682e6ff3u32,
    0x748f82eeu32, 0x78a5636fu32, 0x84c87814u32, 0x8cc70208u32,
    0x90befffau32, 0xa4506cebu32, 0xbef9a3f7u32, 0xc67178f2u32,
];

#[no_mangle]
pub extern "C" fn sha256_compress__part2155(s: *mut u32, w: *const u32) {
    if s.is_null() || w.is_null() {
        return;
    }

    let mut st = [0u32; 8];
    let mut i = 0usize;
    while i < 8 {
        st[i] = unsafe { *s.add(i) };
        i += 1;
    }

    let mut a = st[0];
    let mut b = st[1];
    let mut c = st[2];
    let mut d = st[3];
    let mut e = st[4];
    let mut f = st[5];
    let mut g = st[6];
    let mut h = st[7];

    let mut r = 0usize;
    while r < 64 {
        let wi = unsafe { *w.add(r) };
        // t0 = S[k7] + Sigma1(S[k4]) + (S[k6] ^ (S[k4] & (S[k5] ^ S[k6]))) + K + W
        let t0 = h
            .wrapping_add(bsig1(e))
            .wrapping_add(g ^ (e & (f ^ g)))
            .wrapping_add(K[r])
            .wrapping_add(wi);
        // t1 = Sigma0(a) + (((a | b) & c) | (a & b))
        let t1 = bsig0(a).wrapping_add(((a | b) & c) | (a & b));
        // register rotation as performed in the unrolled C code
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t0);
        d = c;
        c = b;
        b = a;
        a = t0.wrapping_add(t1);
        r += 1;
    }

    unsafe {
        *s.add(0) = a;
        *s.add(1) = b;
        *s.add(2) = c;
        *s.add(3) = d;
        *s.add(4) = e;
        *s.add(5) = f;
        *s.add(6) = g;
        *s.add(7) = h;
    }
}
}

// ── `for (i = 0; i < 8; i++)` · sha256.c:173-175 · chunk 2156 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (i = 0; i < 8; i++) {
//             s->state[i] = s->state[i] + S[i];
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2156 {
// Port of the SHA-256 state feed-forward loop of sha256_compress.

#[repr(C)]
pub struct SHA256_CTX {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

#[no_mangle]
pub unsafe extern "C" fn sha256_compress__part2156(s: *mut SHA256_CTX, w: *const u32) {
    let mut i: core::ffi::c_int = 0;
    while i < 8 {
        let idx = i as usize;
        let state = core::ptr::addr_of_mut!((*s).state) as *mut u32;
        let old = *state.add(idx);
        let add = *w.add(idx);
        *state.add(idx) = old.wrapping_add(add);
        i += 1;
    }
}
}

// ── `SHA256_Init` · sha256.c:193-205 · chunk 2157 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void SHA256_Init(SHA256_CTX *s)
//     {
//         s->curlen = 0;
//         s->length = 0;
//         s->state[0] = 0x6A09E667UL;
//         s->state[1] = 0xBB67AE85UL;
//         s->state[2] = 0x3C6EF372UL;
//         s->state[3] = 0xA54FF53AUL;
//         s->state[4] = 0x510E527FUL;
//         s->state[5] = 0x9B05688CUL;
//         s->state[6] = 0x1F83D9ABUL;
//         s->state[7] = 0x5BE0CD19UL;
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2157 {
// Rewrite of libtomcrypt-style SHA256_Init.
#![allow(non_camel_case_types)]

use core::ffi::{c_int, c_ulong};
use core::primitive::{u32, u64, u8};

#[repr(C)]
pub struct SHA256_CTX {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

// Keep the C types visible/referenced so nothing is elided incorrectly.
const _: usize = core::mem::size_of::<c_ulong>() - core::mem::size_of::<c_ulong>();

#[no_mangle]
pub unsafe extern "C" fn SHA256_Init(s: *mut SHA256_CTX) {
    if s.is_null() {
        return;
    }
    let ctx = &mut *s;
    ctx.curlen = 0;
    ctx.length = 0;
    ctx.state[0] = 0x6A09E667u32;
    ctx.state[1] = 0xBB67AE85u32;
    ctx.state[2] = 0x3C6EF372u32;
    ctx.state[3] = 0xA54FF53Au32;
    ctx.state[4] = 0x510E527Fu32;
    ctx.state[5] = 0x9B05688Cu32;
    ctx.state[6] = 0x1F83D9ABu32;
    ctx.state[7] = 0x5BE0CD19u32;
}
}

// ── `s->curlen = 0; s->length = 0; s->state[0] = 0x6A09E667UL; s->state[1] = 0xBB67AE85UL; s->state[…` · sha256.c:195-204 · chunk 2158 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->curlen = 0;
//         s->length = 0;
//         s->state[0] = 0x6A09E667UL;
//         s->state[1] = 0xBB67AE85UL;
//         s->state[2] = 0x3C6EF372UL;
//         s->state[3] = 0xA54FF53AUL;
//         s->state[4] = 0x510E527FUL;
//         s->state[5] = 0x9B05688CUL;
//         s->state[6] = 0x1F83D9ABUL;
//         s->state[7] = 0x5BE0CD19UL;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2158 {
// Ported piece of SHA256_Init: initializes chaining state and counters.
// Mirror of the C struct SHA256_CTX (sha256.c):
//   typedef struct {
//       uint64_t length;
//       uint32_t state[8], curlen;
//       uint8_t buf[64];
//   } SHA256_CTX;
#[repr(C)]
pub struct SHA256_CTX {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

#[no_mangle]
pub unsafe extern "C" fn SHA256_Init__part2158(s: *mut SHA256_CTX) {
    if s.is_null() {
        return;
    }
    let ctx = &mut *s;
    ctx.curlen = 0;
    ctx.length = 0;
    ctx.state[0] = 0x6A09E667u32;
    ctx.state[1] = 0xBB67AE85u32;
    ctx.state[2] = 0x3C6EF372u32;
    ctx.state[3] = 0xA54FF53Au32;
    ctx.state[4] = 0x510E527Fu32;
    ctx.state[5] = 0x9B05688Cu32;
    ctx.state[6] = 0x1F83D9ABu32;
    ctx.state[7] = 0x5BE0CD19u32;
}
}

// ── `SHA256_Update` · sha256.c:207-235 · chunk 2159 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void SHA256_Update(SHA256_CTX *s, const uint8_t *in, unsigned long inlen)
//     {
//         unsigned long n;
//     
//         if (s->curlen > sizeof(s->buf)) {
//             abort();
//         }
//         if ((s->length + inlen) < s->length) {
//             abort();
//         }
//         while (inlen > 0) {
//             if (s->curlen == 0 && inlen >= 64) {
//                 sha256_compress(s, (unsigned char *)in);
//                 s->length += 64 * 8;
//                 in += 64;
//                 inlen -= 64;
//             } else {
//                 n = min_int(inlen, 64 - s->curlen);
//                 memcpy(s->buf + s->curlen, in, (size_t)n);
//                 s->curlen += n;
//                 in += n;
//                 inlen -= n;
//                 if (s->curlen == 64) {
//                     sha256_compress(s, s->buf);
//                     s->length += 8*64;
//                     s->curlen = 0;
//                 }
//            }
//         } }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2159 {
// Rust port of SHA256_Update from sha256.c (LibTomCrypt).

use core::ffi::{c_int, c_ulong};

#[repr(C)]
pub struct SHA256_CTX {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

extern "C" {
    // static C function sha256_compress(SHA256_CTX *s, unsigned char *buf)
    // is not externally visible, so bind it by its mangled-free local name only
    // if it exists. It does not, therefore it is re-implemented below.
}

// The C `sha256_compress` is declared `static` in sha256.c and has no linker
// symbol, so its logic is ported here as a private Rust function. The frozen
// generated code in the translation unit uses the fully unrolled RND form; the
// equivalent loop form from the original source computes exactly the same
// values (all arithmetic is wrapping 32-bit).

#[inline(always)]
fn rotr32(x: u32, n: u32) -> u32 {
    x.rotate_right(n & 31)
}

#[inline(always)]
fn sigma0(x: u32) -> u32 {
    rotr32(x, 2) ^ rotr32(x, 13) ^ rotr32(x, 22)
}

#[inline(always)]
fn sigma1(x: u32) -> u32 {
    rotr32(x, 6) ^ rotr32(x, 11) ^ rotr32(x, 25)
}

#[inline(always)]
fn gamma0(x: u32) -> u32 {
    rotr32(x, 7) ^ rotr32(x, 18) ^ (x >> 3)
}

#[inline(always)]
fn gamma1(x: u32) -> u32 {
    rotr32(x, 17) ^ rotr32(x, 19) ^ (x >> 10)
}

#[inline(always)]
fn ch(x: u32, y: u32, z: u32) -> u32 {
    z ^ (x & (y ^ z))
}

#[inline(always)]
fn maj(x: u32, y: u32, z: u32) -> u32 {
    ((x | y) & z) | (x & y)
}

static K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[inline(always)]
unsafe fn get_be32(d: *const u8) -> u32 {
    ((*d as u32) << 24)
        | ((*d.add(1) as u32) << 16)
        | ((*d.add(2) as u32) << 8)
        | (*d.add(3) as u32)
}

unsafe fn sha256_compress(s: *mut SHA256_CTX, buf: *const u8) {
    let mut st = [0u32; 8];
    let mut w = [0u32; 64];

    let mut i = 0;
    while i < 8 {
        st[i] = (*s).state[i];
        i += 1;
    }

    i = 0;
    while i < 16 {
        w[i] = get_be32(buf.add(4 * i));
        i += 1;
    }

    i = 16;
    while i < 64 {
        w[i] = gamma1(w[i - 2])
            .wrapping_add(w[i - 7])
            .wrapping_add(gamma0(w[i - 15]))
            .wrapping_add(w[i - 16]);
        i += 1;
    }

    i = 0;
    while i < 64 {
        let t0 = st[7]
            .wrapping_add(sigma1(st[4]))
            .wrapping_add(ch(st[4], st[5], st[6]))
            .wrapping_add(K[i])
            .wrapping_add(w[i]);
        let t1 = sigma0(st[0]).wrapping_add(maj(st[0], st[1], st[2]));
        st[3] = st[3].wrapping_add(t0);
        st[7] = t0.wrapping_add(t1);
        let t = st[7];
        st[7] = st[6];
        st[6] = st[5];
        st[5] = st[4];
        st[4] = st[3];
        st[3] = st[2];
        st[2] = st[1];
        st[1] = st[0];
        st[0] = t;
        i += 1;
    }

    i = 0;
    while i < 8 {
        (*s).state[i] = (*s).state[i].wrapping_add(st[i]);
        i += 1;
    }
}

// static inline int min_int(int a, int b) — ported inline.
#[inline(always)]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

extern "C" {
    fn abort() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn SHA256_Update(s: *mut SHA256_CTX, in_: *const u8, inlen: c_ulong) {
    let mut inlen: c_ulong = inlen;
    let mut inptr: *const u8 = in_;
    let mut n: c_ulong;

    // if (s->curlen > sizeof(s->buf)) abort();
    if ((*s).curlen as usize) > 64 {
        abort();
    }
    // if ((s->length + inlen) < s->length) abort();
    if (*s).length.wrapping_add(inlen as u64) < (*s).length {
        abort();
    }

    while inlen > 0 {
        if (*s).curlen == 0 && inlen >= 64 {
            sha256_compress(s, inptr);
            (*s).length = (*s).length.wrapping_add(64u64.wrapping_mul(8));
            inptr = inptr.add(64);
            inlen -= 64;
        } else {
            // n = min_int(inlen, 64 - s->curlen);
            // inlen is unsigned long, the second argument is int; mimic the
            // usual arithmetic conversions applied after min_int returns.
            let b: c_int = 64 - (*s).curlen as c_int;
            let n_signed: c_int = if inlen > i64::MAX as c_ulong {
                b
            } else {
                min_int(inlen as c_int as c_int, b)
            };
            n = if inlen > i64::MAX as c_ulong {
                n_signed as c_ulong
            } else {
                // min_int takes (int, int); when inlen fits in an int the
                // conversion below is exact, otherwise the result is always b.
                if (inlen as u64) <= i32::MAX as u64 {
                    n_signed as c_ulong
                } else {
                    b as c_ulong
                }
            };

            core::ptr::copy_nonoverlapping(
                inptr,
                (*s).buf.as_mut_ptr().add((*s).curlen as usize),
                n as usize,
            );
            (*s).curlen = (*s).curlen.wrapping_add(n as u32);
            inptr = inptr.add(n as usize);
            inlen -= n;
            if (*s).curlen == 64 {
                sha256_compress(s, (*s).buf.as_ptr());
                (*s).length = (*s).length.wrapping_add((8u64).wrapping_mul(64));
                (*s).curlen = 0;
            }
        }
    }
}
}

// ── `if (s->curlen > sizeof(s->buf)` · sha256.c:211-213 · chunk 2160 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->curlen > sizeof(s->buf)) {
//             abort();
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2160 {
// Rust replacement for the curlen sanity check at the top of SHA256_Update.

#[repr(C)]
pub struct SHA256_CTX {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

extern "C" {
    fn abort() -> !;
}

#[no_mangle]
pub extern "C" fn SHA256_Update__part2160(s: *mut SHA256_CTX) {
    unsafe {
        // sizeof(s->buf) == 64 (size_t); curlen is uint32_t promoted to size_t.
        if (*s).curlen as u64 > 64u64 {
            abort();
        }
    }
}
}

// ── `if (s->length + inlen) < s->length)` · sha256.c:214-216 · chunk 2161 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if ((s->length + inlen) < s->length) {
//             abort();
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2161 {
// Port of the overflow check inside SHA256_Update:
//   if ((s->length + inlen) < s->length) { abort(); }

use core::ffi::c_ulong;

#[repr(C)]
pub struct SHA256_CTX {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

extern "C" {
    fn abort() -> !;
}

#[no_mangle]
pub extern "C" fn SHA256_Update__part2161(s: *mut SHA256_CTX, inlen: c_ulong) {
    let length: u64 = unsafe { (*s).length };
    let sum: u64 = length.wrapping_add(inlen as u64);
    if sum < length {
        unsafe {
            abort();
        }
    }
}
}

// ── `while (inlen > 0)` · sha256.c:217-235 · chunk 2162 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (inlen > 0) {
//             if (s->curlen == 0 && inlen >= 64) {
//                 sha256_compress(s, (unsigned char *)in);
//                 s->length += 64 * 8;
//                 in += 64;
//                 inlen -= 64;
//             } else {
//                 n = min_int(inlen, 64 - s->curlen);
//                 memcpy(s->buf + s->curlen, in, (size_t)n);
//                 s->curlen += n;
//                 in += n;
//                 inlen -= n;
//                 if (s->curlen == 64) {
//                     sha256_compress(s, s->buf);
//                     s->length += 8*64;
//                     s->curlen = 0;
//                 }
//            }
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2162 {
// Port of the SHA256_Update update loop (sha256.c lines 217-235).
// sha256_compress is `static` in C (no linker symbol) so its logic is ported
// here as a private Rust function. min_int is `static inline`, also ported.

use core::ffi::{c_int, c_ulong};

#[repr(C)]
pub struct SHA256_CTX {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

static K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[inline]
fn sigma0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

#[inline]
fn sigma1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

#[inline]
fn gamma0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

#[inline]
fn gamma1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

#[inline]
fn ch(x: u32, y: u32, z: u32) -> u32 {
    z ^ (x & (y ^ z))
}

#[inline]
fn maj(x: u32, y: u32, z: u32) -> u32 {
    ((x | y) & z) | (x & y)
}

#[inline]
unsafe fn get_be32(d: *const u8) -> u32 {
    ((*(d as *const u8) as u32) << 24)
        | ((*d.add(1) as u32) << 16)
        | ((*d.add(2) as u32) << 8)
        | (*d.add(3) as u32)
}

// Ported logic of the C `static void sha256_compress(SHA256_CTX *s, unsigned char *buf)`
unsafe fn sha256_compress(s: *mut SHA256_CTX, buf: *const u8) {
    let mut v: [u32; 8] = [0u32; 8];
    let mut w: [u32; 64] = [0u32; 64];
    let mut i: usize = 0;

    // copy state into v
    while i < 8 {
        v[i] = (*s).state[i];
        i += 1;
    }

    // load message block, big endian
    i = 0;
    while i < 16 {
        w[i] = get_be32(buf.add(4 * i));
        i += 1;
    }

    // expand
    i = 16;
    while i < 64 {
        w[i] = gamma1(w[i - 2])
            .wrapping_add(w[i - 7])
            .wrapping_add(gamma0(w[i - 15]))
            .wrapping_add(w[i - 16]);
        i += 1;
    }

    // compress (unrolled C RND sequence, expressed as the equivalent rotation loop)
    let mut j: usize = 0;
    while j < 64 {
        let a = v[0];
        let e = v[4];
        let t0 = v[7]
            .wrapping_add(sigma1(e))
            .wrapping_add(ch(e, v[5], v[6]))
            .wrapping_add(K[j])
            .wrapping_add(w[j]);
        let t1 = sigma0(a).wrapping_add(maj(a, v[1], v[2]));
        v[3] = v[3].wrapping_add(t0);
        v[7] = t0.wrapping_add(t1);
        // rotate: t = S[7]; S[7]=S[6]; ... ; S[0] = t
        let t = v[7];
        v[7] = v[6];
        v[6] = v[5];
        v[5] = v[4];
        v[4] = v[3];
        v[3] = v[2];
        v[2] = v[1];
        v[1] = v[0];
        v[0] = t;
        j += 1;
    }

    // feedback
    i = 0;
    while i < 8 {
        (*s).state[i] = (*s).state[i].wrapping_add(v[i]);
        i += 1;
    }
}

// C: static inline int min_int(int a, int b)
#[inline]
fn min_int(a: c_int, b: c_int) -> c_int {
    if a < b {
        a
    } else {
        b
    }
}

#[no_mangle]
pub unsafe extern "C" fn SHA256_Update__part2162(
    s: *mut SHA256_CTX,
    inp: *const u8,
    inlen: *mut c_ulong,
    n_out: *mut c_ulong,
) {
    let mut in_ptr: *const u8 = inp;
    let mut rem: c_ulong = *inlen;
    let mut n: c_ulong = 0;

    while rem > 0 {
        if (*s).curlen == 0 && rem >= 64 {
            sha256_compress(s, in_ptr);
            (*s).length = (*s).length.wrapping_add(64 * 8);
            in_ptr = in_ptr.wrapping_offset(64);
            rem -= 64;
        } else {
            // n = min_int(inlen, 64 - s->curlen);  (arguments are converted to int)
            let x: c_int = rem as c_int;
            let y: c_int = 64i32.wrapping_sub((*s).curlen as c_int);
            n = min_int(x, y) as c_ulong;
            // memcpy(s->buf + s->curlen, in, (size_t)n);
            let dst = core::ptr::addr_of_mut!((*s).buf) as *mut u8;
            core::ptr::copy_nonoverlapping(in_ptr, dst.wrapping_add((*s).curlen as usize), n as usize);
            (*s).curlen = (*s).curlen.wrapping_add(n as u32);
            in_ptr = in_ptr.wrapping_offset(n as isize);
            rem -= n;
            if (*s).curlen == 64 {
                let bptr = core::ptr::addr_of_mut!((*s).buf) as *const u8;
                sha256_compress(s, bptr);
                (*s).length = (*s).length.wrapping_add(8 * 64);
                (*s).curlen = 0;
            }
        }
    }

    *inlen = rem;
    *n_out = n;
}
}

// ── `SHA256_Final` · sha256.c:243-286 · chunk 2163 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void SHA256_Final(uint8_t *out, SHA256_CTX *s)
//     {
//         int i;
//     
//         if (s->curlen >= sizeof(s->buf)) {
//             abort();
//         }
//     
//     
//     
//         s->length += s->curlen * 8;
//     
//     
//         s->buf[s->curlen++] = (unsigned char)0x80;
//     
//     
//     
//     
//     
//         if (s->curlen > 56) {
//             while (s->curlen < 64) {
//                 s->buf[s->curlen++] = (unsigned char)0;
//             }
//             sha256_compress(s, s->buf);
//             s->curlen = 0;
//         }
//     
//     
//         while (s->curlen < 56) {
//             s->buf[s->curlen++] = (unsigned char)0;
//         }
//     
//     
//         put_be64(s->buf+56, s->length);
//         sha256_compress(s, s->buf);
//     
//     
//         for (i = 0; i < 8; i++) {
//             put_be32(out+(4*i), s->state[i]);
//         }
//     
//     
//     
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2163 {
// Reimplementation of SHA256_Final (and the file-static sha256_compress it calls)
// from sha256.c. The C struct SHA256_CTX and its helpers are mirrored here.

use core::ffi::c_void;

#[repr(C)]
pub struct SHA256_CTX {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

extern "C" {
    fn abort() -> !;
}

#[inline]
fn get_be32(d: *const u8) -> u32 {
    unsafe {
        ((*d as u32) << 24)
            | ((*d.add(1) as u32) << 16)
            | ((*d.add(2) as u32) << 8)
            | (*d.add(3) as u32)
    }
}

#[inline]
fn put_be32(d: *mut u8, v: u32) {
    unsafe {
        *d = (v >> 24) as u8;
        *d.add(1) = (v >> 16) as u8;
        *d.add(2) = (v >> 8) as u8;
        *d.add(3) = v as u8;
    }
}

#[inline]
fn put_be64(d: *mut u8, v: u64) {
    put_be32(d, (v >> 32) as u32);
    put_be32(unsafe { d.add(4) }, v as u32);
}

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[inline]
fn ch(x: u32, y: u32, z: u32) -> u32 {
    z ^ (x & (y ^ z))
}

#[inline]
fn maj(x: u32, y: u32, z: u32) -> u32 {
    ((x | y) & z) | (x & y)
}

#[inline]
fn sigma0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

#[inline]
fn sigma1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

#[inline]
fn gamma0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

#[inline]
fn gamma1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

// Port of the file-static void sha256_compress(SHA256_CTX *s, unsigned char *buf)
fn sha256_compress(s: *mut SHA256_CTX, buf: *const u8) {
    let mut w = [0u32; 64];
    let mut i = 0usize;
    while i < 16 {
        w[i] = get_be32(unsafe { buf.add(4 * i) });
        i += 1;
    }
    i = 16;
    while i < 64 {
        w[i] = gamma1(w[i - 2])
            .wrapping_add(w[i - 7])
            .wrapping_add(gamma0(w[i - 15]))
            .wrapping_add(w[i - 16]);
        i += 1;
    }

    let mut state = [0u32; 8];
    i = 0;
    while i < 8 {
        state[i] = unsafe { (*s).state[i] };
        i += 1;
    }

    i = 0;
    while i < 64 {
        let a = state[0];
        let b = state[1];
        let c = state[2];
        let d = state[3];
        let e = state[4];
        let f = state[5];
        let g = state[6];
        let h = state[7];

        let t0 = h
            .wrapping_add(sigma1(e))
            .wrapping_add(ch(e, f, g))
            .wrapping_add(K[i])
            .wrapping_add(w[i]);
        let t1 = sigma0(a).wrapping_add(maj(a, b, c));

        state[3] = d.wrapping_add(t0);
        state[7] = t0.wrapping_add(t1);

        // rotate: t = S[7]; S[7]=S[6]; ... S[1]=S[0]; S[0]=t;
        state[7] = state[6];
        state[6] = state[5];
        state[5] = state[4];
        state[4] = state[3];
        state[3] = state[2];
        state[2] = state[1];
        state[1] = state[0];
        state[0] = h.wrapping_add(t1).wrapping_add(t0).wrapping_sub(t0).wrapping_add(t0);

        i += 1;
    }

    i = 0;
    while i < 8 {
        unsafe {
            (*s).state[i] = (*s).state[i].wrapping_add(state[i]);
        }
        i += 1;
    }
}

#[no_mangle]
pub extern "C" fn SHA256_Final(out: *mut u8, s: *mut SHA256_CTX) {
    unsafe {
        if (*s).curlen >= 64 {
            abort();
        }

        // increase the length of the message
        let curlen = (*s).curlen;
        (*s).length = (*s)
            .length
            .wrapping_add(curlen.wrapping_mul(8) as u64);

        // append the '1' bit
        let mut cur = (*s).curlen;
        *(*s).buf.as_mut_ptr().add(cur as usize) = 0x80u8;
        cur += 1;

        // if the length is currently above 56 bytes we append zeros then compress
        if cur > 56 {
            while cur < 64 {
                *(*s).buf.as_mut_ptr().add(cur as usize) = 0u8;
                cur += 1;
            }
            sha256_compress(s, (*s).buf.as_ptr());
            cur = 0;
        }

        // pad upto 56 bytes of zeroes
        while cur < 56 {
            *(*s).buf.as_mut_ptr().add(cur as usize) = 0u8;
            cur += 1;
        }
        (*s).curlen = cur;

        // store length
        put_be64((*s).buf.as_mut_ptr().add(56), (*s).length);
        sha256_compress(s, (*s).buf.as_ptr());

        // copy output
        let mut i = 0i32;
        while i < 8 {
            put_be32(out.add(4 * i as usize), (*s).state[i as usize]);
            i += 1;
        }
    }
}

// silence unused warnings for the mirrored c_void import
#[allow(dead_code)]
type UnusedCVoid = c_void;
}

// ── `if (s->curlen >= sizeof(s->buf)` · sha256.c:247-249 · chunk 2164 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->curlen >= sizeof(s->buf)) {
//             abort();
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2164 {
// Port of the length-check guard at the top of SHA256_Final.

#[repr(C)]
pub struct SHA256_CTX {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

extern "C" {
    fn abort() -> !;
}

#[no_mangle]
pub extern "C" fn SHA256_Final__part2164(s: *mut SHA256_CTX) {
    unsafe {
        // s->curlen >= sizeof(s->buf)  (sizeof == 64, compared as unsigned long)
        if (*s).curlen as u64 >= 64u64 {
            abort();
        }
    }
}
}

// ── `s->length += s->curlen * 8; s->buf[s->curlen++] = (unsigned char)0x80;` · sha256.c:253-256 · chunk 2165 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     s->length += s->curlen * 8;
//     
//     
//         s->buf[s->curlen++] = (unsigned char)0x80;
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2165 {
// Port of the SHA256_Final finalization prologue:
//   s->length += s->curlen * 8;
//   s->buf[s->curlen++] = (unsigned char)0x80;

#[repr(C)]
pub struct SHA256_CTX {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

#[no_mangle]
pub unsafe extern "C" fn SHA256_Final__part2165(s: *mut SHA256_CTX) {
    let c = (*s).curlen;
    // curlen * 8 is computed in unsigned int (32-bit) arithmetic in C, then
    // widened to uint64_t for the addition.
    let bits: u32 = c.wrapping_mul(8);
    (*s).length = (*s).length.wrapping_add(bits as u64);

    let idx = (*s).curlen as usize;
    let buf = (*s).buf.as_mut_ptr();
    // Mirrors buf[curlen] = 0x80 with a raw store (no bounds check / panic).
    core::ptr::write_volatile(buf.add(idx), 0x80u8);
    (*s).curlen = (*s).curlen.wrapping_add(1);
}
}

// ── `if (s->curlen > 56)` · sha256.c:262-268 · chunk 2166 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     if (s->curlen > 56) {
//             while (s->curlen < 64) {
//                 s->buf[s->curlen++] = (unsigned char)0;
//             }
//             sha256_compress(s, s->buf);
//             s->curlen = 0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2166 {
// Port of the "length > 56 -> pad to 64, compress, reset" branch of SHA256_Final
// (sha256.c lines 262-268). sha256_compress is a static C function with no
// linker symbol, so its logic is reimplemented here privately.

#[repr(C)]
pub struct SHA256_CTX {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

static K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
    0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
    0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
    0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
    0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
    0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[inline]
fn ror(x: u32, n: u32) -> u32 {
    x.rotate_right(n)
}

#[inline]
fn ch(x: u32, y: u32, z: u32) -> u32 {
    z ^ (x & (y ^ z))
}

#[inline]
fn maj(x: u32, y: u32, z: u32) -> u32 {
    ((x | y) & z) | (x & y)
}

#[inline]
fn sigma0(x: u32) -> u32 {
    ror(x, 2) ^ ror(x, 13) ^ ror(x, 22)
}

#[inline]
fn sigma1(x: u32) -> u32 {
    ror(x, 6) ^ ror(x, 11) ^ ror(x, 25)
}

#[inline]
fn gamma0(x: u32) -> u32 {
    ror(x, 7) ^ ror(x, 18) ^ (x >> 3)
}

#[inline]
fn gamma1(x: u32) -> u32 {
    ror(x, 17) ^ ror(x, 19) ^ (x >> 10)
}

#[inline]
unsafe fn get_be32(d: *const u8) -> u32 {
    ((*d as u32) << 24)
        | ((*d.add(1) as u32) << 16)
        | ((*d.add(2) as u32) << 8)
        | (*d.add(3) as u32)
}

unsafe fn sha256_compress(s: *mut SHA256_CTX, buf: *const u8) {
    let mut sv = [0u32; 8];
    let mut w = [0u32; 64];

    let mut i: usize = 0;
    while i < 8 {
        sv[i] = (*s).state[i];
        i += 1;
    }

    let mut i: usize = 0;
    while i < 16 {
        w[i] = get_be32(buf.add(4 * i));
        i += 1;
    }

    let mut i: usize = 16;
    while i < 64 {
        w[i] = gamma1(w[i - 2])
            .wrapping_add(w[i - 7])
            .wrapping_add(gamma0(w[i - 15]))
            .wrapping_add(w[i - 16]);
        i += 1;
    }

    let mut i: usize = 0;
    while i < 64 {
        let r = i & 7;
        let a = sv[(8 - r) & 7];
        let b = sv[(9 - r) & 7];
        let c = sv[(10 - r) & 7];
        let d = sv[(11 - r) & 7];
        let e = sv[(12 - r) & 7];
        let f = sv[(13 - r) & 7];
        let g = sv[(14 - r) & 7];
        let h = sv[(15 - r) & 7];

        let t0 = h
            .wrapping_add(sigma1(e))
            .wrapping_add(ch(e, f, g))
            .wrapping_add(K[i])
            .wrapping_add(w[i]);
        let t1 = sigma0(a).wrapping_add(maj(a, b, c));

        sv[(11 - r) & 7] = d.wrapping_add(t0);
        sv[(15 - r) & 7] = t0.wrapping_add(t1);
        i += 1;
    }

    let mut i: usize = 0;
    while i < 8 {
        (*s).state[i] = (*s).state[i].wrapping_add(sv[i]);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn SHA256_Final__part2166(s: *mut SHA256_CTX) {
    if (*s).curlen > 56 {
        while (*s).curlen < 64 {
            let c = (*s).curlen;
            (*s).buf[(c & 63) as usize] = 0u8;
            (*s).curlen = c.wrapping_add(1);
        }
        let bufptr = core::ptr::addr_of_mut!((*s).buf) as *const u8;
        sha256_compress(s, bufptr);
        (*s).curlen = 0;
    }
}
}

// ── `while (s->curlen < 56)` · sha256.c:271-273 · chunk 2167 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     while (s->curlen < 56) {
//             s->buf[s->curlen++] = (unsigned char)0;
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2167 {
// Port of the padding-zero loop from SHA256_Final (sha256.c lines 271-273).

#[repr(C)]
pub struct SHA256_CTX {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

#[no_mangle]
pub unsafe extern "C" fn SHA256_Final__part2167(s: *mut SHA256_CTX) {
    if s.is_null() {
        return;
    }
    let mut curlen = (*s).curlen;
    while curlen < 56 {
        let idx = curlen as usize;
        if idx < 64 {
            let bp: *mut u8 = core::ptr::addr_of_mut!((*s).buf) as *mut u8;
            core::ptr::write(bp.add(idx), 0u8);
        }
        curlen = curlen.wrapping_add(1);
        (*s).curlen = curlen;
    }
}
}

// ── `put_be64(s->buf+56, s->length); sha256_compress(s, s->buf);` · sha256.c:276-277 · chunk 2168 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     put_be64(s->buf+56, s->length);
//         sha256_compress(s, s->buf);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2168 {
// Rust rewrite of the tail of SHA256_Final: store the 64-bit length big-endian
// into s->buf[56..64] and then run one final compression over s->buf.
//
// sha256_compress is a `static` function in the C translation unit (no linker
// symbol), so its logic is ported here as a private Rust function.

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct SHA256_CTX {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

static K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[inline]
unsafe fn get_be32(d: *const u8) -> u32 {
    ((*d as u32) << 24)
        | ((*d.add(1) as u32) << 16)
        | ((*d.add(2) as u32) << 8)
        | (*d.add(3) as u32)
}

#[inline]
unsafe fn put_be32(d: *mut u8, v: u32) {
    *d = (v >> 24) as u8;
    *d.add(1) = (v >> 16) as u8;
    *d.add(2) = (v >> 8) as u8;
    *d.add(3) = v as u8;
}

#[inline]
unsafe fn put_be64(d: *mut u8, v: u64) {
    put_be32(d, (v >> 32) as u32);
    put_be32(d.add(4), v as u32);
}

// Port of the static C function sha256_compress(SHA256_CTX *s, unsigned char *buf)
unsafe fn sha256_compress(s: *mut SHA256_CTX, buf: *mut u8) {
    let mut w = [0u32; 64];
    let mut sv = [0u32; 8];

    let mut i = 0usize;
    while i < 8 {
        sv[i] = (*s).state[i];
        i += 1;
    }

    i = 0;
    while i < 16 {
        w[i] = get_be32(buf.add(4 * i) as *const u8);
        i += 1;
    }

    i = 16;
    while i < 64 {
        let x = w[i - 2];
        let y = w[i - 15];
        let gamma1 = x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10);
        let gamma0 = y.rotate_right(7) ^ y.rotate_right(18) ^ (y >> 3);
        w[i] = gamma1
            .wrapping_add(w[i - 7])
            .wrapping_add(gamma0)
            .wrapping_add(w[i - 16]);
        i += 1;
    }

    let mut a = sv[0];
    let mut b = sv[1];
    let mut c = sv[2];
    let mut d = sv[3];
    let mut e = sv[4];
    let mut f = sv[5];
    let mut g = sv[6];
    let mut h = sv[7];

    let mut r = 0usize;
    while r < 64 {
        let sigma1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = g ^ (e & (f ^ g));
        let t0 = h
            .wrapping_add(sigma1)
            .wrapping_add(ch)
            .wrapping_add(K[r])
            .wrapping_add(w[r]);
        let sigma0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj = (a | b) & c | (a & b);
        let t1 = sigma0.wrapping_add(maj);

        d = d.wrapping_add(t0);
        h = t0.wrapping_add(t1);

        // rotate the working variables: (a,b,c,d,e,f,g,h) <- (h',a,b,c,d',e,f,g)
        let nt = h;
        h = g;
        g = f;
        f = e;
        e = d;
        d = c;
        c = b;
        b = a;
        a = nt;

        r += 1;
    }

    i = 0;
    while i < 8 {
        (*s).state[i] = (*s).state[i].wrapping_add(sv[i]);
        i += 1;
    }

    // fold the final working variables into the sv array in S[0..7] order
    let _ = (a, b, c, d, e, f, g, h);
}

// S[0..7] after the rounds is the final working state (a,b,c,d,e,f,g,h).
unsafe fn sha256_compress_entry(s: *mut SHA256_CTX, buf: *mut u8) {
    sha256_compress_impl(s, buf);
}

unsafe fn sha256_compress_impl(s: *mut SHA256_CTX, buf: *mut u8) {
    let mut w = [0u32; 64];
    let mut sv = [0u32; 8];

    let mut i = 0usize;
    while i < 8 {
        sv[i] = (*s).state[i];
        i += 1;
    }

    i = 0;
    while i < 16 {
        w[i] = get_be32(buf.add(4 * i) as *const u8);
        i += 1;
    }

    i = 16;
    while i < 64 {
        let x = w[i - 2];
        let y = w[i - 15];
        let gamma1 = x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10);
        let gamma0 = y.rotate_right(7) ^ y.rotate_right(18) ^ (y >> 3);
        w[i] = gamma1
            .wrapping_add(w[i - 7])
            .wrapping_add(gamma0)
            .wrapping_add(w[i - 16]);
        i += 1;
    }

    let mut a = sv[0];
    let mut b = sv[1];
    let mut c = sv[2];
    let mut d = sv[3];
    let mut e = sv[4];
    let mut f = sv[5];
    let mut g = sv[6];
    let mut h = sv[7];

    let mut r = 0usize;
    while r < 64 {
        let sigma1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = g ^ (e & (f ^ g));
        let t0 = h
            .wrapping_add(sigma1)
            .wrapping_add(ch)
            .wrapping_add(K[r])
            .wrapping_add(w[r]);
        let sigma0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj = (a | b) & c | (a & b);
        let t1 = sigma0.wrapping_add(maj);

        d = d.wrapping_add(t0);
        h = t0.wrapping_add(t1);

        let nt = h;
        h = g;
        g = f;
        f = e;
        e = d;
        d = c;
        c = b;
        b = a;
        a = nt;

        r += 1;
    }

    sv[0] = a;
    sv[1] = b;
    sv[2] = c;
    sv[3] = d;
    sv[4] = e;
    sv[5] = f;
    sv[6] = g;
    sv[7] = h;

    i = 0;
    while i < 8 {
        (*s).state[i] = (*s).state[i].wrapping_add(sv[i]);
        i += 1;
    }
}

#[no_mangle]
pub extern "C" fn SHA256_Final__part2168(s: *mut SHA256_CTX) {
    if s.is_null() {
        return;
    }
    unsafe {
        let buf = (*s).buf.as_mut_ptr();
        put_be64(buf.add(56), (*s).length);
        sha256_compress_entry(s, (*s).buf.as_mut_ptr());
    }
}
}

// ── `for (i = 0; i < 8; i++)` · sha256.c:280-282 · chunk 2169 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     for (i = 0; i < 8; i++) {
//             put_be32(out+(4*i), s->state[i]);
//         }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2169 {
// Port of SHA256_Final's final output loop.
// put_be32 is a `static inline` C helper with no linker symbol, so its logic is
// reproduced here as a private Rust function.

#[repr(C)]
pub struct SHA256_CTX {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

#[inline]
fn put_be32(d: *mut u8, v: u32) {
    unsafe {
        *d.add(0) = (v >> 24) as u8;
        *d.add(1) = (v >> 16) as u8;
        *d.add(2) = (v >> 8) as u8;
        *d.add(3) = v as u8;
    }
}

#[no_mangle]
pub extern "C" fn SHA256_Final__part2169(out: *mut u8, s: *mut SHA256_CTX) {
    let mut i: i32 = 0;
    while i < 8 {
        unsafe {
            let state_ptr = core::ptr::addr_of!((*s).state) as *const u32;
            let v = *state_ptr.add(i as usize);
            put_be32(out.offset((4 * i) as isize), v);
        }
        i += 1;
    }
}
}

// ── `SHA256` · sha256.c:288-295 · chunk 2170 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     void SHA256(const uint8_t *buf, int buf_len, uint8_t *out)
//     {
//         SHA256_CTX ctx;
//     
//         SHA256_Init(&ctx);
//         SHA256_Update(&ctx, buf, buf_len);
//         SHA256_Final(out, &ctx);
//     }
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2170 {
use core::ffi::{c_int, c_ulong};

#[repr(C)]
struct SHA256_CTX {
    length: u64,
    state: [u32; 8],
    curlen: u32,
    buf: [u8; 64],
}

extern "C" {
    fn SHA256_Init(s: *mut SHA256_CTX);
    fn SHA256_Update(s: *mut SHA256_CTX, in_: *const u8, inlen: c_ulong);
    fn SHA256_Final(out: *mut u8, s: *mut SHA256_CTX);
}

#[no_mangle]
pub extern "C" fn SHA256(buf: *const u8, buf_len: c_int, out: *mut u8) {
    let mut ctx = SHA256_CTX {
        length: 0,
        state: [0u32; 8],
        curlen: 0,
        buf: [0u8; 64],
    };
    unsafe {
        SHA256_Init(&mut ctx);
        SHA256_Update(&mut ctx, buf, buf_len as c_ulong);
        SHA256_Final(out, &mut ctx);
    }
}
}

// ── `SHA256_CTX ctx; SHA256_Init(&ctx); SHA256_Update(&ctx, buf, buf_len); SHA256_Final(out, &ctx);` · sha256.c:290-294 · chunk 2171 · deepseek/deepseek-v4.1-flash-1017 · 1 attempt(s) · Unvalidated
//
// Original C:
//     SHA256_CTX ctx;
//     
//         SHA256_Init(&ctx);
//         SHA256_Update(&ctx, buf, buf_len);
//         SHA256_Final(out, &ctx);
// Its own module: each chunk's file brings its own `use`s and C type stubs.
pub mod chunk2171 {
// Port of the whole body of C SHA256(): run the streaming SHA-256 pipeline
// over a single in-memory buffer with a stack-local context.
use core::ffi::{c_int, c_ulong};

// Mirror of the C struct:
// typedef struct { uint64_t length; uint32_t state[8], curlen; uint8_t buf[64]; } SHA256_CTX;
#[repr(C)]
pub struct SHA256Ctx {
    pub length: u64,
    pub state: [u32; 8],
    pub curlen: u32,
    pub buf: [u8; 64],
}

extern "C" {
    fn SHA256_Init(s: *mut SHA256Ctx);
    fn SHA256_Update(s: *mut SHA256Ctx, input: *const u8, inlen: c_ulong);
    fn SHA256_Final(out: *mut u8, s: *mut SHA256Ctx);
}

#[no_mangle]
pub extern "C" fn SHA256__part2171(buf: *const u8, buf_len: c_int, out: *mut u8) {
    let mut ctx = SHA256Ctx {
        length: 0,
        state: [0u32; 8],
        curlen: 0,
        buf: [0u8; 64],
    };

    unsafe {
        SHA256_Init(&mut ctx);
        // int -> unsigned long conversion, as C performs it (sign-extending -1 to ULONG_MAX).
        SHA256_Update(&mut ctx, buf, buf_len as c_ulong);
        SHA256_Final(out, &mut ctx);
    }
}
}

