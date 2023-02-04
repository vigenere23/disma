export type Feature = {
  title: string
  description: string
  code: string
  language: string
}

export const features: Feature[] = [
  {
    title: "Design once, repeat infinitely",
    description:
      "Loop over repeated configs instead of copy-pasting by leveraging the power of handlebars templating.",
    code: `
categories:
  items:
  {{#each teams}}
    - name: Team-{{this}}
  {{/each}}

channels:
  items:
  {{#each teams}}
    - category: Team-{{this}}
      name: general
  {{/each}}
    `,
    language: "handlebars",
  },
  {
    title: "See changes before they happen",
    description:
      "Always lists the commands to apply and asks for confirmation before applying them.",
    code: `
➜ 🔎 Looking for changes...
➜ 📜 Found the following changes :

● 🗑️  Removing Role team-26

● 🗑️  Removing Category Team-26

● 🗑️  Removing Channel Team-26:general (TEXT)

● 🗑️  Removing Channel Team-26:General (VOICE)

➜ ❔ Ready to apply? (y/N)
    `,
    language: "text",
  },
  {
    title: "Start slowly, adopt incrementally",
    description:
      "Skip existing items that are not yet present in the config to integrate them one step at a time.",
    code: `
roles:
  items:
  - name: Staff
    show_in_sidebar: true
    mentionable: true
    permissions: [ADMINISTRATOR]
  - name: Student
    show_in_sidebar: false
    mentionable: false
    permissions: [VIEW_CHANNEL]
  extra_items:
    strategy: KEEP
    `,
    language: "yaml",
  },
  {
    title: "Explicitize the implicit",
    description:
      "Clearly define if channel's permissions should always be synched with its category's.",
    code: `
categories:
  items:
  - name: 'Team-23'
    permissions_overwrites:
    - role: team23
      allow: [VIEW_CHANNEL]
channels:
  items:
  - category: 'Team-23'
    name: general
    permissions_overwrites:
      strategy: FROM_CATEGORY
    `,
    language: "yaml",
  },
]
