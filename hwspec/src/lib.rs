/// This enum provides a type-safe way to handle audio devices
#[derive(Debug)]
pub enum AudioCard {
    AdLib,
    CMS,
    SB10,
    SB15,
    SB20,
    SBPRO,
    SBPRO2,
    SB16,
    SBAWE32,
    MT32,
    LAPC1,
    MPU401,
    SC55,
    SCC1,
    COVOX,
    GUS,
    GUSMAX,
}
