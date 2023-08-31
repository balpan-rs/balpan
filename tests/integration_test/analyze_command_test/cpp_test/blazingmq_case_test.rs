use crate::integration_test::assert_analyzed_source_code;
use indoc::indoc;

#[test]
fn test_function_definition_with_nested_scope() {
    let result = indoc! { r#"
    /// [TODO] BloombergLP
    namespace BloombergLP {
        /// [TODO] BloombergLP > bmqimp
        namespace bmqimp {
            /// [TODO] BloombergLP > bmqimp > anonymous
            namespace {
                // CONSTANTS
                const double             k_RECONNECT_INTERVAL_MS = 500;
                const int                k_RECONNECT_COUNT = bsl::numeric_limits<int>::max();
                const bsls::Types::Int64 k_CHANNEL_LOW_WATERMARK = 512 * 1024;

                /// Create the StatContextConfiguration to use, from the specified
                /// `options`, and using the specified `allocator` for memory allocations.
                /// [TODO] BloombergLP > bmqimp > anonymous > statContextConfiguration
                mwcst::StatContextConfiguration
                statContextConfiguration(const bmqt::SessionOptions& options,
                                         bslma::Allocator*           allocator)
                {
                    mwcst::StatContextConfiguration config("stats", allocator);
                    if (options.statsDumpInterval() != bsls::TimeInterval()) {
                        // Stats configuration:
                        //   we snapshot every second
                        //   first level keeps 30s of history
                        //   second level keeps enough for the dump interval
                        // Because some stats require range computation, second level actually
                        // has to be of size 1 more than the dump interval
                        config.defaultHistorySize(
                            30,
                            (options.statsDumpInterval().seconds() / 30) + 1);
                    }
                    else {
                        config.defaultHistorySize(2);
                    }

                    return config;
                }
            }
        }
    }"#};

    let source_code = indoc! { r#"
    namespace BloombergLP {
        namespace bmqimp {
            namespace {
                // CONSTANTS
                const double             k_RECONNECT_INTERVAL_MS = 500;
                const int                k_RECONNECT_COUNT = bsl::numeric_limits<int>::max();
                const bsls::Types::Int64 k_CHANNEL_LOW_WATERMARK = 512 * 1024;

                /// Create the StatContextConfiguration to use, from the specified
                /// `options`, and using the specified `allocator` for memory allocations.
                mwcst::StatContextConfiguration
                statContextConfiguration(const bmqt::SessionOptions& options,
                                         bslma::Allocator*           allocator)
                {
                    mwcst::StatContextConfiguration config("stats", allocator);
                    if (options.statsDumpInterval() != bsls::TimeInterval()) {
                        // Stats configuration:
                        //   we snapshot every second
                        //   first level keeps 30s of history
                        //   second level keeps enough for the dump interval
                        // Because some stats require range computation, second level actually
                        // has to be of size 1 more than the dump interval
                        config.defaultHistorySize(
                            30,
                            (options.statsDumpInterval().seconds() / 30) + 1);
                    }
                    else {
                        config.defaultHistorySize(2);
                    }

                    return config;
                }
            }
        }
    }"#};

    assert_analyzed_source_code(source_code, result, "cpp");
}

#[test]
fn test_class_declaration_with_nested_scope() {
    let source_code = indoc! { r#"
    namespace m_bmqbrkr {
        class Task_AllocatorManager {
          private:
            mqbcfg::AllocatorType::Value d_type;

            bsls::ObjectBuffer<mwcma::CountingAllocatorStore> d_store;
          private:
            Task_AllocatorManager(const Task_AllocatorManager&);  // = delete;
          public:
            explicit Task_AllocatorManager(mqbcfg::AllocatorType::Value type);

            ~Task_AllocatorManager();
        };
    }"#};

    let result = indoc! { r#"
    /// [TODO] m_bmqbrkr
    namespace m_bmqbrkr {
        /// [TODO] m_bmqbrkr > Task_AllocatorManager
        class Task_AllocatorManager {
          private:
            mqbcfg::AllocatorType::Value d_type;

            bsls::ObjectBuffer<mwcma::CountingAllocatorStore> d_store;
          private:
            Task_AllocatorManager(const Task_AllocatorManager&);  // = delete;
          public:
            explicit Task_AllocatorManager(mqbcfg::AllocatorType::Value type);

            ~Task_AllocatorManager();
        };
    }"#};

    assert_analyzed_source_code(source_code, result, "cpp");
}

#[ignore]
fn test_templated_function_definition() {
    let source_code = indoc! { r#"
    template <typename CMD>
    bool parseCommand(CMD* command, const bsl::string& jsonInput)
    {
        bsl::istringstream     is(jsonInput);
        baljsn::DecoderOptions options;
        options.setSkipUnknownElements(true);
        baljsn::Decoder decoder;
        int             rc = decoder.decode(is, command, options);
        if (rc != 0) {
            BALL_LOG_ERROR << "Unable to decode: " << jsonInput << bsl::endl
                           << decoder.loggedMessages();
            return false;  // RETURN
        }

        return true;
    }

    template <typename TYPE>
    inline bool Value::is() const
    {
        return d_value.is<TYPE>();
    }

    template <typename TYPE>
    inline const TYPE& Value::the() const
    {
        return d_value.the<TYPE>();
    }

    template <class VISITOR>
    inline typename VISITOR::ResultType Value::apply(const VISITOR& visitor) const
    {
        return d_value.apply(visitor);
    }
    "#};
    
    let result = indoc! { r#"
    /// [TODO] parseCommand
    template <typename CMD>
    bool parseCommand(CMD* command, const bsl::string& jsonInput)
    {
        bsl::istringstream     is(jsonInput);
        baljsn::DecoderOptions options;
        options.setSkipUnknownElements(true);
        baljsn::Decoder decoder;
        int             rc = decoder.decode(is, command, options);
        if (rc != 0) {
            BALL_LOG_ERROR << "Unable to decode: " << jsonInput << bsl::endl
                           << decoder.loggedMessages();
            return false;  // RETURN
        }

        return true;
    }

    /// [TODO] Value::is
    template <typename TYPE>
    inline bool Value::is() const
    {
        return d_value.is<TYPE>();
    }

    /// [TODO] Value::the
    template <typename TYPE>
    inline const TYPE& Value::the() const
    {
        return d_value.the<TYPE>();
    }

    /// [TODO] Value::apply
    template <class VISITOR>
    inline typename VISITOR::ResultType Value::apply(const VISITOR& visitor) const
    {
        return d_value.apply(visitor);
    }"#};

    assert_analyzed_source_code(source_code, result, "cpp");
}
