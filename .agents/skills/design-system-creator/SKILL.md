---
name: design-system-creator
description: Establish the visual language, brand philosophy, interaction principles, and UI architecture of a project before feature-level implementation begins. Make sure to use this skill whenever the user mentions setting up a design system, defining brand identity, creating color palettes, establishing UI rules, writing brand philosophy, or configuring AGENTS.md for visual and design guidelines. Acts as a design system architect that inspects the technology stack and existing codebase, asks targeted high-impact questions, and generates machine-readable DESIGN.md and BRAND_PHILOSOPHY.md specifications that bind all future AI sessions to prevent UI drift.
---

# Design System & Brand Philosophy Architect

`design-system-creator` establishes the visual language, interaction principles, brand identity, and UI implementation rules of a project before feature-level UI development begins. Its primary purpose is to prevent **design drift across separate AI coding sessions**. The skill analyzes the existing project, understands its technology stack, audits any existing design decisions, asks the user only for high-impact decisions that cannot be reliably inferred, and produces two durable project specifications:

* `BRAND_PHILOSOPHY.md` — defines **WHY** the product looks, feels, and communicates the way it does.
* `DESIGN.md` — defines **HOW** that philosophy becomes an interface and implementation system.

It then integrates those specifications into `AGENTS.md` so future AI coding sessions are explicitly instructed to follow them.

The skill should behave as a **design-system architect**, not as a document generator.

---

# Core Objective

The skill must establish a coherent design language that future AI sessions can reliably follow.

It should answer:

> What should this product feel like?

> Why should it feel that way?

> How should that feeling translate into visual and interaction decisions?

> How should those decisions be implemented in this project's technology stack?

> What rules should future AI sessions follow when creating new UI?

The final system should create a chain of authority:

```text
BRAND_PHILOSOPHY.md
        │
        │ WHY
        ↓
    DESIGN.md
        │
        │ HOW
        ↓
    AGENTS.md
        │
        │ ENFORCE
        ↓
Future AI Sessions
```

---

# Core Principles

The skill MUST follow these principles.

## 1. Discover Before Designing

Never immediately invent a new visual system.

First inspect the project and determine what already exists.

Existing intentional design decisions should be preserved unless the user explicitly requests a redesign.

---

## 2. Audit Before Replacing

An existing project may already contain a partial design system.

Determine whether existing decisions are:

* intentional and consistent
* intentional but incomplete
* inconsistent
* unclear
* conflicting
* missing

Do not assume that an existing implementation is wrong simply because it is undocumented.

---

## 3. Ask About Intent, Not Pixels

Do not overwhelm the user with low-level questions such as:

* What border radius should buttons use?
* What exact spacing should cards have?
* What exact shadow should dialogs use?
* What should the heading line-height be?

Instead ask about high-level intent:

* What should the product feel like?
* How expressive should it be?
* What should it avoid?
* What products or aesthetics inspire it?
* What should the user feel while using it?

Translate those answers into concrete design decisions.

---

## 4. Infer Where Reasonable

If the answer can be confidently derived from:

* existing code
* existing tokens
* installed libraries
* existing components
* project configuration
* brand assets
* existing UI patterns

do not ask the user unnecessarily.

Record important inferred decisions as **inferred**, rather than presenting them as explicit user requirements.

---

## 5. Never Silently Resolve Significant Conflicts

If existing implementation conflicts with the emerging design system, do not silently rewrite it.

Classify the conflict and ask the user when the decision has meaningful product or visual consequences.

For example:

```text
Existing:
Buttons use 12px radius.

Existing:
Cards use 24px radius.

Proposed:
System favors restrained 8px radius.

Status:
CONFLICT

Action:
Ask whether existing values should be preserved,
normalized, or redesigned.
```

---

## 6. Preserve User Intent Over System Defaults

The system is a framework, not a prison.

Priority should generally be:

```text
Explicit current user instruction
        ↓
Existing product requirements
        ↓
BRAND_PHILOSOPHY.md
        ↓
DESIGN.md
        ↓
Existing component conventions
        ↓
Reasonable AI inference
```

If the user explicitly requests something that differs from the existing design system, follow the user's request and determine whether the design system should subsequently be updated.

---

## 7. Extend the System Instead of Violating It

If a new UI requirement is not covered by `DESIGN.md`:

