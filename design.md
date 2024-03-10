This library doesn't operate on Japanese.
It operates on an imaginary language that no meaningful words.

Let's call it **Mugo** (無語).
In Mugo, there are no words, only expressions, which are sequences of hiragana.

Let's look at the expression 「うごきませんでした」 for example.
In Mugo, this doesn't mean anything specific. There are no words.
Instead, it's a a list of **roots**. A root is a piece of hiragana **text** and
its conjugation **steps**.

「うごきませんでした」can be all of these **roots**:

| **text**                            | **steps**
|-------------------------------------|---------------------------------------------
| ichidan (ru) verb 「うごきませんでし」 | た (past)
| godan su verb 「うごきませんで」       | (し)た (past)
| godan ku verb 「うご」               | (き)ません (polite negative) + (でし)た (past)

This library takes a Mugo expression and lists all of the possible **roots** for it.
The user of this library then (presumably) uses a dictionary to filter out
roots that are actual words.