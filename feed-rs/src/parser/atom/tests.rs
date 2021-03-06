use crate::model::{Entry, Person, Link, Feed, Text, Generator, Image, Category, Content};
use crate::parser;
use crate::util::test;

// Verify we can parse a more complete example than the one provided in the standard
#[test]
fn test_example_1() {
    // Parse the feed
    let test_data = test::fixture_as_string("atom_example_1.xml");
    let actual = parser::parse(test_data.as_bytes()).unwrap();

    // Expected feed
    let expected = Feed::default()
        .title(Text::new("dive into mark".into()))
        .description(Text::new("A <em>lot</em> of effort\n        went into making this effortless".into())
            .content_type("text/html"))
        .updated_rfc3339("2005-07-31T12:29:29Z")
        .id("tag:example.org,2003:3")
        .link(Link::new("http://example.org/".into())
            .rel("alternate")
            .media_type("text/html")
            .href_lang("en"))
        .link(Link::new("http://example.org/feed.atom".into())
            .rel("self")
            .media_type("application/atom+xml"))
        .rights(Text::new("Copyright (c) 2003, Mark Pilgrim".into()))
        .generator(Generator::new("Example Toolkit".into())
            .uri("http://www.example.com/")
            .version("1.0"))
        .entry(Entry::default()
            .id("tag:example.org,2003:3.2397")
            .title(Text::new("Atom draft-07 snapshot".into()))
            .updated_rfc3339("2005-07-31T12:29:29Z")
            .author(Person::new("Mark Pilgrim".into())
                .uri("http://example.org/")
                .email("f8dy@example.com"))
            .link(Link::new("http://example.org/2005/04/02/atom".into())
                .rel("alternate")
                .media_type("text/html"))
            .link(Link::new("http://example.org/audio/ph34r_my_podcast.mp3".into())
                .rel("enclosure")
                .media_type("audio/mpeg")
                .length(1337))
            .contributor(Person::new("Sam Ruby".into()))
            .contributor(Person::new("Joe Gregorio".to_string()))
            .content(Content::default()
                .content_type("text/html")
                .body("<div><p><i>[Update: The Atom draft is finished.]</i></p></div>"))
            .published_rfc3339("2003-12-13T08:29:29-04:00"));

    // Check
    assert_eq!(actual, expected);
}