1. Determine whether an existing pattern can be composed.
2. If not, derive a new pattern from the existing design language.
3. If the new pattern represents a meaningful system-level decision, update `DESIGN.md`.
4. Do not introduce unrelated visual styles merely because the existing document does not explicitly mention them.

---

## 8. Be Idempotent

Running this skill multiple times must not create duplicated documentation or conflicting rules.

If:

* `BRAND_PHILOSOPHY.md` already exists
* `DESIGN.md` already exists
* `AGENTS.md` already contains the managed design-system section

update and improve the existing artifacts rather than blindly replacing or duplicating them.

---

# Architectural Execution Flow

```text
1. Discover
      ↓
2. Audit
      ↓
3. Classify
      ↓
4. Identify Missing Decisions
      ↓
5. Ask Targeted Questions
      ↓
6. Synthesize Design Direction
      ↓
7. Generate / Update BRAND_PHILOSOPHY.md
      ↓
8. Generate / Update DESIGN.md
      ↓
9. Validate
      ↓
10. Update / Create AGENTS.md
      ↓
11. Final Verification
```

---

# Step 1 — Discover Project Context

Before asking the user questions, inspect the project.

Determine:

## Framework

Identify the active framework and architecture.

Examples:

* Next.js App Router
* Next.js Pages Router
* React + Vite
* React Native
* Vue
* Nuxt
* Svelte
* SvelteKit
* Shopify Liquid
* Vanilla HTML/CSS/JS
* Other

---

## Styling System

Determine whether the project uses:

* Tailwind CSS
* Tailwind CSS v3
* Tailwind CSS v4
* Vanilla CSS
* CSS Modules
* Styled Components
* Emotion
* Sass / SCSS
* CSS-in-JS
* Design-token CSS variables
* Other

---

## Component System

Determine whether the project uses:

* shadcn/ui
* Radix UI
* Material UI
* Chakra UI
* Ant Design
* Polaris
* React Aria
* Custom components
* No component library

---

## Icons

Identify the active icon system.

Examples:

* Lucide
* Heroicons
* Font Awesome
* Material Icons
* Custom SVG
* Other

Do not introduce a new icon library if the project already has an established one unless there is a clear reason.

---

## Typography

Inspect:

* `next/font`
* Google Fonts
* local font files
* CSS font declarations
* Tailwind font configuration
* typography tokens

Record existing font choices.

---

## Animation / Motion

Identify:

* GSAP
* Framer Motion
* Motion
* CSS animations
* Web Animations API
* React Native Reanimated
* no motion system

---

## Theme / Token Infrastructure

Inspect for:

* CSS custom properties
* Tailwind theme configuration
* design tokens
* dark mode
* light mode
* theme providers
* color schemes
* existing semantic variables

---

## Relevant Project Files

Inspect appropriate files such as:

```text
package.json
components.json
tailwind.config.*
app/globals.css
src/styles/*
src/components/*
src/design-system/*
src/theme/*
theme/*
```

Use the actual project structure rather than assuming these paths exist.

---

# Step 2 — Read Stack Constraints

Read:

```text
references/stack-constraints.md
```

Use this document to understand ecosystem-specific conventions and limitations.

The generated design system must be compatible with the project's actual stack.

For example:

### Tailwind

Prefer design tokens and semantic utilities over arbitrary values.

Avoid unnecessary patterns such as:

```text
w-[287px]
mt-[37px]
bg-[#123456]
```

when an existing design token can represent the same intent.

### shadcn/ui

Prefer extending and composing existing primitives rather than recreating accessible primitives from scratch.

### React Native

Do not assume browser CSS, Tailwind behavior, or DOM semantics.

### Shopify

Respect Shopify theme architecture, merchant customization, Liquid constraints, and existing theme conventions.

The design system must adapt to the ecosystem instead of forcing a generic implementation model onto it.

---

# Step 3 — Audit Existing UI

If UI already exists, inspect representative screens and components.

Look for:

## Colors

Identify:

* primary colors
* secondary colors
* backgrounds
* surfaces
* text colors
* borders
* status colors
* dark-mode colors

---

## Typography

Identify:

* heading hierarchy
* body text
* labels
* captions
* font families
* weights
* sizes
* line heights

---

## Spacing

Identify repeated:

* padding
* margins
* gaps
* section spacing
* container widths

Determine whether a consistent spatial rhythm already exists.

---

## Shape

Inspect:

