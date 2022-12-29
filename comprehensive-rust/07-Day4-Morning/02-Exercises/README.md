# 32: Exercises

https://google.github.io/comprehensive-rust/exercises/day-4/morning.html

<!-- MarkdownTOC -->

- Exercises
    - 32.1: Dining Philosophers
    - 32.2: Multi-threaded Link Checker

<!-- /MarkdownTOC -->

## Exercises

### 32.1: Dining Philosophers

```bash
cd dining_philosophers;
cargo run;
```

Expected Output:

```bash
```

### 32.2: Multi-threaded Link Checker

```bash
cd link-checker;
cargo run;
```

<details>
    <summary>(Click to Show) Expected Output</summary>

    ```bash
       Compiling link-checker v0.1.0 (/Users/__USERNAME__/__PATHTO__/link-checker)
    Finished dev [unoptimized + debuginfo] target(s) in 14.49s
     Running `target/debug/link-checker`
Links: [
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.org",
            ),
        ),
        port: None,
        path: "/",
        query: None,
        fragment: Some(
            "content",
        ),
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.org",
            ),
        ),
        port: None,
        path: "/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.org",
            ),
        ),
        port: None,
        path: "/covid-19/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.org",
            ),
        ),
        port: None,
        path: "/our-work/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.org",
            ),
        ),
        port: None,
        path: "/our-approach/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.org",
            ),
        ),
        port: None,
        path: "/opportunities/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.org",
            ),
        ),
        port: None,
        path: "/latest/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.org",
            ),
        ),
        port: None,
        path: "/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.blog.google",
            ),
        ),
        port: None,
        path: "/outreach-initiatives/google-org/ideas-to-drive-climate-action/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "globalgoals.withgoogle.com",
            ),
        ),
        port: None,
        path: "/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.blog.google",
            ),
        ),
        port: None,
        path: "/outreach-initiatives/google-org/ongoing-support-for-refugees-and-displaced-people/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.org",
            ),
        ),
        port: None,
        path: "/racial-justice/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.org",
            ),
        ),
        port: None,
        path: "/covid-19/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.org",
            ),
        ),
        port: None,
        path: "/our-approach/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "twitter.com",
            ),
        ),
        port: None,
        path: "/Googleorg",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.youtube.com",
            ),
        ),
        port: None,
        path: "/user/Googleorg",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.org",
            ),
        ),
        port: None,
        path: "/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.org",
            ),
        ),
        port: None,
        path: "/covid-19/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.org",
            ),
        ),
        port: None,
        path: "/our-work/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.org",
            ),
        ),
        port: None,
        path: "/our-approach/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.org",
            ),
        ),
        port: None,
        path: "/opportunities/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.com",
            ),
        ),
        port: None,
        path: "/nonprofits/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "edu.google.com",
            ),
        ),
        port: None,
        path: "/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "grow.google",
            ),
        ),
        port: None,
        path: "/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "sustainability.google",
            ),
        ),
        port: None,
        path: "/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "crisisresponse.google",
            ),
        ),
        port: None,
        path: "/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "ai.google",
            ),
        ),
        port: None,
        path: "/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "newsinitiative.withgoogle.com",
            ),
        ),
        port: None,
        path: "/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "www.google.com",
            ),
        ),
        port: None,
        path: "/",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "policies.google.com",
            ),
        ),
        port: None,
        path: "/privacy",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "policies.google.com",
            ),
        ),
        port: None,
        path: "/terms",
        query: None,
        fragment: None,
    },
    Url {
        scheme: "https",
        cannot_be_a_base: false,
        username: "",
        password: None,
        host: Some(
            Domain(
                "support.google.com",
            ),
        ),
        port: None,
        path: "/",
        query: None,
        fragment: None,
    },
]
    ```
</details>
