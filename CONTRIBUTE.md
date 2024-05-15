# Contributing to DockyMcProxy 🚀

First off, thank you for considering contributing to DockyMcProxy. It's people like you that make DockyMcProxy such a great tool. 💪

## Where do I go from here? 🔍

If you've noticed a bug or have a feature request, make sure to check our [Issues](https://github.com/unit214/docky-mc-proxy/issues) to see if someone else in the community has already created a ticket. If not, go ahead and [make one](https://github.com/unit214/docky-mc-proxy/issues/new)! 📝

## Fork & Create a Branch 🌿

If this is something you think you can fix, then [fork DockyMcProxy](https://help.github.com/articles/fork-a-repo) and create a branch with a descriptive name. 🏷️

A good branch name would be (where issue #325 is the ticket you're working on):

```bash
git checkout -b 325-add-japanese-localization
```

## Implement Your Fix or Feature 🛠️

At this point, you're ready to make your changes! Feel free to ask for help; everyone is a beginner at first. 🤝

## Get the Code 📥

The first thing you'll need to do is get the code. The easiest way to do this is to fork the repository, and then clone your fork:

```bash
git clone https://github.com/your-username/docky-mc-proxy.git
cd docky-mc-proxy
```

## Make Your Changes ✏️

Make the changes to the code and the documentation, and make sure that your changes are in line with the guidelines described in the [README.md](https://github.com/unit214/docky-mc-proxy/blob/main/README.md). 📚

## Test Your Changes ✅

It's important to ensure that your changes don't break anything and that the code adheres to the existing style. 🧪

## Create a Pull Request 🔄

At this point, you should switch back to your main branch and make sure it's up to date with DockyMcProxy's main branch:

```bash
git remote add upstream https://github.com/unit214/docky-mc-proxy.git
git checkout main
git pull upstream main
```

Then update your feature branch from your local copy of main, and push it! 🚀

```bash
git checkout 325-add-japanese-localization
git rebase main
git push --set-upstream origin 325-add-japanese-localization
```

Go to the [DockyMcProxy repo](https://github.com/unit214/docky-mc-proxy) and press the "Compare & pull request" button. 🔍

Write a [good, clear pull request message](http://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html), and press the "Create pull request" button. ✨

And you're done! Well, not quite. There will probably be discussion about the pull request, and, hopefully, your code will get merged in! 🎉