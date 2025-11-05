# Portfolio and Ecosystem

This repository even if named `portfolio` it's an ecosystem resume or
entry site. It's purpose is to connect all the services I host such
as my blog (coming soon), the things I sell and so on.

## Contents and Objectives

The objectives for my whole ecosystem are defined in various
issues and some not yet written issues @ [`the ecosystem project`](https://github.com/users/stifskere/projects/3)

> [!IMPORTANT]
> This section is very likely to change in a near future.

The overall contents this page should have are
- A top section
	- The top section should have some nice animations (first impressions matter)
	- It should contain the socials I use along some quick links to other parts of the ecosystem.
- A presentation section
	- I'm not yet sure if I should merge this with the top section
	- Essentially a description of who I am, what I do and an extended social profiles view.
	- Some widgets such as discord activity, GitHub rating and others.
- A programming projects section
	- This specially is required for me to show GitHub activity, such as issues/pr I'm subscribed to
	- It should also show the repositories ordered by last commit date, maybe the last 5 repositories.
	- An overview of the actual deployed projects, this may be configured in an admin page.
	- Blog posts with the `software` tag could be shown yes.
- A hardware projects section
	- I sell and repair computers, a display of the built computers along images would be nice like articles.
	- I also repair motherboards/GPUs and other components as a side business, an overview to a shop of mine would be cool.
	- Blog posts with the `hardware` tag could be shown yes.
- A contact section
	- The section should have an email form, this form would send an email to the sender to verify identity.
	- It would be cool for a WhatsApp link that says "Find email boring?" and changes the whole thing to a chat.
- A footer with a summary of links
	- The links should be ordered by sections, the sections should have a position and the links within should be ordered by last modified.
	- A small contact card besides the links.

For the other part, this should have an administration panel, as I don't want to touch code
to modify some part of the portfolio which could specially be modified with a form.

The administration panel would have a simple authentication handled with environment variables
which can then be applied while running from terraform.

It should contain the sections to modify as
- A form for the presentation section
	- This form should contain the portfolio `moto` which is also shown in the window title and `og` tags.
	- The presentation section (about me) should be able to be modified from here.
	- This should be available in multiple languages.
- A logo upload button
- An `OAuth2` section
	- All the GitHub related content should be obtained from the `OAuth2` user perspective (me)
	- Maybe also a Discord login to display current activity in a widget
- A displayed social media section
	- I should be able to choose which social media to display in order to show that in the portfolio.
- Blog configuration section
	- Maybe change the tags shown in each category and limit how many posts are shown.
- Shop configuration section
	- Maybe change some aspects of some posts or other render configuration.
- A done things section
	- Things that don't update automatically such as built computers or actual projects should be added from here
	- It should have a micro component system so I can add multiple buttons and images per post.
- Contact configuration
	- An email configuration to configure the relay, who sends and who receives.
	- WhatsApp configuration where should a message redirect to (for now).
	- Should also contain a little section for email history, send attempts, if a message is lost in the way be able to recover it.
- Webhook configuration
	- Having an `n8n` in the infrastructure would be really cool, being able to configure WebHooks too.
	- Would permit posting updates to other social media maybe.
- A footer modification section.


## Deployments

For now this hosts a `coming soon` page, which won't be touched until
all the back-end needs are met.

The deployments are described in [`BUILDING.md`](/BUILDING.md), basically
a deployment is triggered when pushing to main, the container
will be built in production mode and pushed
if all the crate versions equal.

## License

This is dual licensed by [Apache-2](/LICENSE-APACHE) or [MIT](/LICENSE-MIT) at your disposal.
The licenses are provided in the repository.