* border radius
* borders
* shadows
* card treatment
* button shape
* input shape

---

## Components

Identify recurring patterns:

* buttons
* inputs
* cards
* dialogs
* navigation
* tables
* badges
* alerts
* forms
* empty states
* loading states

Determine whether these are genuinely reusable patterns or accidental duplication.

---

## Motion

Inspect:

* page transitions
* hover effects
* scroll animations
* entrance animations
* loading animations
* micro-interactions

Determine whether motion has a consistent personality.

---

# Step 4 — Classify Findings

Classify discovered design decisions into one of the following categories:

```text
EXPLICIT
Clearly defined by the user or project requirements.

EXISTING
Clearly implemented in the current project.

INFERRED
Strongly implied by existing implementation.

RECOMMENDED
A new design-system recommendation proposed by the skill.

MISSING
Required for a coherent system but currently undefined.

INCONSISTENT
Multiple conflicting implementations exist.

CONFLICTING
Existing behavior conflicts with an explicit user requirement.
```

Do not treat `INFERRED` or `RECOMMENDED` decisions as user requirements.

---

# Step 5 — Detect Design-System Maturity

Determine the approximate maturity of the project's design system.

## Stage 0 — No System

Little or no consistent visual language exists.

## Stage 1 — Informal System

Visual decisions exist but are scattered throughout the codebase.

## Stage 2 — Tokenized

Colors, spacing, typography, or other primitives are partially centralized.

## Stage 3 — Componentized

Reusable UI components and patterns exist.

## Stage 4 — Documented

The design system is documented and consistently implemented.

## Stage 5 — Synchronized

Design decisions, tokens, components, documentation, and implementation are actively kept in sync.

Use this maturity level to determine whether to:

* create
* normalize
* document
* audit
* or extend

the existing system.

Do not rebuild a mature system unnecessarily.

---

# Step 6 — Identify Missing Decisions

Determine which high-level decisions are required before generating the specifications.

Focus on decisions that materially affect the product's identity.

Do not ask questions whose answers can be reliably inferred.

Potential areas:

## Product Personality

What should the product feel like?

Possible directions:

* Minimal / editorial
* Technical / precise
* Calm / reflective
* Energetic / bold
* Warm / approachable
* Premium / sophisticated
* Playful / expressive

Allow the user to provide their own direction.

---

## Visual Expressiveness

Determine whether the design should be:

```text
Highly restrained
        ↓
Balanced
        ↓
Expressive / dynamic
```

This influences:

* visual variation
* typography scale
* decoration
* animation
* layout composition

---

## Brand Voice

Determine:

* tone
* writing personality
* microcopy style
* terminology
* messaging philosophy

Ask for a phrase or mindset when useful.

Example:

> "Understand yourself better"

is meaningfully different from:

> "Optimize yourself harder."

---

## Emotional Direction

Determine how users should feel while interacting with the product.

Examples:

* calm
* confident
* focused
* energized
* curious
* empowered
* secure
* sophisticated

---

## Visual References

Ask whether the user has:

* reference products
* websites
* design systems
* brands
* screenshots
* visual styles

that they admire.

Use references to understand direction, not to blindly copy another product.

---

## Anti-Tropes

Ask what the product should explicitly avoid.

Examples:

* excessive gradients
* generic SaaS appearance
* excessive glassmorphism
* over-rounded cards
* excessive shadows
* neon accents
* excessive animation
* visual clutter
* unnecessary badges
* nested cards
* excessive gamification

---

# Step 7 — Ask Targeted Questions

Ask only the questions necessary to resolve meaningful uncertainty.

Do not interrogate the user with dozens of questions.

Prefer a small number of high-impact questions.

A good question:

> Should the interface feel restrained and editorial, technical and precise, or expressive and dynamic?

A bad question:

> Should cards have 12px or 16px radius?

The skill should translate human preferences into implementation decisions.

If sufficient information already exists, do not ask unnecessary questions.

---

# Step 8 — Generate `BRAND_PHILOSOPHY.md`

Create or update:

```text
BRAND_PHILOSOPHY.md
```

This document answers:

> **WHY does this product communicate and look the way it does?**

It should contain:

## 1. Overview & Mission

Explain:

* what the product is
* what problem it solves
* what role the interface plays

---

## 2. Brand Personality

Define approximately 3–5 core personality traits.

Each trait should be described in human terms.

Also define personality anti-patterns.

