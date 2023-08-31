use crate::integration_test::assert_analyzed_source_code;
use indoc::indoc;

#[test]
fn test_function_definition_with_nginx_convention() {
    let source_code = indoc! { r#"
    static int
    ngx_stream_ssl_alpn_select(ngx_ssl_conn_t *ssl_conn, const unsigned char **out,
        unsigned char *outlen, const unsigned char *in, unsigned int inlen,
        void *arg)
    {
        ngx_str_t         *alpn;
    #if (NGX_DEBUG)
        unsigned int       i;
        ngx_connection_t  *c;

        c = ngx_ssl_get_connection(ssl_conn);

        for (i = 0; i < inlen; i += in[i] + 1) {
            ngx_log_debug2(NGX_LOG_DEBUG_STREAM, c->log, 0,
                           "SSL ALPN supported by client: %*s",
                           (size_t) in[i], &in[i + 1]);
        }

    #endif

        alpn = arg;

        if (SSL_select_next_proto((unsigned char **) out, outlen, alpn->data,
                                  alpn->len, in, inlen)
            != OPENSSL_NPN_NEGOTIATED)
        {
            return SSL_TLSEXT_ERR_ALERT_FATAL;
        }

        ngx_log_debug2(NGX_LOG_DEBUG_STREAM, c->log, 0,
                       "SSL ALPN selected: %*s", (size_t) *outlen, *out);

        return SSL_TLSEXT_ERR_OK;
    }"#};

    let result = indoc! { r#"
    /// [TODO] ngx_stream_ssl_alpn_select
    static int
    ngx_stream_ssl_alpn_select(ngx_ssl_conn_t *ssl_conn, const unsigned char **out,
        unsigned char *outlen, const unsigned char *in, unsigned int inlen,
        void *arg)
    {
        ngx_str_t         *alpn;
    #if (NGX_DEBUG)
        unsigned int       i;
        ngx_connection_t  *c;

        c = ngx_ssl_get_connection(ssl_conn);

        for (i = 0; i < inlen; i += in[i] + 1) {
            ngx_log_debug2(NGX_LOG_DEBUG_STREAM, c->log, 0,
                           "SSL ALPN supported by client: %*s",
                           (size_t) in[i], &in[i + 1]);
        }

    #endif

        alpn = arg;

        if (SSL_select_next_proto((unsigned char **) out, outlen, alpn->data,
                                  alpn->len, in, inlen)
            != OPENSSL_NPN_NEGOTIATED)
        {
            return SSL_TLSEXT_ERR_ALERT_FATAL;
        }

        ngx_log_debug2(NGX_LOG_DEBUG_STREAM, c->log, 0,
                       "SSL ALPN selected: %*s", (size_t) *outlen, *out);

        return SSL_TLSEXT_ERR_OK;
    }"#};

    assert_analyzed_source_code(source_code, result, "cpp");
}
