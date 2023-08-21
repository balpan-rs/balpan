#[cfg(test)]
mod pattern_search_test {
    #[test]
    fn test_search_multiple_pattern() {
        use balpan::pattern_search::PatternTree;

        let searcher = PatternTree::new();

        let text = "ABAAABCDABCDABABCD";
        let patterns = vec!["ABCD".to_string(), "BCD".to_string()];

        let expected = (true, vec![4, 8, 14]);
        assert_eq!(searcher.aho_corasick_search(text, &patterns), expected);
    }

    #[test]
    fn test_search_todo_done_comments_using_aho_corasick() {
        use balpan::pattern_search::PatternTree;

        let searcher = PatternTree::new();
        let text = r#"
        //[TODO] ABC
        //some comment
        //struct ABC {
        //    field: i32,
        //    field2: i32,
        //}
        //
        //[DONE] DEF
        //some comment about DEF
        //fn DEF() {
        //   unimplemented!();
        //}
        //"#;

        let patterns = vec!["[TODO]".to_string(), "[DONE]".to_string()];
        let expected = (true, vec![13, 170]);

        let result = searcher.aho_corasick_search(text, &patterns);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_selective_search() {
        use balpan::pattern_search::PatternTree;

        let searcher = PatternTree::new();
        let text = r#"
        //[TODO] ABC
        //some comment
        //struct ABC {
        //    field: i32,
        //    field2: i32,
        //}
        //
        //[TODO] DEF
        //some comment about DEF
        //fn DEF() {
        //   unimplemented!();
        //}
        //"#;

        // let patterns = vec!["[TODO]".to_string(), "[DONE]".to_string()];
        // let expected = (true, vec![11, 154]);

        // let result = searcher.selective_search(&patterns, text);

        // assert_eq!(result, expected);

        let pattern = vec!["[TODO]".to_string()];
        let expected = (true, vec![11, 154]);

        let result = searcher.selective_search(&pattern, text);

        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod boyer_moore_tests {
    use balpan::commands::boyer_moore::SearchIn;

    #[test]
    fn test_find_pending_character_index() {
        use balpan::commands::boyer_moore::find_pending_character_index;

        let chars = vec!['A', 'B', 'C', 'B', 'D'];
        let start = 1;
        let pattern = &'B';

        let result = find_pending_character_index(&chars, start, pattern);

        assert_eq!(2, result);
    }

    #[test]
    fn test_suffix_table() {
        use balpan::commands::boyer_moore::get_suffix_table;

        let text = "GCAGAGAG".as_bytes();

        let table = get_suffix_table(&text);
        let expected = vec![1, 0, 0, 2, 0, 4, 0, 0];

        assert_eq!(table, expected);

        let text = "abcbabcabab".as_bytes();

        let table = get_suffix_table(&text);
        let expected = vec![0, 2, 0, 1, 0, 3, 0, 0, 2, 0, 0];

        assert_eq!(table, expected);
    }

    #[test]
    fn test_build_suffix_table() {
        use balpan::commands::boyer_moore::build_suffixes_table;

        let pattern = "GCAGAGAG".as_bytes();

        let table = build_suffixes_table(&pattern);
        let expected = vec![7, 7, 7, 2, 7, 4, 7, 1];

        assert_eq!(table, expected);

        let pattern = "abcbabcabab".as_bytes();

        let table = build_suffixes_table(&pattern);
        let expected = vec![10, 10, 10, 10, 10, 10, 10, 5, 2, 7, 1];

        assert_eq!(table, expected);
    }

    #[test]
    fn test_find_first_occurrence() {
        use balpan::commands::boyer_moore::BoyerMooreSearch;

        let searcher = BoyerMooreSearch::new(b"abc");
        let text = "abababc";

        assert_eq!(Some(4), searcher.find_first_position(text.as_bytes()));
    }

    #[test]
    fn test_overlapping() {
        use balpan::commands::boyer_moore::BoyerMooreSearch;

        let searcher = BoyerMooreSearch::new(b"aaba");
        let text = b"aabaabaaba";
        let result = searcher.find_overlapping_in(text).collect::<Vec<usize>>();

        assert_eq!(vec![0, 3, 6], result);
    }

    #[test]
    fn test_no_pattern_found() {
        use balpan::commands::boyer_moore::BoyerMooreSearch;

        let searcher = BoyerMooreSearch::new(b"abc");
        let text = "ababab";

        assert_eq!(None, searcher.find_first_position(text.as_bytes()));
    }

    #[test]
    fn test_find_patterns_in_source_code() {
        use balpan::commands::boyer_moore::BoyerMooreSearch;

        let source = r#"
        //[TODO] main
        //comment for main
        fn main() {
            println!("Hello, world!");
        }

        pub trait Foo<'a, T> {
            fn foo(&'a self) -> None;
            fn foo2(&'a self) -> bool;
        }

        impl <'a, T> Foo<'a, T> for Foo {
            fn foo(&'a self) -> None {
                None
            }

            fn foo2(&'a self) -> bool {
                true
            }
        }
        "#.as_bytes();

        let searcher = BoyerMooreSearch::new(b"fn");
        let result = searcher.find_in(source).collect::<Vec<usize>>();

        assert_eq!(vec![62, 167, 205, 297, 372], result);
    }

    #[test]
    fn test_search_word() {
        use balpan::commands::boyer_moore::BoyerMooreSearch;

        let text = "
        MALCOM.
        'Tis call'd the evil:
        A most miraculous work in this good king;
        Which often, since my here-remain in England,
        I have seen him do. How he solicits heaven,
        Himself best knows, but strangely-visited people,
        All swoln and ulcerous, pitiful to the eye,
        The mere despair of surgery, he cures;
        Hanging a golden stamp about their necks,
        Put on with holy prayers: and 'tis spoken,
        To the succeeding royalty he leaves
        The healing benediction. With this strange virtue,
        He hath a heavenly gift of prophecy;
        And sundry blessings hang about his throne,
        That speak him full of grace.
        
        MACDUFF.
        See, who comes here?
        
        MALCOLM.
        My countryman; but yet I know him not.
        
        MACDUFF.
        My ever-gentle cousin, welcome hither.
        
        MALCOLM.
        I know him now. Good God, betimes remove
        The means that makes us strangers!
        
        ROSS.
        Sir, amen.
        
        MACDUFF.
        Stands Scotland where it did?
        
        ROSS.
        Alas, poor country,
        Almost afraid to know itself! It cannot
        Be call'd our mother, but our grave, where nothing,
        But who knows nothing, is once seen to smile;
        Where sighs, and groans, and shrieks, that rent the air,
        Are made, not mark'd; where violent sorrow seems
        A modern ecstasy. The dead man's knell
        Is there scarce ask'd for who; and good men's lives
        Expire before the flowers in their caps,
        Dying or ere they sicken. 
        
        MACDUFF.
        O, relation
        Too nice, and yet too true!
        
        MALCOLM.
        What’s the newest grief?
        
        ROSS.
        That of an hour’s age doth hiss the speaker;
        Each minute teems a new one.
        
        MACDUFF.
        How does my wife?
        
        ROSS.
        Why, well.
        
        MACDUFF.
        And all my children?
        
        ROSS.
        Well too.
        
        MACDUFF.
        The tyrant has not batter’d at their peace?
        
        ROSS.
        No; they were well at peace when I did leave ’em.
        
        MACDUFF.
        Be not a niggard of your speech: how goes’t?
        
        ROSS.
        When I came hither to transport the tidings,
        Which I have heavily borne, there ran a rumour
        Of many worthy fellows that were out;
        Which was to my belief witness’d the rather,
        For that I saw the tyrant’s power afoot.
        Now is the time of help. Your eye in Scotland
        Would create soldiers, make our women fight,
        To doff their dire distresses.
        
        MALCOLM.
        Be’t their comfort
        We are coming thither. Gracious England hath
        Lent us good Siward and ten thousand men;
        An older and a better soldier none
        That Christendom gives out.
        
        ROSS.
        Would I could answer
        This comfort with the like! But I have words
        That would be howl’d out in the desert air,
        Where hearing should not latch them.
        
        MACDUFF.
        What concern they?
        The general cause? or is it a fee-grief
        Due to some single breast?
        
        ROSS.
        No mind that’s honest
        But in it shares some woe, though the main part
        Pertains to you alone.
        
        MACDUFF.
        If it be mine,
        Keep it not from me, quickly let me have it.
        
        ROSS.
        Let not your ears despise my tongue for ever,
        Which shall possess them with the heaviest sound
        That ever yet they heard.
        
        MACDUFF.
        Humh! I guess at it.
        
        ROSS.
        Your castle is surpris’d; your wife and babes
        Savagely slaughter’d. To relate the manner
        Were, on the quarry of these murder’d deer,
        To add the death of you.
        
        MALCOLM.
        Merciful heaven!—
        What, man! ne’er pull your hat upon your brows.
        Give sorrow words. The grief that does not speak
        Whispers the o’er-fraught heart, and bids it break.
        
        MACDUFF.
        My children too?
        
        ROSS.
        Wife, children, servants, all
        That could be found.
        
        MACDUFF.
        And I must be from thence!
        My wife kill’d too?
        
        ROSS.
        I have said.".as_bytes();

        let searcher = BoyerMooreSearch::new(b"MALCOM");
        let first_occurrence = searcher.find_first_position(text);
        assert_eq!(Some(9), first_occurrence);

        let searcher = BoyerMooreSearch::new(b"MACDUFF");
        let find_all = searcher.find_in(text).collect::<Vec<usize>>();
        let expected = vec![
            716, 844, 1077, 1667, 
            1925, 2019, 2115, 2278, 
            3229, 3507, 3777, 4282, 4423
        ];
        assert_eq!(expected, find_all);
    }

    #[test]
    fn test_is_work_for_non_alphabet() {
        use balpan::commands::boyer_moore::BoyerMooreSearch;

        let pattern = "🦀🦀🐪🔥🐍✅".as_bytes();
        let searcher = BoyerMooreSearch::new(pattern);
        let text = "🦀🦀🐪🔥🐍✅🦀🦀🐪🔥🐍✅🦀🦀🐪🔥🐍✅🦀🦀🐪🔥🐍✅🦀🦀🐪🔥🐍✅";

        let result = searcher.find_in(text.as_bytes()).collect::<Vec<usize>>();
        assert_eq!(vec![0, 23, 46, 69, 92], result);
    }
}