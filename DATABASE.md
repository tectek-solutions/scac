# Database

## Relational vs. Non-Relational Databases

When choosing a database for a project, understanding the difference between relational and non-relational databases is key.  

**Relational Databases** store data in structured tables with predefined schemas and use SQL for data manipulation. They are ideal for projects with predictable data, clear relationships, and strict consistency needs (e.g., banking).  

- **Advantages**: ACID compliance ensures reliability, data accuracy, simplicity, and normalization reduces redundancy.  
- **Disadvantages**: Limited scalability (vertical only), rigid schema, and slower performance with complex queries.  

**Non-Relational Databases** (NoSQL) store data flexibly, often in JSON-like documents, key-value pairs, graphs, or wide columns. They excel in handling unstructured or big data and are optimized for horizontal scaling, making them suitable for modern, cloud-based applications.  

- **Advantages**: High scalability, flexibility, and performance, suitable for dynamic or massive datasets.  
- **Disadvantages**: Less standardization and reliability (e.g., ACID compliance).  

**Use Cases**:  

- **Relational Databases**: Structured, predictable data with complex relationships.  
- **Non-Relational Databases**: Unstructured, flexible data or applications requiring high scalability and availability.  

**Key Features Comparison**:  

| Feature                | Non-Relational        | Relational              |  
|------------------------|-----------------------|--------------------------|  
| Availability           | High                 | High                    |  
| Horizontal Scaling     | High                 | Low                     |  
| Performance            | High                 | Low to Medium           |  
| Flexibility            | High                 | Low (Strict Schema)     |  
| Reliability            | Medium               | High (ACID Compliance)  |  

**Conclusion**: Relational databases offer structure and reliability for traditional use cases, while non-relational databases provide flexibility and scalability for modern, dynamic applications. For example, **MongoDB**, a popular non-relational database, excels in performance, scalability, and flexibility, especially when paired with its cloud platform, MongoDB Atlas.