Example:

```text
Core traits:
- Precise
- Calm
- Intelligent
- Focused

Avoid:
- Loud
- Chaotic
- Overly playful
- Corporate
```

---

## 3. Voice & Tone

Define:

* writing style
* tone
* terminology
* microcopy behavior
* tagline or core framing when available

---

## 4. Target User & Emotional Direction

Describe:

* target user
* user context
* desired emotional state
* what the interface should help the user feel

Do not invent detailed personas without evidence.

Clearly label assumptions.

---

## 5. Visual References & Anti-Tropes

Document:

* aesthetic references
* visual inspiration
* patterns worth adopting
* patterns explicitly forbidden

References are directional rather than instructions to clone another product.

---

## 6. Brand Do's & Don'ts

Provide practical rules for:

* visual identity
* language
* messaging
* interaction personality

---

## 7. Design Decision Record

Where useful, record major brand decisions:

```text
Decision
Rationale
Source
Confidence
```

Possible sources:

* User
* Existing project
* Inferred
* Recommended

---

# Step 9 — Generate `DESIGN.md`

Create or update:

```text
DESIGN.md
```

This document answers:

> **HOW is the brand philosophy transformed into UI and code?**

It should contain the following sections.

---

## 1. Design Philosophy

Explain the core visual principles.

Connect the design decisions to the brand philosophy.

Do not merely list adjectives.

Explain how the principles affect UI.

Example:

```text
The interface favors strong hierarchy over decoration.
Visual emphasis should primarily come from typography,
spacing, scale, and contrast rather than decorative effects.
```

---

## 2. Technology & Stack Constraints

Document the actual stack:

```text
Framework
Styling
Component Library
Icons
Typography
Animation
Theme System
```

Then define implementation rules appropriate to the stack.

Examples:

* extend existing primitives
* reuse existing tokens
* avoid arbitrary values
* avoid unnecessary dependencies
* use semantic tokens
* preserve accessibility behavior

Never recommend technology that contradicts the project.

---

## 3. Color System & Tokens

Define semantic color roles.

At minimum consider:

```text
background
foreground
surface
surface-muted
primary
primary-foreground
secondary
secondary-foreground
muted
muted-foreground
border
input
ring
success
warning
error
info
```

Where applicable, define:

* light mode
* dark mode
* hover states
* active states
* disabled states

Prefer semantic roles over raw color names.

Prefer:

```text
--color-primary
```

over:

```text
--green-500
```

when the token represents a semantic role.

---

## 4. Typography System

Define:

* font families
* heading roles
* body roles
* labels
* captions
* monospace usage
* weights
* sizes
* line heights
* letter spacing

Use semantic roles where possible.

Example:

```text
display
heading-1
heading-2
heading-3
body-lg
body
body-sm
label
caption
code
```

---

## 5. Spacing & Spatial Grammar

Define:

* spacing scale
* base unit
* container width
* page padding
* section spacing
* component gaps
* card padding
* responsive scaling

The goal is to establish a consistent spatial rhythm rather than prescribe every individual pixel.

---

## 6. Shape, Borders & Elevation

Define:

* radius scale
* border philosophy
* border widths
* shadows
* surface hierarchy

Explain when each should be used.

Avoid creating multiple visually redundant treatments.

---

## 7. Component Principles

Define rules for recurring component categories.

At minimum consider:

### Buttons

Define:

* variants
* sizes
* hierarchy
* states
* shape
* icon behavior

### Cards

Define:

* purpose
* hierarchy
* padding
* borders
* elevation
* when not to use cards

### Form Controls

Define:

* labels
* inputs
* focus states
* validation
* errors
* disabled states

### Tables / Data Displays

Define:

* hierarchy
* density
* alignment
* empty states
* loading states

### Feedback

Define:

* alerts
* errors
* success
* warnings
* loading
* empty states

Do not create exhaustive component specifications unless the project actually needs them.

---

## 8. Composition & Engineering Rules

Define how components should be composed.

At minimum:

### Internal vs External Spacing

Components control internal spacing.

Parents control:

* positioning
* layout
* gaps
* relationships between components

Avoid reusable components containing arbitrary external margins.

---

### Reuse

Prefer:

```text
compose existing primitive
        ↓
extend existing component
        ↓
create new component
```

in that order.

Do not create a new component simply because a small variation exists.

---

### Token Compliance

