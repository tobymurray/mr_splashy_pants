pub mod api;
pub mod pants;

#[cfg(test)]
mod tests {
    use std::env;

    use crate::api::generated::request::links_and_comments;
    use crate::api::generated::request::listings as listings_request;
    use crate::pants::Pants;

    const USER_AGENT: &str = "Microsoft Windows 10 Home:ca.technicallyrural.testapp:0.0.1 (by /u/ample_bird)";
    const SUBREDDIT: &str = "testingground4bots";

    #[allow(clippy::dead_code)]
    fn setup_logger() -> Result<(), fern::InitError> {
        fern::Dispatch::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    record.level(),
                    message
                ))
            })
            .level(log::LevelFilter::Trace)
            .chain(std::io::stdout())
            .chain(fern::log_file("output.log")?)
            .apply()?;
        Ok(())
    }

    fn build_pants() -> Pants {
        dotenv::dotenv().ok();

        Pants::new(
            USER_AGENT,
            env::var("ACCESS_TOKEN").unwrap(),
            &env::var("REFRESH_TOKEN").unwrap(),
            &env::var("CLIENT_ID").unwrap(),
            &env::var("CLIENT_SECRET").unwrap(),
        )
    }

    // Accounts
    #[test]
    fn me() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.me()) {
            Ok(response) => println!("Successfully got response on first invocation: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn me_karma() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.me_karma()) {
            Ok(response) => println!("Response to me_karma is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn me_prefs() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.me_prefs()) {
            Ok(response) => println!("Response to me_prefs is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn me_trophies() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.me_trophies()) {
            Ok(response) => println!("Response to me_trophies is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn prefs_friends() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.prefs_friends()) {
            Ok(response) => println!("Response to prefs_friends is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn prefs_blocked() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.prefs_blocked()) {
            Ok(response) => println!("Response to prefs_blocked is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn prefs_messaging() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.prefs_messaging()) {
            Ok(response) => println!("Response to prefs_messaging is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn prefs_trusted() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.prefs_trusted()) {
            Ok(response) => println!("Response to prefs_trusted is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn me_friends() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.me_friends()) {
            Ok(response) => println!("Response to me_friends is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn me_blocked() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.me_blocked()) {
            Ok(response) => println!("Response to me_blocked is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    // Listings

    #[test]
    fn trending_subreddits() {
        dotenv::dotenv().ok();

        let mut pants = build_pants();

        match tokio_test::block_on(pants.trending_subreddits()) {
            Ok(response) => println!("Response to trending_subreddits is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn best() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.front_page().best()) {
            Ok(response) => println!("Response to best is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    // fn by_id_names() {
    //     let mut pants = build_pants();
    //     // TODO: Figure this out
    //     match tokio_test::block_on(pants.by_id_names(fullnames: Vec<String>)) {
    //         Ok(response) => println!("Response to best is: {:#?}", response),
    //         Err(e) => panic!("An error ocurred: {}", e),
    //     };
    // }

    // fn comments_article() {
    //     let mut pants = build_pants();
    //     // TODO: Figure this out
    //     match tokio_test::block_on(pants.get_comments_article(article: String)() {
    //         Ok(response) => println!("Response to best is: {:#?}", response),
    //         Err(e) => panic!("An error ocurred: {}", e),
    //     };
    // }

    #[test]
    fn subreddit_comments_article() {
        let mut pants = build_pants();

        let query_parameters = listings_request::RSubredditCommentsArticle {
            comment: "".to_string(),
            showedits: "".to_string(),
            sr_detail: "".to_string(),
            threaded: "".to_string(),
            truncate: "".to_string(),
            depth: "".to_string(),
            sort: "".to_string(),
            article: "".to_string(),
            context: "".to_string(),
            showmore: "".to_string(),
            limit: "".to_string(),
        };

        match tokio_test::block_on(pants.subreddit(SUBREDDIT).comments(&crate::pants::utils::Fullname {
            value: "ins0kg".to_string(),
        })) {
            Ok(response) => println!(
                "Response to subreddit_comments_article is: {}",
                serde_json::to_string_pretty(&response).unwrap()
            ),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    // fn duplicates_article() {
    //     let mut pants = build_pants();
    //     // TODO: Figure this out
    //     match tokio_test::block_on(pants.duplicates_article(article: String)) {
    //         Ok(response) => println!("Response to best is: {:#?}", response),
    //         Err(e) => panic!("An error ocurred: {}", e),
    //     };
    // }

    #[test]
    fn hot() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.front_page().hot()) {
            Ok(response) => println!("Response to hot is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn subreddit_hot() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.subreddit(SUBREDDIT).hot()) {
            Ok(response) => println!("Response to hot is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn get_new() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.front_page().new()) {
            Ok(response) => println!("Response to get_new is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn subreddit_new() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.subreddit(SUBREDDIT).new()) {
            Ok(response) => println!(
                "Response to new is: {}",
                serde_json::to_string_pretty(&response).unwrap()
            ),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn random() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.front_page().random()) {
            Ok(response) => println!("Response to random is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn subreddit_random() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.subreddit(SUBREDDIT).random()) {
            Ok(response) => println!("Response to random is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn rising() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.front_page().rising()) {
            Ok(response) => println!("Response to rising is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn subreddit_rising() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.subreddit(SUBREDDIT).rising()) {
            Ok(response) => println!("Response to rising is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn top() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.front_page().top()) {
            Ok(response) => println!("Response to top is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn subreddit_top() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.subreddit(SUBREDDIT).top()) {
            Ok(response) => println!("Response to top is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn controversial() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.front_page().controversial()) {
            Ok(response) => println!("Response to controversial is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn subreddit_controversial() {
        let mut pants = build_pants();

        match tokio_test::block_on(pants.subreddit(SUBREDDIT).controversial()) {
            Ok(response) => println!("Response to controversial is: {:#?}", response),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }

    #[test]
    fn submit() {
        let mut pants = build_pants();

        let request_body = links_and_comments::ApiSubmit {
            url: "".to_string(),
            video_poster_url: "".to_string(),
            sendreplies: "".to_string(),
            collection_id: "".to_string(),
            resubmit: "".to_string(),
            richtext_json: "".to_string(),
            title: "Self Test title".to_string(),
            ad: "".to_string(),
            flair_text: "".to_string(),
            g_recaptcha_response: "".to_string(),
            extension: "".to_string(),
            nsfw: "".to_string(),
            api_type: "json".to_string(),
            kind: "self".to_string(),
            event_end: "".to_string(),
            event_start: "".to_string(),
            app: "".to_string(),
            flair_id: "".to_string(),
            event_tz: "".to_string(),
            sr: SUBREDDIT.to_string(),
            spoiler: "".to_string(),
            text: "Sample text".to_string(),
        };

        let response = match tokio_test::block_on(pants.submit(request_body)) {
            Ok(response) => response,
            Err(e) => panic!("An error ocurred: {}", e),
        };

        println!(
            "Response to submit is: {}",
            serde_json::to_string_pretty(&response).unwrap()
        );

        let submission_name = response.json.data.name;
        println!("The name of the submission is '{}'", submission_name);

        let delete_request_body = links_and_comments::ApiDel { id: submission_name };

        match tokio_test::block_on(pants.del(delete_request_body)) {
            Ok(response) => println!(
                "Response to submit is: {}",
                serde_json::to_string_pretty(&response).unwrap()
            ),
            Err(e) => panic!("An error ocurred: {}", e),
        };
    }
}
