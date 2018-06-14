use crate::*;

#[repr(u32)] enum CFStringEncoding {
  kCFStringEncodingMacRoman = 0,
  kCFStringEncodingMacJapanese = 1,
  kCFStringEncodingMacChineseTrad = 2,
  kCFStringEncodingMacKorean = 3,
  kCFStringEncodingMacArabic = 4,
  kCFStringEncodingMacHebrew = 5,
  kCFStringEncodingMacGreek = 6,
  kCFStringEncodingMacCyrillic = 7,
  kCFStringEncodingMacDevanagari = 9,
  kCFStringEncodingMacGurmukhi = 10,
  kCFStringEncodingMacGujarati = 11,
  kCFStringEncodingMacOriya = 12,
  kCFStringEncodingMacBengali = 13,
  kCFStringEncodingMacTamil = 14,
  kCFStringEncodingMacTelugu = 15,
  kCFStringEncodingMacKannada = 16,
  kCFStringEncodingMacMalayalam = 17,
  kCFStringEncodingMacSinhalese = 18,
  kCFStringEncodingMacBurmese = 19,
  kCFStringEncodingMacKhmer = 20,
  kCFStringEncodingMacThai = 21,
  kCFStringEncodingMacLaotian = 22,
  kCFStringEncodingMacGeorgian = 23,
  kCFStringEncodingMacArmenian = 24,
  kCFStringEncodingMacChineseSimp = 25,
  kCFStringEncodingMacTibetan = 26,
  kCFStringEncodingMacMongolian = 27,
  kCFStringEncodingMacEthiopic = 28,
  kCFStringEncodingMacCentralEurRoman = 29,
  kCFStringEncodingMacVietnamese = 30,
  kCFStringEncodingMacExtArabic = 31,

  kCFStringEncodingMacSymbol = 33,
  kCFStringEncodingMacDingbats = 34,
  kCFStringEncodingMacTurkish = 35,
  kCFStringEncodingMacCroatian = 36,
  kCFStringEncodingMacIcelandic = 37,
  kCFStringEncodingMacRomanian = 38,
  kCFStringEncodingMacCeltic = 39,
  kCFStringEncodingMacGaelic = 40,

  kCFStringEncodingMacFarsi = 0x8C,

  kCFStringEncodingMacUkrainian = 0x98,

  kCFStringEncodingMacInuit = 0xEC,
  kCFStringEncodingMacVT100 = 0xFC,

  kCFStringEncodingMacHFS = 0xFF,

  // Alias for UTF16, kCFStringEncodingUnicode = 0x0100,
  kCFStringEncodingUTF16 = 0x0100,
  kCFStringEncodingUTF16BE = 0x10000100,
  kCFStringEncodingUTF16LE = 0x14000100,

  kCFStringEncodingISOLatin1 = 0x0201,
  kCFStringEncodingISOLatin2 = 0x0202,
  kCFStringEncodingISOLatin3 = 0x0203,
  kCFStringEncodingISOLatin4 = 0x0204,
  kCFStringEncodingISOLatinCyrillic = 0x0205,
  kCFStringEncodingISOLatinArabic = 0x0206,
  kCFStringEncodingISOLatinGreek = 0x0207,
  kCFStringEncodingISOLatinHebrew = 0x0208,
  kCFStringEncodingISOLatin5 = 0x0209,
  kCFStringEncodingISOLatin6 = 0x020A,
  kCFStringEncodingISOLatinThai = 0x020B,
  kCFStringEncodingISOLatin7 = 0x020D,
  kCFStringEncodingISOLatin8 = 0x020E,
  kCFStringEncodingISOLatin9 = 0x020F,
  kCFStringEncodingISOLatin10 = 0x0210,

  kCFStringEncodingDOSLatinUS = 0x0400,
  kCFStringEncodingDOSGreek = 0x0405,
  kCFStringEncodingDOSBalticRim = 0x0406,
  kCFStringEncodingDOSLatin1 = 0x0410,
  kCFStringEncodingDOSGreek1 = 0x0411,
  kCFStringEncodingDOSLatin2 = 0x0412,
  kCFStringEncodingDOSCyrillic = 0x0413,
  kCFStringEncodingDOSTurkish = 0x0414,
  kCFStringEncodingDOSPortuguese = 0x0415,
  kCFStringEncodingDOSIcelandic = 0x0416,
  kCFStringEncodingDOSHebrew = 0x0417,
  kCFStringEncodingDOSCanadianFrench = 0x0418,
  kCFStringEncodingDOSArabic = 0x0419,
  kCFStringEncodingDOSNordic = 0x041A,
  kCFStringEncodingDOSRussian = 0x041B,
  kCFStringEncodingDOSGreek2 = 0x041C,
  kCFStringEncodingDOSThai = 0x041D,
  kCFStringEncodingDOSJapanese = 0x0420,
  kCFStringEncodingDOSChineseSimplif = 0x0421,
  kCFStringEncodingDOSKorean = 0x0422,
  kCFStringEncodingDOSChineseTrad = 0x0423,

  kCFStringEncodingWindowsLatin1 = 0x0500,
  kCFStringEncodingWindowsLatin2 = 0x0501,
  kCFStringEncodingWindowsCyrillic = 0x0502,
  kCFStringEncodingWindowsGreek = 0x0503,
  kCFStringEncodingWindowsLatin5 = 0x0504,
  kCFStringEncodingWindowsHebrew = 0x0505,
  kCFStringEncodingWindowsArabic = 0x0506,
  kCFStringEncodingWindowsBalticRim = 0x0507,
  kCFStringEncodingWindowsVietnamese = 0x0508,
  kCFStringEncodingWindowsKoreanJohab = 0x0510,

  kCFStringEncodingASCII = 0x0600,
  kCFStringEncodingANSEL = 0x0601,
  kCFStringEncodingJIS_X0201_76 = 0x0620,
  kCFStringEncodingJIS_X0208_83 = 0x0621,
  kCFStringEncodingJIS_X0208_90 = 0x0622,
  kCFStringEncodingJIS_X0212_90 = 0x0623,
  kCFStringEncodingJIS_C6226_78 = 0x0624,
  kCFStringEncodingShiftJIS_X0213 = 0x0628,
  kCFStringEncodingShiftJIS_X0213_MenKuTen = 0x0629,
  kCFStringEncodingGB_2312_80 = 0x0630,
  kCFStringEncodingGBK_95 = 0x0631,
  kCFStringEncodingGB_18030_2000 = 0x0632,
  kCFStringEncodingKSC_5601_87 = 0x0640,
  kCFStringEncodingKSC_5601_92_Johab = 0x0641,
  kCFStringEncodingCNS_11643_92_P1 = 0x0651,
  kCFStringEncodingCNS_11643_92_P2 = 0x0652,
  kCFStringEncodingCNS_11643_92_P3 = 0x0653,

  kCFStringEncodingISO_2022_JP = 0x0820,
  kCFStringEncodingISO_2022_JP_2 = 0x0821,
  kCFStringEncodingISO_2022_JP_1 = 0x0822,
  kCFStringEncodingISO_2022_JP_3 = 0x0823,
  kCFStringEncodingISO_2022_CN = 0x0830,
  kCFStringEncodingISO_2022_CN_EXT = 0x0831,
  kCFStringEncodingISO_2022_KR = 0x0840,

  kCFStringEncodingEUC_JP = 0x0920,
  kCFStringEncodingEUC_CN = 0x0930,
  kCFStringEncodingEUC_TW = 0x0931,
  kCFStringEncodingEUC_KR = 0x0940,

  kCFStringEncodingShiftJIS = 0x0A01,
  kCFStringEncodingKOI8_R = 0x0A02,
  kCFStringEncodingBig5 = 0x0A03,
  kCFStringEncodingMacRomanLatin1 = 0x0A04,
  kCFStringEncodingHZ_GB_2312 = 0x0A05,
  kCFStringEncodingBig5_HKSCS_1999 = 0x0A06,
  kCFStringEncodingVISCII = 0x0A07,
  kCFStringEncodingKOI8_U = 0x0A08,
  kCFStringEncodingBig5_E = 0x0A09,

  kCFStringEncodingNextStepLatin = 0x0B01,
  kCFStringEncodingNextStepJapanese = 0x0B02,

  kCFStringEncodingEBCDIC_US = 0x0C01,
  kCFStringEncodingEBCDIC_CP037 = 0x0C02,

  kCFStringEncodingUTF7 = 0x04000100,
  kCFStringEncodingUTF7_IMAP = 0x0A10,

  kCFStringEncodingUTF8 = 0x08000100,
  kCFStringEncodingNonLossyASCII = 0x0BFF,

  kCFStringEncodingUTF32 = 0x0c000100,
  kCFStringEncodingUTF32BE = 0x18000100,
  kCFStringEncodingUTF32LE = 0x1c000100
}

