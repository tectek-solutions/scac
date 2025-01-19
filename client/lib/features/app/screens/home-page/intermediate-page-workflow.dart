import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:client/features/services/api.service.dart';
import 'package:client/features/services/api.area.service.dart';

class IntermediatePageWorkflow extends StatefulWidget {
  static const baseUrlString = String.fromEnvironment('API_URL', defaultValue: 'http://localhost:8000');
  final int itemIndex;
  final int id;

  IntermediatePageWorkflow({required this.itemIndex, required this.id, super.key});

  @override
  State<IntermediatePageWorkflow> createState() => _IntermediatePageWorkflow();
}

class _IntermediatePageWorkflow extends State<IntermediatePageWorkflow> {
  late final ApiService apiService;
  late final ApiService apiServiceWorkflow;

  List<dynamic> cards = [];

  @override
  void initState() {
    super.initState();
    apiService = ApiService(baseUrl: IntermediatePageWorkflow.baseUrlString, route: '/workflows/${widget.id}');
    apiService.fetchCards().then((value) {
      if (value is Map<String, dynamic>) {
        value = [value];
      }
      setState(() {
        for (var i = 0; i < value.length; i++) {
          cards.add({'title': value[i]['name'], 'description': value[i]['description']});
        }
      });
    });

    apiServiceWorkflow = ApiService(baseUrl: IntermediatePageWorkflow.baseUrlString, route: '/triggers/workflows/${widget.id}');
    apiServiceWorkflow.fetchCards().then((value) {
      if (value is Map<String, dynamic>) {
        value = [value];
      }
      setState(() {
        for (var i = 0; i < value.length; i++) {
          cards.add({'status': value[i]['status'], 'id': value[i]['id']});
        }
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Workflow Details'),
        backgroundColor: Colors.teal,
        elevation: 0,
      ),
      body: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16.0, vertical: 8.0),
        child: cards.isEmpty
            ? const Center(child: CircularProgressIndicator())
            : ListView.builder(
                itemCount: cards.length,
                itemBuilder: (context, index) {
                  return Padding(
                    padding: const EdgeInsets.only(bottom: 8.0),
                    child: Card(
                      elevation: 4.0,
                      shape: RoundedRectangleBorder(
                        borderRadius: BorderRadius.circular(12.0),
                      ),
                      color: Colors.white,
                      child: ListTile(
                        contentPadding: const EdgeInsets.all(16.0),
                        title: Text(
                          cards[index]['title'] ?? 'No title available',
                          style: const TextStyle(
                            fontWeight: FontWeight.bold,
                            fontSize: 18.0,
                            color: Colors.teal,
                          ),
                        ),
                        subtitle: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text(
                              cards[index]['description'] ?? 'No description available',
                              style: const TextStyle(fontSize: 14.0, color: Colors.black54),
                            ),
                            const SizedBox(height: 8.0),
                            Row(
                              children: [
                                Icon(
                                  Icons.work,
                                  size: 16.0,
                                  color: cards[index]['status'] == 'active' ? Colors.green : Colors.red,
                                ),
                                const SizedBox(width: 8.0),
                                Text(
                                  'Status: ${cards[index]['status'] ?? 'Unknown'}',
                                  style: const TextStyle(fontSize: 12.0, color: Colors.black54),
                                ),
                                const SizedBox(width: 16.0),
                                Text(
                                  'ID: ${cards[index]['id']}',
                                  style: const TextStyle(fontSize: 12.0, color: Colors.black54),
                                ),
                              ],
                            ),
                          ],
                        ),
                      ),
                    ),
                  );
                },
              ),
      ),
    );
  }
}