// Real-life example from the Register - https://www.theregister.co.uk/science/headlines.atom
#[test]
fn test_example_2() {
    // Parse the feed
    let test_data = test::fixture_as_string("atom_example_2.xml");
    let actual = parser::parse(test_data.as_bytes()).unwrap();

    let expected = Feed::default()
        .id("tag:theregister.co.uk,2005:feed/theregister.co.uk/science/")
        .title(Text::new("The Register - Science".into()))
        .link(Link::new("https://www.theregister.co.uk/science/headlines.atom".into())
            .rel("self")
            .media_type("application/atom+xml"))
        .link(Link::new("https://www.theregister.co.uk/science/".into())
            .rel("alternate")
            .media_type("text/html"))
        .rights(Text::new("Copyright © 2019, Situation Publishing".into()))
        .author(Person::new("Team Register".into())
            .email("webmaster@theregister.co.uk")
            .uri("https://www.theregister.co.uk/odds/about/contact/"))
        .icon(Image::new("https://www.theregister.co.uk/Design/graphics/icons/favicon.png".into()))
        .description(Text::new("Biting the hand that feeds IT — sci/tech news and views for the world".into()))
        .logo(Image::new("https://www.theregister.co.uk/Design/graphics/Reg_default/The_Register_r.png".into()))
        .updated_rfc3339("2019-07-31T11:54:28Z")
        .entry(Entry::default()
            .id("tag:theregister.co.uk,2005:story204156")
            .updated_rfc3339("2019-07-31T11:54:28Z")
            .author(Person::new("Richard Speed".into())
                .uri("https://search.theregister.co.uk/?author=Richard%20Speed"))
            .link(Link::new("http://go.theregister.com/feed/www.theregister.co.uk/2019/07/31/orbitbeyond_drops_nasa_moon_contract/".into())
                .rel("alternate")
                .media_type("text/html"))
            .title(Text::new("Will someone plz dump our shizz on the Moon, NASA begs as one of the space biz vendors drops out".into())
                .content_type("text/html"))
            .summary(Text::new("<h4>OrbitBeyond begone: Getting to the Moon is <i>hard</i></h4> <p>NASA made a slew of announcements yesterday aimed at bigging up the agency's efforts to get commercial companies involved with its deep space ambitions – despite one vendor dumping plans for a 2020 lunar landing.…</p>".into())
                .content_type("text/html")))
        .entry(Entry::default()
            .id("tag:theregister.co.uk,2005:story204131")
            .updated_rfc3339("2019-07-30T05:41:09Z")
            .author(Person::new("Kieren McCarthy".into())
                .uri("https://search.theregister.co.uk/?author=Kieren%20McCarthy"))
            .link(Link::new("http://go.theregister.com/feed/www.theregister.co.uk/2019/07/30/french_arming_satellites/".into())
                .rel("alternate")
                .media_type("text/html"))
            .title(Text::new("Satellites with lasers and machine guns coming! China's new plans? Trump's Space Force? Nope, the French".into())
                .content_type("text/html"))
            .summary(Text::new(r#"<h4>After all, what could possibly go wrong, apart from everything?</h4> <p>France is threatening to stick submachine guns on its next generation of satellites as part of an "active space defense" strategy that would enable it to shoot down other space hardware.…</p>"#.to_owned())
                .content_type("text/html")));

    // Check
    assert_eq!(actual, expected);
}

// Real-life example from Akamai (includes categories and elements from a different namespace, along with locally declared namespaces on atom 1.0 elements)
// TODO test we ignore elements with the same name as Atom element but in a different namespace
#[test]
fn test_example_3() {
    // Parse the feed
    let test_data = test::fixture_as_string("atom_example_3.xml");
    let actual = parser::parse(test_data.as_bytes()).unwrap();

    let expected = Feed::default()
        .title(Text::new("The Akamai Blog".into()))
        .link(Link::new("https://blogs.akamai.com/".into())
            .rel("alternate")
            .media_type("text/html"))
        .id("tag:blogs.akamai.com,2019-07-30://2")
        .updated_rfc3339("2019-07-30T15:02:05Z")
        .generator(Generator::new("Movable Type Pro 5.2.13".into())
            .uri("http://www.sixapart.com/movabletype/"))
        .link(Link::new("http://feeds.feedburner.com/TheAkamaiBlog".into())
            .rel("self")
            .media_type("application/atom+xml"))
        .link(Link::new("http://pubsubhubbub.appspot.com/".into())
            .rel("hub"))
        .entry(Entry::default()
            .title(Text::new("Time to Transfer Risk: Why Security Complexity & VPNs Are No Longer Sustainable".into()))
            .link(Link::new("http://feedproxy.google.com/~r/TheAkamaiBlog/~3/NnQEuqRSyug/time-to-transfer-risk-why-security-complexity-vpns-are-no-longer-sustainable.html".into())
                .rel("alternate")
                .media_type("text/html"))
            .id("tag:blogs.akamai.com,2019://2.3337")
            .published_rfc3339("2019-07-30T16:00:00Z")
            .updated_rfc3339("2019-07-30T15:02:05Z")
            .summary(Text::new("Now, there are many reasons to isolate your infrastructure from the Internet. Minimizing the number of exposed things not only reduces risk, it also reduces operational complexity. VPNs are counter to this. VPNs make it so you aren't exposing all of your applications publicly in a DMZ, which is good. But for the most part, they still provide access to the corporate network to get access to corporate apps. Definitely bad. At this point, I think we all agree that moats and castles belong in the past.".into()))
            .author(Person::new("Lorenz Jakober".into()))
            .category(Category::new("Zero Trust".into())
                .scheme("http://www.sixapart.com/ns/types#category"))
            .category(Category::new("ssl".into())
                .label("SSL")
                .scheme("http://www.sixapart.com/ns/types#tag"))
            .category(Category::new("zerotrust".into())
                .label("Zero Trust")
                .scheme("http://www.sixapart.com/ns/types#tag"))
            .content(Content::default()
                .body(r#"<p>We all heed the gospel of patching, but as recent incidents made clear, even cutting-edge disruptors struggle to patch everything, everywhere, and all the time.</p>
        <img src="http://feeds.feedburner.com/~r/TheAkamaiBlog/~4/NnQEuqRSyug" height="1" width="1" alt=""/>"#)
                .content_type("text/html")));

    // Check
    assert_eq!(actual, expected);
}

// Real-life example from ebmpapst (CDATA text elements, unusual category representation)
#[test]
fn test_example_4() {
    // Parse the feed
    let test_data = test::fixture_as_string("atom_example_4.xml");
    let actual = parser::parse(test_data.as_bytes()).unwrap();

    let expected = Feed::default()
        .author(Person::new("ebm-papst".into()))
        .link(Link::new("http://www.ebmpapst.com/en/ebmpapst_productnews_atom_feed.xml".into())
            .rel("self")
            .media_type("application/atom+xml"))
        .title(Text::new("ebm-papst product news".into()))
        .id("tag:ebmpapst.com,2011-06-30:1309426729931")
        .updated_rfc3339("2019-07-29T09:41:09Z")
        .entry(Entry::default()
            .title(Text::new("Connection with future".into()))
            .link(Link::new("https://idt.ebmpapst.com/de/en/idt/campaign/simatic-micro-drive.html".into())
                .rel("alternate"))
            .id("tag:ebmpapst.com,2019-07-17:0310161724098")
            .updated_rfc3339("2019-07-17T03:10:16Z")
            .summary(Text::new(r#"<a href="https://idt.ebmpapst.com/de/en/idt/campaign/simatic-micro-drive.html"><img src="http://www.ebmpapst.com//media/content/homepage/currenttopic/ads_cd2013/FF_ep_keyvisual_100px.jpg" border="0" align="right"></a> Working in perfect harmony: the ebm-papst drive solutions for SIMATIC MICRO-DRIVE drive regulators from Siemens."#.to_owned())
                .content_type("text/html")));

    // Check
    assert_eq!(actual, expected);
}

// Real-life example from USGS (CDATA, different namespaces)
#[test]
fn test_example_5() {
    // Parse the feed
    let test_data = test::fixture_as_string("atom_example_5.xml");
    let actual = parser::parse(test_data.as_bytes()).unwrap();

    let expected = Feed::default()
        .title(Text::new("USGS Magnitude 2.5+ Earthquakes, Past Hour".into()))
        .updated_rfc3339("2019-07-31T13:17:27Z")
        .author(Person::new("U.S. Geological Survey".into())
            .uri("https://earthquake.usgs.gov/"))
        .id("https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/2.5_hour.atom")
        .link(Link::new("https://earthquake.usgs.gov/earthquakes/feed/v1.0/summary/2.5_hour.atom".into())
            .rel("self"))
        .icon(Image::new("https://earthquake.usgs.gov/favicon.ico".into()))
        .entry(Entry::default()
            .id("urn:earthquake-usgs-gov:nc:73239366")
            .title(Text::new("M 3.6 - 15km W of Petrolia, CA".into()))
            .updated_rfc3339("2019-07-31T13:07:31.364Z")
            .link(Link::new("https://earthquake.usgs.gov/earthquakes/eventpage/nc73239366".into())
                .rel("alternate")
                .media_type("text/html"))
            .summary(Text::new(r#"<p class="quicksummary"><a href="https://earthquake.usgs.gov/earthquakes/eventpage/nc73239366#shakemap" title="ShakeMap maximum estimated intensity" class="mmi-II">ShakeMap - <strong class="roman">II</strong></a> <a href="https://earthquake.usgs.gov/earthquakes/eventpage/nc73239366#dyfi" class="mmi-IV" title="Did You Feel It? maximum reported intensity (4 reports)">DYFI? - <strong class="roman">IV</strong></a></p><dl><dt>Time</dt><dd>2019-07-31 12:26:15 UTC</dd><dd>2019-07-31 04:26:15 -08:00 at epicenter</dd><dt>Location</dt><dd>40.347&deg;N 124.460&deg;W</dd><dt>Depth</dt><dd>29.35 km (18.24 mi)</dd></dl>"#.to_owned())
                .content_type("text/html"))
            .category(Category::new("Past Hour".into())
                .label("Age"))
            .category(Category::new("Magnitude 3".into())
                .label("Magnitude"))
            .category(Category::new("nc".into())
                .label("Contributor"))
            .category(Category::new("nc".into())
                .label("Author")));

    // Check
    assert_eq!(actual, expected);
}


// GitHub Atom feed for feed-rs (https://github.com/feed-rs/feed-rs/issues/6)
#[test]
fn test_example_6() {
    // Parse the feed
    let test_data = test::fixture_as_string("atom_example_6.xml");
    let actual = parser::parse(test_data.as_bytes()).unwrap();

    let expected = Feed::default()
        .id("tag:github.com,2008:https://github.com/feed-rs/feed-rs/releases")
        .link(Link::new("https://github.com/feed-rs/feed-rs/releases".into())
            .rel("alternate")
            .media_type("text/html"))
        .link(Link::new("https://github.com/feed-rs/feed-rs/releases.atom".into())
            .rel("self")
            .media_type("application/atom+xml"))
        .title(Text::new("Release notes from feed-rs".into()))
        .updated_rfc3339("2020-01-19T16:01:56+11:00")
        .entry(Entry::default()
            .id("tag:github.com,2008:Repository/90976281/v0.2.0")
            .updated_rfc3339("2020-01-19T16:08:59+11:00")
            .link(Link::new("https://github.com/feed-rs/feed-rs/releases/tag/v0.2.0".into())
                .rel("alternate")
                .media_type("text/html"))
            .title(Text::new("0.2.0".into()))
            .content(Content::default()
                .body(r#"<p>A range of maintenance work, including:</p>
            <ul>
            <li>migrate to Rust 2018 edition</li>
            <li>Align domain model around Atom spec as it is more modern+complete</li>
            <li>Switch to event-based parser (xml-rs) to reduce peak memory usage and use of clone()</li>
            <li>Expanded test coverage</li>
            <li>Documentation improvements</li>
            </ul>"#)
                .content_type("text/html"))
            .author(Person::new("markpritchard".into())))
        .entry(Entry::default()
            .id("tag:github.com,2008:Repository/90976281/0.1.3")
            .updated_rfc3339("2017-07-07T21:47:46+10:00")
            .link(Link::new("https://github.com/feed-rs/feed-rs/releases/tag/0.1.3".into())
                .rel("alternate")
                .media_type("text/html"))
            .title(Text::new("0.1.3".into()))
            .content(Content::default()
                .body(r#"<p>Update version to 0.1.3</p>"#)
                .content_type("text/html"))
            .author(Person::new("kumabook".into())))
        .entry(Entry::default()
            .id("tag:github.com,2008:Repository/90976281/0.1.1")
            .updated_rfc3339("2017-06-16T18:49:36+10:00")
            .link(Link::new("https://github.com/feed-rs/feed-rs/releases/tag/0.1.1".into())
                .rel("alternate")
                .media_type("text/html"))
            .title(Text::new("0.1.1".into()))
            .content(Content::default()
                .body(r#"<p>Handle rel attribute of link element of entry of atom</p>"#)
                .content_type("text/html"))
            .author(Person::new("kumabook".into())))
        .entry(Entry::default()
            .id("tag:github.com,2008:Repository/90976281/0.1.0")
            .updated_rfc3339("2017-06-15T16:44:26+10:00")
            .link(Link::new("https://github.com/feed-rs/feed-rs/releases/tag/0.1.0".into())
                .rel("alternate")
                .media_type("text/html"))
            .title(Text::new("0.1.0".into()))
            .content(Content::default()
                .body(r#"<p>Update crate info to Cargo.toml</p>"#)
                .content_type("text/html"))
            .author(Person::new("kumabook".into())));

    // Check
    assert_eq!(actual, expected);
}

// Verify we can parse the example contained in the Atom specification
#[test]
fn test_spec_1() {
    // Parse the feed
    let test_data = test::fixture_as_string("atom_spec_1.xml");
    let actual = parser::parse(test_data.as_bytes()).unwrap();

    // Expected feed
    let expected = Feed::default()
        .id("urn:uuid:60a76c80-d399-11d9-b93C-0003939e0af6")
        .title(Text::new("Example Feed".into()))
        .link(Link::new("http://example.org/".into())
            .rel("alternate"))
        .updated_rfc3339("2003-12-13T18:30:02Z")
        .author(Person::new("John Doe".into()))
        .entry(Entry::default()
            .id("urn:uuid:1225c695-cfb8-4ebb-aaaa-80da344efa6a")
            .title(Text::new("Atom-Powered Robots Run Amok".into()))
            .updated_rfc3339("2003-12-13T18:30:02Z")
            .summary(Text::new("Some text.".into()))
            .link(Link::new("http://example.org/2003/12/13/atom03".into())
                .rel("alternate")));

    // Check
    assert_eq!(actual, expected);
}