Prefer design tokens over arbitrary values.

Do not introduce a new token without a meaningful semantic reason.

---

### Semantic Ownership

Components should own their internal behavior.

Layouts should own relationships between components.

---

## 9. Responsive Layout Principles

Define how the design adapts rather than merely listing breakpoints.

Consider:

* mobile-first behavior
* column collapse
* navigation simplification
* typography scaling
* spacing compression
* image behavior
* touch targets
* content priority

Interactive targets should generally provide approximately `44x44px` of usable touch area unless platform constraints or existing conventions justify otherwise.

---

## 10. Motion & Interaction Guidelines

Define:

* motion philosophy
* duration ranges
* easing
* hover behavior
* focus behavior
* entrance animation
* exit animation
* scroll animation
* loading states
* reduced-motion behavior

A typical baseline may use approximately:

```text
150ms–300ms
```

for common micro-interactions, but do not apply a rigid duration to every animation.

Motion should serve hierarchy, feedback, continuity, or orientation.

Do not add animation merely to make empty space feel interesting.

---

## 11. Accessibility Principles

Define baseline expectations for:

* semantic HTML
* keyboard navigation
* focus-visible states
* color contrast
* labels
* validation messaging
* reduced motion
* screen-reader behavior
* touch targets

Accessibility should be part of the system rather than an afterthought.

---

## 12. Practical Guardrails

Create an actionable Do / Don't section.

Example:

### Do

* Reuse existing components.
* Follow semantic tokens.
* Preserve visual hierarchy.
* Use the established spacing system.
* Follow the brand voice.
* Prefer composition over duplication.
* Add new system patterns intentionally.

### Don't

* Invent random colors.
* Introduce arbitrary spacing everywhere.
* Create one-off button styles.
* Add decorative animation without purpose.
* Introduce unrelated visual trends.
* Create nested cards without a clear hierarchy.
* Replace established patterns without justification.

---

## 13. Design Decision Record

Document important decisions and their rationale.

Example:

```text
Decision:
Use restrained motion.

Rationale:
The product personality emphasizes calmness and focus.

Source:
Brand philosophy.

Confidence:
Explicit.
```

This section helps future AI sessions understand not only **what** the rule is, but **why** it exists.

---

# Step 10 — Validate the Generated System

Before finalizing the files, perform a self-audit.

## Brand Validation

Check:

* Is the product identity coherent?
* Are personality traits clear?
* Are anti-patterns defined?
* Does the voice match the personality?
* Are assumptions clearly distinguished from explicit decisions?

---

## Design Validation

Check:

* Are semantic colors defined?
* Is typography defined?
* Is spacing defined?
* Is shape/elevation defined?
* Are responsive rules defined?
* Are motion principles defined?
* Are accessibility principles defined?
* Are component principles present?
* Are composition rules present?

---

## Stack Validation

Check:

* Does `DESIGN.md` match the actual framework?
* Does it match the styling system?
* Does it match the installed component library?
* Does it reference libraries that do not exist?
* Does it recommend patterns incompatible with the framework?
* Are the token implementation rules compatible with the project's actual tooling?

---

## Consistency Validation

Check that:

```text
BRAND_PHILOSOPHY.md
        ↓
DESIGN.md
```

does not contradict itself.

For example, if the brand says:

```text
Calm
Restrained
Focused
```

but the design system recommends:

```text
Heavy gradients
Constant motion
Large decorative illustrations
Highly saturated colors
```

the system is inconsistent and must be corrected.

---

# Step 11 — Update `AGENTS.md`

Locate `AGENTS.md` in the project root.

If it does not exist, create it.

If it exists, **never overwrite unrelated instructions**.

Add or update only the managed section:

