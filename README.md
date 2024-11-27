# SCAC 

## For developers, by developers.

[FEATURE_DEVELOPMENT_PROCESS.md](FEATURE_DEVELOPMENT_PROCESS.md) | [PROJECT_STRUCTURE.md](PROJECT_STRUCTURE.md) | [PLANNING.md](PLANNING.md) | 

## Tech Stack Choices

### Frontend and Mobile

- **Framework**: Flutter  
  - Chosen for its cross-platform capabilities, enabling the creation of high-performance web and mobile applications with a single codebase.  
  - Benchmarked against alternatives like React Native and Xamarin for performance and ease of use, Flutter demonstrated superior UI consistency and lower latency.  
- **UI Framework**: Material Design 3  
  - Reason: Offers modern, standardized design principles, and integrates naturally with Flutter.

### Backend

View the [API.md](API.md) file for more information.

- **Language**: Rust  
  - Rust was selected for its memory safety, performance, and modern concurrency model. Benchmarks consistently show Rust-based frameworks outperforming alternatives in speed and resource efficiency.  
- **Libraries**:  
  - **Actix** (web server): Recognized for its exceptional speed and asynchronous capabilities.  
  - **reqwest** (HTTP client): Reliable and flexible for external API interactions.  
  - **Diesel** (ORM): Enables efficient, type-safe database queries.  
  - **Serde** (serialization): Offers fast and secure data serialization.  

- **APIs**: REST-based architecture ensures scalability and platform independence.

### Database

View the [DATABASE.md](DATABASE.md) file for more information.
Also the schema can be found in the [SCHEMA.md](SCHEMA.md) file.

- **Relational**: PostgreSQL  
  - Selected for its robust ACID compliance and extensive support for complex queries.  
- **Non-relational**: PostgreSQL JSONB  
  - Offers the flexibility of NoSQL with the reliability of SQL, ideal for dynamic data structures.

---

## Deployment

- **Containerization**: Docker  
  - Simplifies deployment by encapsulating the application and its dependencies.  
- **Orchestration**: Kubernetes  
  - Facilitates scaling and management of containerized applications.  
- **Package Management**: Helm  
  - Streamlines deployment processes by managing Kubernetes configurations.

---

## Monitoring

- **Elastic Stack**: Offers comprehensive monitoring, logging, and visualization for better system observability.

---

## CI/CD

- **Integration**: GitHub Actions  
  - Automated testing and building pipeline simplifies development cycles.  
- **Deployment**: ArgoCD  
  - Ensures continuous delivery with version control synchronization.

---

## Documentation

- **API Documentation**: Swagger  
  - Provides interactive, auto-generated documentation for developers.

---

## Testing

- **Frameworks**:  
  - **Rust**: Ensures backend reliability.  
  - **Flutter**: Verifies UI and logic for mobile and web.  
  - **Actix**: Tests web server performance and reliability.

---

## Benchmark References

To ensure the best performance for each part of the stack, benchmarking sources were consulted:  

- **Backend**: [TechEmpower Web Framework Benchmarks](https://www.techempower.com/benchmarks/#hw=ph&test=fortune&section=data-r22) evaluates web frameworks in speed, scalability, and efficiency.  
- **Frontend**: [JS Framework Benchmark](https://krausest.github.io/js-framework-benchmark/current.html) compares JavaScript frameworks for rendering speed and performance.  
- **Mobile**: [MobiDev Cross-Platform Framework Comparison](https://media.mobidev.biz/2024/08/comparison-of-the-best-cross-platform-app-development-frameworks-by-mobidev.pdf) assesses mobile development frameworks for usability and platform reach.  

These sources helped identify robust, high-performing technologies for each layer.

---

### Conclusion

The stack and tools were selected based on performance benchmarks, developer familiarity, and alignment with project goals, ensuring efficient development, maintainability, and scalability. A PoC validated these choices with successful integrations and deployments in the target environment.
