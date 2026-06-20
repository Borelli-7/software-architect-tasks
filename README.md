# Software Architect Tasks

> A curated monorepo housing all my software architecture work — design patterns, system design exercises, architectural decision records, reference implementations, and more.

---

## 🎯 Purpose

This repository serves as the **central hub** for all my software architect-related projects and studies. It is intentionally structured as a collection of sub-repositories (either as subdirectories or Git submodules), each focused on a specific topic or domain within software architecture.

My goal is to document, explore, and share the knowledge I accumulate throughout my journey as a software architect, covering areas such as:

- **System Design** — scalable, resilient, and maintainable system blueprints
- **Design Patterns** — classic and modern patterns applied in real scenarios
- **Architectural Styles** — microservices, event-driven, hexagonal, CQRS, and more
- **Architectural Decision Records (ADRs)** — rationale behind key design choices
- **Reference Implementations** — working examples that illustrate architectural concepts
- **Best Practices & Guidelines** — coding standards, API design, security, and observability

---

## 📁 Repository Structure

Each sub-repository lives in its own directory (or as a Git submodule) and contains its own `README.md` explaining its scope, goals, and usage.

```
software-architect-tasks/
├── README.md               ← you are here
├── LICENSE
└── <topic-or-project>/     ← individual sub-repositories or directories
    ├── README.md
    └── ...
```

> Sub-repositories will be added progressively as topics are explored and projects mature.

---

## 🚀 Getting Started

Clone the repository (including all submodules if present):

```bash
git clone --recurse-submodules https://github.com/Borelli-7/software-architect-tasks.git
```

If you already cloned without submodules:

```bash
git submodule update --init --recursive
```

---

## 🤝 Contributing

This is a personal knowledge repository, but suggestions, issues, and discussions are welcome. Feel free to open an issue if you spot something worth improving or want to propose a topic.

---

## 📄 License

This project is licensed under the terms of the [LICENSE](./LICENSE) file included in this repository.
