#[cfg(test)]
mod mastodon_case_test {
    use crate::integration_test::assert_analyzed_source_code;
    use indoc::indoc;

    #[test]
    fn test_class_declaration_with_nested_scope() {
        let source_code = indoc! { r#"
        module Chewy
          class Strategy
            class Mastodon < Base
              def initialize
                super

                @stash = Hash.new { |hash, key| hash[key] = [] }
              end

              def update(type, objects, _options = {})
                @stash[type].concat(type.root.id ? Array.wrap(objects) : type.adapter.identify(objects)) if Chewy.enabled?
              end

              def leave
                RedisConfiguration.with do |redis|
                  redis.pipelined do |pipeline|
                    @stash.each do |type, ids|
                      pipeline.sadd("chewy:queue:#{type.name}", ids)
                    end
                  end
                end
              end
            end
          end
        end"#};

        let result = indoc! { r#"
        # [TODO] Chewy
        module Chewy
          # [TODO] Chewy > Strategy
          class Strategy
            # [TODO] Chewy > Strategy > Mastodon
            class Mastodon < Base
              # [TODO] Chewy > Strategy > Mastodon > initialize
              def initialize
                super

                @stash = Hash.new { |hash, key| hash[key] = [] }
              end

              # [TODO] Chewy > Strategy > Mastodon > update
              def update(type, objects, _options = {})
                @stash[type].concat(type.root.id ? Array.wrap(objects) : type.adapter.identify(objects)) if Chewy.enabled?
              end

              # [TODO] Chewy > Strategy > Mastodon > leave
              def leave
                RedisConfiguration.with do |redis|
                  redis.pipelined do |pipeline|
                    @stash.each do |type, ids|
                      pipeline.sadd("chewy:queue:#{type.name}", ids)
                    end
                  end
                end
              end
            end
          end
        end"#};

        assert_analyzed_source_code(source_code, result, "ruby");
    }
}