```markdown
<!-- BEGIN:design-and-brand-rules -->

# Design & Brand System Rules

**CRITICAL RULE: Never invent a visual language independently when project design specifications exist.**

Before creating, modifying, or refactoring any UI component, page, layout, interaction, or user-facing copy, AI agents MUST inspect:

1. [BRAND_PHILOSOPHY.md](./BRAND_PHILOSOPHY.md) — voice, tone, personality, emotional goals, and brand identity.
2. [DESIGN.md](./DESIGN.md) — design tokens, typography, spacing, layout, component principles, motion, accessibility, and implementation rules.

## Implementation Rules

- **Token Compliance:** Use existing semantic design tokens instead of hardcoded colors or arbitrary values.
- **Component Reuse:** Prefer composing or extending existing primitives before creating new components.
- **Spacing Ownership:** Components control internal spacing. Parent layouts control positioning and relationships.
- **Brand Compliance:** User-facing copy must follow `BRAND_PHILOSOPHY.md`.
- **Visual Consistency:** Do not introduce unrelated colors, typography, radius styles, shadows, or visual patterns.
- **Motion Discipline:** Animation must follow the motion principles defined in `DESIGN.md`.
- **Accessibility:** Preserve the accessibility requirements defined by the design system.
- **System Evolution:** When a new reusable design pattern is required, derive it from the existing system and document significant system-level additions.
- **Conflict Handling:** Do not silently override established design rules. Resolve significant conflicts with the user.

## Before Completing UI Work

Verify:

- Existing design tokens were reused.
- Existing components were reused where appropriate.
- New visual patterns are consistent with `DESIGN.md`.
- Copy follows `BRAND_PHILOSOPHY.md`.
- Responsive behavior follows the established principles.
- Accessibility requirements are satisfied.

<!-- END:design-and-brand-rules -->
```

If this managed section already exists, replace only that section.

Do not duplicate it.

---

# Step 12 — Final Verification

Before completing the task, verify:

### Files

```text
BRAND_PHILOSOPHY.md
DESIGN.md
AGENTS.md
```

are present as appropriate.

---

### Brand Philosophy

Verify that it articulates:

* mission
* personality
* voice
* emotional direction
* references
* anti-tropes
* brand guardrails

---

### Design System

Verify that it documents:

* technology stack
* design philosophy
* color system
* typography
* spacing
* shape/elevation
* component principles
* composition rules
* responsive behavior
* motion
* accessibility
* practical guardrails
* design decisions

---

### Agent Enforcement

Verify that `AGENTS.md` explicitly instructs future AI sessions to inspect and follow:

```text
BRAND_PHILOSOPHY.md
DESIGN.md
```

---

### Technology Alignment

Verify that all generated rules reflect the project's actual technology stack.

Do not document technologies that are not actually being used as though they are part of the system.

---

# Important Behavioral Rules

## Do Not Generate a Generic Design System

Never produce a generic:

```text
Modern SaaS
Blue primary
12px radius
Inter
Purple gradients
```

system without considering the actual project.

The system must emerge from:

```text
Existing project
+
User intent
+
Brand philosophy
+
Technology constraints
+
Design-system best practices
```

---

## Do Not Overdesign

A design system should create consistency, not visual complexity.

Avoid introducing:

* unnecessary gradients
* excessive shadows
* excessive border radius
* decorative animations
* arbitrary illustrations
* unnecessary badges
* nested cards
* excessive glassmorphism
* visual noise

unless they are justified by the product's brand philosophy.

---

## Do Not Over-Question

If the project already establishes:

```text
Satoshi
Geist
Lime primary
Tailwind
shadcn
```

do not ask the user to choose those again.

Ask about what is genuinely unknown.

---

## Do Not Treat Inference as Fact

If a design decision is inferred, make that distinction clear.

Do not write:

```text
The brand requires restrained motion.
```

when the user never said that.

Instead:

```text
The system recommends restrained motion based on the
product's calm and focused personality.
```

---

## Do Not Destroy Existing Work

If an established design system already exists:

1. inspect it
2. understand it
3. document it
4. identify inconsistencies
5. propose improvements when necessary

Do not replace it simply because the skill has its own preferred structure.

---

## Do Not Create Unnecessary Dependencies

A design-system skill should not introduce new packages merely to implement design decisions.

Use the project's existing ecosystem whenever possible.

---

# Desired Output

The final result should leave the project with a coherent chain:

```text
                         USER INTENT
                              │
                              ↓
                    BRAND_PHILOSOPHY.md
                         "WHY"
                              │
                              ↓
                         DESIGN.md
                         "HOW"
                              │
                              ↓
                         AGENTS.md
                        "ENFORCEMENT"
                              │
                              ↓
                    Future AI Sessions
                              │
                              ↓
                    Consistent Product UI
```

The ultimate success criterion is not whether the generated Markdown looks impressive.

The success criterion is:

> **Can a different AI session, weeks later, read these files and produce a new feature that feels like it belongs to the same product without needing the original design conversation?**

If the answer is yes, the skill has succeeded.
