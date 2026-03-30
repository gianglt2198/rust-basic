// use trpl::{Html, Either};
// use std::env;

// async fn page_title(url: &str) -> (&str, Option<String>) {
//     let text = trpl::get(url).await.text().await;
//     let title = Html::parse(&text).select_first("title").map(|title| title.inner_html());
//     (url, title)
// }

use trpl::StreamExt;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // trpl::block_on(async {
    //     let title_1 = page_title(&args[1]);
    //     let title_2 = page_title(&args[2]);

    //     let (url, maybe_title) = match trpl::select(title_1, title_2).await {
    //         Either::Left(left) => left,
    //         Either::Right(right) => right,
    //     };

    //     match maybe_title {
    //         Some(title) => println!("The title for {url} was {title}"),
    //         None => println!("{url} has no title"),
    //     }
    // })

    trpl::block_on(async {
   let values = [1,2,3,4,5,6,7,8,9];
    let iter = values.iter();
    let mut stream = trpl::stream_from_iter(iter);

    while let Some(value) = stream.next().await {
        println!("The value was: {value}");
    }
    });
}