// Should be a bitfield
// #[repr(u64)] enum CFStringCompareFlags {
//   kCFCompareCaseInsensitive = 1,	
//   kCFCompareBackwards = 4,
//   kCFCompareAnchored = 8,
//   kCFCompareNonliteral = 16,
//   kCFCompareLocalized = 32,
//   kCFCompareNumerically = 64,
//   kCFCompareDiacriticInsensitive = 128,
//   kCFCompareWidthInsensitive = 256,
//   kCFCompareForcedOrdering = 512
// }

#[repr(u64)] enum CFStringNormalizationForm {
  kCFStringNormalizationFormD = 0,
  kCFStringNormalizationFormKD = 1,
  kCFStringNormalizationFormC = 2,
  kCFStringNormalizationFormKC = 3
}


mod ext {
  use crate::*;

  extern {
    pub fn CFStringGetTypeID() -> CFTypeID;

    // CFStringRef CFStringCreateWithPascalString(CFAllocatorRef alloc, ConstStr255Param pStr, CFStringEncoding encoding);
    // CFStringRef CFStringCreateWithCString(CFAllocatorRef alloc, const char *cStr, CFStringEncoding encoding);
    // CFStringRef CFStringCreateWithBytes(CFAllocatorRef alloc, const UInt8 *bytes, CFIndex numBytes, CFStringEncoding encoding, Boolean isExternalRepresentation);
    // CFStringRef CFStringCreateWithCharacters(CFAllocatorRef alloc, const UniChar *chars, CFIndex numChars);
    // CFStringRef CFStringCreateWithPascalStringNoCopy(CFAllocatorRef alloc, ConstStr255Param pStr, CFStringEncoding encoding, CFAllocatorRef contentsDeallocator);
    // CFStringRef CFStringCreateWithCStringNoCopy(CFAllocatorRef alloc, const char *cStr, CFStringEncoding encoding, CFAllocatorRef contentsDeallocator);
    // CFStringRef CFStringCreateWithBytesNoCopy(CFAllocatorRef alloc, const UInt8 *bytes, CFIndex numBytes, CFStringEncoding encoding, Boolean isExternalRepresentation, CFAllocatorRef contentsDeallocator);
    // CFStringRef CFStringCreateWithCharactersNoCopy(CFAllocatorRef alloc, const UniChar *chars, CFIndex numChars, CFAllocatorRef contentsDeallocator);
    // CFStringRef CFStringCreateWithSubstring(CFAllocatorRef alloc, CFStringRef str, CFRange range);
    // CFStringRef CFStringCreateCopy(CFAllocatorRef alloc, CFStringRef theString);
    // CFStringRef CFStringCreateWithFormat(CFAllocatorRef alloc, CFDictionaryRef formatOptions, CFStringRef format, ...) CF_FORMAT_FUNCTION(3,4);
    // CFStringRef CFStringCreateWithFormatAndArguments(CFAllocatorRef alloc, CFDictionaryRef formatOptions, CFStringRef format, va_list arguments) CF_FORMAT_FUNCTION(3,0);
    // CFMutableStringRef CFStringCreateMutable(CFAllocatorRef alloc, CFIndex maxLength);
    // CFMutableStringRef CFStringCreateMutableCopy(CFAllocatorRef alloc, CFIndex maxLength, CFStringRef theString);
    // CFMutableStringRef CFStringCreateMutableWithExternalCharactersNoCopy(CFAllocatorRef alloc, UniChar *chars, CFIndex numChars, CFIndex capacity, CFAllocatorRef externalCharactersAllocator);
    // 
    // CFIndex CFStringGetLength(CFStringRef theString);
    // UniChar CFStringGetCharacterAtIndex(CFStringRef theString, CFIndex idx);
    // void CFStringGetCharacters(CFStringRef theString, CFRange range, UniChar *buffer);
    // Boolean CFStringGetPascalString(CFStringRef theString, StringPtr buffer, CFIndex bufferSize, CFStringEncoding encoding);
    // Boolean CFStringGetCString(CFStringRef theString, char *buffer, CFIndex bufferSize, CFStringEncoding encoding);
    // ConstStringPtr CFStringGetPascalStringPtr(CFStringRef theString, CFStringEncoding encoding);
    // const char *CFStringGetCStringPtr(CFStringRef theString, CFStringEncoding encoding);
    // const UniChar *CFStringGetCharactersPtr(CFStringRef theString);
    // CFIndex CFStringGetBytes(CFStringRef theString, CFRange range, CFStringEncoding encoding, UInt8 lossByte, Boolean isExternalRepresentation, UInt8 *buffer, CFIndex maxBufLen, CFIndex *usedBufLen);
    // CFStringRef CFStringCreateFromExternalRepresentation(CFAllocatorRef alloc, CFDataRef data, CFStringEncoding encoding);	/* May return NULL on conversion error */
    // CFDataRef CFStringCreateExternalRepresentation(CFAllocatorRef alloc, CFStringRef theString, CFStringEncoding encoding, UInt8 lossByte);	/* May return NULL on conversion error */	
    // CFStringEncoding CFStringGetSmallestEncoding(CFStringRef theString);	/* Result in O(n) time max */
    // CFStringEncoding CFStringGetFastestEncoding(CFStringRef theString);	/* Result in O(1) time max */
    // CFStringEncoding CFStringGetSystemEncoding(void);		/* The default encoding for the system; untagged 8-bit characters are usually in this encoding */
    // CFIndex CFStringGetMaximumSizeForEncoding(CFIndex length, CFStringEncoding encoding);	/* Max bytes a string of specified length (in UniChars) will take up if encoded */
    // Boolean CFStringGetFileSystemRepresentation(CFStringRef string, char *buffer, CFIndex maxBufLen);
    // CFIndex CFStringGetMaximumSizeOfFileSystemRepresentation(CFStringRef string);
    // CFStringRef CFStringCreateWithFileSystemRepresentation(CFAllocatorRef alloc, const char *buffer);
    // 
    // CFComparisonResult CFStringCompareWithOptionsAndLocale(CFStringRef theString1, CFStringRef theString2, CFRange rangeToCompare, CFStringCompareFlags compareOptions, CFLocaleRef locale) API_AVAILABLE(macos(10.5), ios(2.0), watchos(2.0), tvos(9.0));
    // CFComparisonResult CFStringCompareWithOptions(CFStringRef theString1, CFStringRef theString2, CFRange rangeToCompare, CFStringCompareFlags compareOptions);
    // CFComparisonResult CFStringCompare(CFStringRef theString1, CFStringRef theString2, CFStringCompareFlags compareOptions);
    // Boolean CFStringFindWithOptionsAndLocale(CFStringRef theString, CFStringRef stringToFind, CFRange rangeToSearch, CFStringCompareFlags searchOptions, CFLocaleRef locale, CFRange *result) API_AVAILABLE(macos(10.5), ios(2.0), watchos(2.0), tvos(9.0));
    // Boolean CFStringFindWithOptions(CFStringRef theString, CFStringRef stringToFind, CFRange rangeToSearch, CFStringCompareFlags searchOptions, CFRange *result);
    // CFArrayRef CFStringCreateArrayWithFindResults(CFAllocatorRef alloc, CFStringRef theString, CFStringRef stringToFind, CFRange rangeToSearch, CFStringCompareFlags compareOptions);
    // CFRange CFStringFind(CFStringRef theString, CFStringRef stringToFind, CFStringCompareFlags compareOptions);
    // Boolean CFStringHasPrefix(CFStringRef theString, CFStringRef prefix);
    // Boolean CFStringHasSuffix(CFStringRef theString, CFStringRef suffix);
    // CFRange CFStringGetRangeOfComposedCharactersAtIndex(CFStringRef theString, CFIndex theIndex);
    // Boolean CFStringFindCharacterFromSet(CFStringRef theString, CFCharacterSetRef theSet, CFRange rangeToSearch, CFStringCompareFlags searchOptions, CFRange *result);
    // void CFStringGetLineBounds(CFStringRef theString, CFRange range, CFIndex *lineBeginIndex, CFIndex *lineEndIndex, CFIndex *contentsEndIndex); 
    // void CFStringGetParagraphBounds(CFStringRef string, CFRange range, CFIndex *parBeginIndex, CFIndex *parEndIndex, CFIndex *contentsEndIndex) API_AVAILABLE(macos(10.5), ios(2.0), watchos(2.0), tvos(9.0));
    // CFIndex CFStringGetHyphenationLocationBeforeIndex(CFStringRef string, CFIndex location, CFRange limitRange, CFOptionFlags options, CFLocaleRef locale, UTF32Char *character) API_AVAILABLE(macos(10.7), ios(4.2), watchos(2.0), tvos(9.0));
    // Boolean CFStringIsHyphenationAvailableForLocale(CFLocaleRef locale) API_AVAILABLE(macos(10.7), ios(4.3), watchos(2.0), tvos(9.0));
    // 
    // CFStringRef CFStringCreateByCombiningStrings(CFAllocatorRef alloc, CFArrayRef theArray, CFStringRef separatorString);	/* Empty array returns empty string; one element array returns the element */
    // CFArrayRef CFStringCreateArrayBySeparatingStrings(CFAllocatorRef alloc, CFStringRef theString, CFStringRef separatorString);	/* No separators in the string returns array with that string; string == sep returns two empty strings */
    // 
    // SInt32 CFStringGetIntValue(CFStringRef str);
    // double CFStringGetDoubleValue(CFStringRef str);
    // 
    // void CFStringAppend(CFMutableStringRef theString, CFStringRef appendedString);
    // void CFStringAppendCharacters(CFMutableStringRef theString, const UniChar *chars, CFIndex numChars);
    // void CFStringAppendPascalString(CFMutableStringRef theString, ConstStr255Param pStr, CFStringEncoding encoding);
    // void CFStringAppendCString(CFMutableStringRef theString, const char *cStr, CFStringEncoding encoding);
    // void CFStringAppendFormat(CFMutableStringRef theString, CFDictionaryRef formatOptions, CFStringRef format, ...) CF_FORMAT_FUNCTION(3,4);
    // void CFStringAppendFormatAndArguments(CFMutableStringRef theString, CFDictionaryRef formatOptions, CFStringRef format, va_list arguments) CF_FORMAT_FUNCTION(3,0);
    // void CFStringInsert(CFMutableStringRef str, CFIndex idx, CFStringRef insertedStr);
    // void CFStringDelete(CFMutableStringRef theString, CFRange range);
    // void CFStringReplace(CFMutableStringRef theString, CFRange range, CFStringRef replacement);
    // void CFStringReplaceAll(CFMutableStringRef theString, CFStringRef replacement);	/* Replaces whole string */
    // CFIndex CFStringFindAndReplace(CFMutableStringRef theString, CFStringRef stringToFind, CFStringRef replacementString, CFRange rangeToSearch, CFStringCompareFlags compareOptions);
    // void CFStringSetExternalCharactersNoCopy(CFMutableStringRef theString, UniChar *chars, CFIndex length, CFIndex capacity);	/* Works only on specially created mutable strings! */
    // void CFStringPad(CFMutableStringRef theString, CFStringRef padString, CFIndex length, CFIndex indexIntoPad);
    // void CFStringTrim(CFMutableStringRef theString, CFStringRef trimString);
    // void CFStringTrimWhitespace(CFMutableStringRef theString);
    // void CFStringLowercase(CFMutableStringRef theString, CFLocaleRef locale);
    // void CFStringUppercase(CFMutableStringRef theString, CFLocaleRef locale);
    // void CFStringCapitalize(CFMutableStringRef theString, CFLocaleRef locale);
    // 
    // void CFStringNormalize(CFMutableStringRef theString, CFStringNormalizationForm theForm);
    // void CFStringFold(CFMutableStringRef theString, CFStringCompareFlags theFlags, CFLocaleRef theLocale) API_AVAILABLE(macos(10.5), ios(2.0), watchos(2.0), tvos(9.0));
    // Boolean CFStringTransform(CFMutableStringRef string, CFRange *range, CFStringRef transform, Boolean reverse);
    // 
    // const CFStringRef kCFStringTransformStripCombiningMarks;
    // const CFStringRef kCFStringTransformToLatin;
    // const CFStringRef kCFStringTransformFullwidthHalfwidth;
    // const CFStringRef kCFStringTransformLatinKatakana;
    // const CFStringRef kCFStringTransformLatinHiragana;
    // const CFStringRef kCFStringTransformHiraganaKatakana;
    // const CFStringRef kCFStringTransformMandarinLatin;
    // const CFStringRef kCFStringTransformLatinHangul;
    // const CFStringRef kCFStringTransformLatinArabic;
    // const CFStringRef kCFStringTransformLatinHebrew;
    // const CFStringRef kCFStringTransformLatinThai;
    // const CFStringRef kCFStringTransformLatinCyrillic;
    // const CFStringRef kCFStringTransformLatinGreek;
    // const CFStringRef kCFStringTransformToXMLHex;
    // const CFStringRef kCFStringTransformToUnicodeName;
    // const CFStringRef kCFStringTransformStripDiacritics;
    // 
    // Boolean CFStringIsEncodingAvailable(CFStringEncoding encoding);
    // const CFStringEncoding *CFStringGetListOfAvailableEncodings(void);
    // CFStringRef CFStringGetNameOfEncoding(CFStringEncoding encoding);
    // unsigned long CFStringConvertEncodingToNSStringEncoding(CFStringEncoding encoding);
    // CFStringEncoding CFStringConvertNSStringEncodingToEncoding(unsigned long encoding);
    // UInt32 CFStringConvertEncodingToWindowsCodepage(CFStringEncoding encoding);
    // CFStringEncoding CFStringConvertWindowsCodepageToEncoding(UInt32 codepage);
    // CFStringEncoding CFStringConvertIANACharSetNameToEncoding(CFStringRef theString);
    // CFStringRef  CFStringConvertEncodingToIANACharSetName(CFStringEncoding encoding);
    // CFStringEncoding CFStringGetMostCompatibleMacStringEncoding(CFStringEncoding encoding);

    fn CFShowStr(string: CFStringRef);
  }
}

#[repr(transparent)] pub struct CFStringRef(usize);
#[repr(transparent)] pub struct CFMutableStringRef(usize);

unsafe impl Subtype<CFTypeRef> for CFStringRef {
  unsafe fn upcast(&self) -> CFTypeRef {
    return CFTypeRef(self.0);
  }
}

unsafe impl Subtype<CFStringRef> for CFStringRef {
  unsafe fn upcast(&self) -> CFStringRef {
    return CFStringRef(self.0);
  }
}

unsafe impl Subtype<CFTypeRef> for CFMutableStringRef {
  unsafe fn upcast(&self) -> CFTypeRef {
    return CFTypeRef(self.0);
  }
}

unsafe impl Subtype<CFStringRef> for CFMutableStringRef {
  unsafe fn upcast(&self) -> CFStringRef {
    return CFStringRef(self.0);
  }
}

unsafe impl Subtype<CFMutableStringRef> for CFMutableStringRef {
  unsafe fn upcast(&self) -> CFMutableStringRef {
    return CFMutableStringRef(self.0);
  }
}
