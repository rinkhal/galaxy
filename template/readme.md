# Galaxy

{{#each languages as |lang|}}
- [{{lang.name}}](#{{lang.name}})
{{/each}}

{{#each languages as |lang|}}
## {{lang.name}}
{{#each lang.stars as |star|}}
- [{{star.name}}]({{star.html_url}})
{{/each}}

{{/each}}
