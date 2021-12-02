# Inox

> Email with Notmuch Rust

[![Build Status](https://travis-ci.org/vhdirk/inox.svg?branch=master)](https://travis-ci.org/vhdirk/inox)

An experimental email client based on notmuch, along with other utilities like
afew and offlineimap/isync.

The UI is ported from [Geary](https://wiki.gnome.org/Apps/Geary), and a lot of inspiration comes from [Astroid](https://github.com/astroidmail/astroid) and [alot](https://github.com/pazz/alot).
I like how alot is all command driven. I also like how VSCode/Atom/Textmate work.
So that's probably how I want this thing to work, too.

## Status

You can read mails, and even that needs improvements.

## Goals

I'm aiming for a nice and full-featured and command-driven UI.

All settings should be stored in easily readable/editable config files, so you
can centralize them in your dotfiles.
While Astroid relies on an external editor for writing emails, I aim for a
default embedded editor. I do like the option of using the editor of your liking, though.
The embedded editor would be plaintext (not WYSIWYG) with rendering markdown or reStructuredText to html (like github/gitlab).

