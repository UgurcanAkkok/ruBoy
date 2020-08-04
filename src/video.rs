/*
struct video {
    struct screen_buf {
        union {
            uint16_t pixels [256][256];
            struct {
                uint16_t pixels[8][8];
            } tiles [32][32];
        };
    } screen_buf;

    struct registers {
        uint8_t lcdc; /* LCD Control at 0xFF40 */
        uint8_t stat; /* LCD Status at 0xFF41 */
        uint8_t scy; /* Scroll Y at 0xFF42 */
        uint8_t scx; /* Scroll X at 0xFF43 */
        uint8_t ly; /* LCD Current Scanned Line at 0xFF44 */
        uint8_t lyc; /* LY Compare at 0xFF45 */
        uint8_t bgp; /* BG Palette Data at 0xFF47 */
        /* When value of OAM palette selection flag is 0, 
         * this value is used, otherwise obp1
         */
        uint8_t obp0; /* at 0xFF48 */
        uint8_t obp1; /* at 0xFF49 */
        uint8_t wy; /* Window Y coordinate at 0xFF4A */
        uint8_t wx; /* Window X coordinate at 0xFF4B */
    } reg;


} video;
 */
