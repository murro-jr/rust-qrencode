#include <iostream>
#include <qrencode.h>

extern "C" {
    void* encode_string(const char* string, int version, int level, int hint, int casesensitive) {
        return QRcode_encodeString (string,
                                    version,
                                    static_cast<QRecLevel>(level),
                                    static_cast<QRencodeMode>(hint),
                                    casesensitive);
    }

    void* encode_string_8bit(const char* string, int version, int level) {
        return QRcode_encodeString8bit(string, version, static_cast<QRecLevel>(level));
    }

    void* encode_string_mqr(const char* string, int version, int level, int hint, int casesensitive) {
        return QRcode_encodeStringMQR (string,
                                    version,
                                    static_cast<QRecLevel>(level),
                                    static_cast<QRencodeMode>(hint),
                                    casesensitive);
    }

    void* encode_string_8bit_mqr(const char* string, int version, int level) {
        return QRcode_encodeString8bitMQR(string, version, static_cast<QRecLevel>(level));
    }

    void* encode_data(int size, const unsigned char* data, int version, int level) {
        return QRcode_encodeData(size, data, version, static_cast<QRecLevel>(level));
    }

    void* encode_data_mqr(int size, const unsigned char* data, int version, int level) {
        return QRcode_encodeDataMQR(size, data, version, static_cast<QRecLevel>(level));
    }

    void free(void* qrcode) {
        return QRcode_free(static_cast<QRcode*>(qrcode));
    }
}