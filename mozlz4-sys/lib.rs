use std::os::raw::{c_char, c_int};

extern "C" {
    /*  LZ4_compress_default() :
		Compresses 'srcSize' bytes from buffer 'src'
		into already allocated 'dst' buffer of size 'dstCapacity'.
		Compression is guaranteed to succeed if 'dstCapacity' >= LZ4_compressBound(srcSize).
		It also runs faster, so it's a recommended setting.
		If the function cannot compress 'src' into a limited 'dst' budget,
		compression stops *immediately*, and the function result is zero.
		As a consequence, 'dst' content is not valid.
		This function never writes outside 'dst' buffer, nor read outside 'source' buffer.
			srcSize : supported max value is LZ4_MAX_INPUT_VALUE
			dstCapacity : full or partial size of buffer 'dst' (which must be already allocated)
			return  : the number of bytes written into buffer 'dst' (necessarily <= dstCapacity)
					  or 0 if compression fails
	
	LZ4LIB_API int LZ4_compress_default(const char* src, char* dst, int srcSize, int dstCapacity);
	*/
    pub fn LZ4_compress_default(
        src: *const c_char,
        dst: *mut c_char,
        srcSize: c_int,
        dstCapacity: c_int,
    ) -> c_int;

    /*  LZ4_decompress_safe() :
		compressedSize : is the exact complete size of the compressed block.
		dstCapacity : is the size of destination buffer, which must be already allocated.
		return : the number of bytes decompressed into destination buffer (necessarily <= dstCapacity)
				 If destination buffer is not large enough, decoding will stop and output an error code (negative value).
				 If the source stream is detected malformed, the function will stop decoding and return a negative result.
				 This function is protected against buffer overflow exploits, including malicious data packets.
				 It never writes outside output buffer, nor reads outside input buffer.
    
    LZ4LIB_API int LZ4_decompress_safe (const char* src, char* dst, int compressedSize, int dstCapacity);	
	*/
    pub fn LZ4_decompress_safe(
        src: *const c_char,
        dst: *mut c_char,
        compressedSize: c_int,
        dstCapacity: c_int,
    ) -> c_int;

    /*
	LZ4_compressBound() :
		Provides the maximum size that LZ4 compression may output in a "worst case" scenario (input data not compressible)
		This function is primarily useful for memory allocation purposes (destination buffer size).
		Macro LZ4_COMPRESSBOUND() is also provided for compilation-time evaluation (stack memory allocation for example).
		Note that LZ4_compress_default() compress faster when dest buffer size is >= LZ4_compressBound(srcSize)
			inputSize  : max supported value is LZ4_MAX_INPUT_SIZE
			return : maximum output size in a "worst case" scenario
				  or 0, if input size is too large ( > LZ4_MAX_INPUT_SIZE)
				  
	LZ4LIB_API int LZ4_compressBound(int inputSize);
	*/
    pub fn LZ4_compressBound(inputSize: c_int) -> c_int;
}
