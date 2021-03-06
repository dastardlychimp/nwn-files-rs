
#![allow(non_camel_case_types)]

use std::convert::From;

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceType {
    Unknown = 0,
    bmp = 1,
    tga = 2,
    wav = 4,
    plt = 6,
    ini = 7,
    txt = 10,
    mdl = 2002,
    nss = 2009,
    ncs = 2010,
    are = 2012,
    set = 2013,
    ifo = 2014,
    bic = 2015,
    wok = 2016,
    x2da = 2017,
    txi = 2020,
    git = 2023,
    uti = 2025,
    utc = 2027,
    dlg = 2029,
    itp = 2030,
    utt = 2032,
    dds = 2033,
    uts = 2035,
    ltr = 2036,
    gff = 2037,
    fac = 2038,
    ute = 2040,
    utd = 2042,
    utp = 2044,
    dft = 2045,
    gic = 2046,
    gui = 2047,
    utm = 2051,
    dwk = 2052,
    pwk = 2053,
    jrl = 2056,
    utw = 2058,
    ssf = 2060,
    ndb = 2064,
    ptm = 2065,
    ptt = 2066,
}

impl From<u16> for ResourceType {
    fn from(i: u16) -> Self {
        match i {
            1 => ResourceType::bmp,
            2 => ResourceType::tga,
            4 => ResourceType::wav,
            6 => ResourceType::plt,
            7 => ResourceType::ini,
            10 => ResourceType::txt,
            2002 => ResourceType::mdl,
            2009 => ResourceType::nss,
            2010 => ResourceType::ncs,
            2012 => ResourceType::are,
            2013 => ResourceType::set,
            2014 => ResourceType::ifo,
            2015 => ResourceType::bic,
            2016 => ResourceType::wok,
            2017 => ResourceType::x2da,
            2020 => ResourceType::txi,
            2023 => ResourceType::git,
            2025 => ResourceType::uti,
            2027 => ResourceType::utc,
            2029 => ResourceType::dlg,
            2030 => ResourceType::itp,
            2032 => ResourceType::utt,
            2033 => ResourceType::dds,
            2035 => ResourceType::uts,
            2036 => ResourceType::ltr,
            2037 => ResourceType::gff,
            2038 => ResourceType::fac,
            2040 => ResourceType::ute,
            2042 => ResourceType::utd,
            2044 => ResourceType::utp,
            2045 => ResourceType::dft,
            2046 => ResourceType::gic,
            2047 => ResourceType::gui,
            2051 => ResourceType::utm,
            2052 => ResourceType::dwk,
            2053 => ResourceType::pwk,
            2056 => ResourceType::jrl,
            2058 => ResourceType::utw,
            2060 => ResourceType::ssf,
            2064 => ResourceType::ndb,
            2065 => ResourceType::ptm,
            2066 => ResourceType::ptt,
            _ => ResourceType::Unknown,
        }
    }
}


impl From<u32> for ResourceType {
    fn from(i: u32) -> Self {
        match i {
            1 => ResourceType::bmp,
            2 => ResourceType::tga,
            4 => ResourceType::wav,
            6 => ResourceType::plt,
            7 => ResourceType::ini,
            10 => ResourceType::txt,
            2002 => ResourceType::mdl,
            2009 => ResourceType::nss,
            2010 => ResourceType::ncs,
            2012 => ResourceType::are,
            2013 => ResourceType::set,
            2014 => ResourceType::ifo,
            2015 => ResourceType::bic,
            2016 => ResourceType::wok,
            2017 => ResourceType::x2da,
            2020 => ResourceType::txi,
            2023 => ResourceType::git,
            2025 => ResourceType::uti,
            2027 => ResourceType::utc,
            2029 => ResourceType::dlg,
            2030 => ResourceType::itp,
            2032 => ResourceType::utt,
            2033 => ResourceType::dds,
            2035 => ResourceType::uts,
            2036 => ResourceType::ltr,
            2037 => ResourceType::gff,
            2038 => ResourceType::fac,
            2040 => ResourceType::ute,
            2042 => ResourceType::utd,
            2044 => ResourceType::utp,
            2045 => ResourceType::dft,
            2046 => ResourceType::gic,
            2047 => ResourceType::gui,
            2051 => ResourceType::utm,
            2052 => ResourceType::dwk,
            2053 => ResourceType::pwk,
            2056 => ResourceType::jrl,
            2058 => ResourceType::utw,
            2060 => ResourceType::ssf,
            2064 => ResourceType::ndb,
            2065 => ResourceType::ptm,
            2066 => ResourceType::ptt,
            _ => ResourceType::Unknown,
        }
    }
}