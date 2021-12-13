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

* UI: although I'm an avid terminal user, I like mail to be displayed in a proper
  desktop client. I'm aiming for a nice and full-featured and command-driven UI.
  For now, the toolkit of choice is GTK. However; VsCode's extension model
  seems to work pretty well. A sort of vscode for mail would be pretty cool.
* Settings: should be stored in easily readable/editable config files,
  so you can centralize them in your dotfiles.
* Editor: While Astroid relies on an external editor for writing emails, I aim for a
  default embedded editor. I do like the option of using the editor of your
  liking, though.
  The embedded editor would be plaintext (not WYSIWYG) with rendering markdown
  or reStructuredText to html (like github/gitlab).
* Plug-ins/extensions. If everything stays in rust, WASM would be the way to go.
  If not, it would probably look like vscodes extension model.
* Core: Everything not UI related should end up centralized. Somewhat like
  Mailcore2.

