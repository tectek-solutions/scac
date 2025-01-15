import 'package:client/utils/constants/sizes.dart';
import 'package:flutter/material.dart';
import '../../../services/api.area.service.dart';

class ClickableCardScreen extends StatefulWidget {
  @override
  _ClickableCardScreenState createState() => _ClickableCardScreenState();
}

class _ClickableCardScreenState extends State<ClickableCardScreen> {
  bool _showDetail = false;
  static const baseUrlString = String.fromEnvironment('API_URL', defaultValue: 'http://localhost:8000');
  ApiService apiService = ApiService(baseUrl: baseUrlString, route: '/workflows/');
  List<dynamic> services = [];
  bool _isLoading = true;
  bool _hasError = false;

  @override
  void initState() {
    super.initState();
    _fetchServices();
  }

  Future<void> _fetchServices() async {
    try {
      final value = await apiService.fetchCards();
      setState(() {
        services = value is List ? value : [value];
        _showDetail = true;
        _isLoading = false;
      });
    } catch (e) {
      setState(() {
        _hasError = true;
        _isLoading = false;
      });
      print('Error fetching cards: $e');
    }
  }

  Future<void> _removeCard(int id) async {
    try {
      await apiService.removeCard(id);
      setState(() {
        services.removeWhere((service) => service['id'] == id);
      });
    } catch (e) {
      print('Error removing card: $e');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Padding(
          padding: const EdgeInsets.all(16.0),
          child: const Text('My Workflows'),
        ),
        backgroundColor: Colors.teal,
        automaticallyImplyLeading: false,
        actions: [
          Padding(
            padding: const EdgeInsets.only(right: 16.0),
            child: IconButton(
              onPressed: () {
              },
              icon: Icon(Icons.download),
            ),
          ),
        ],
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          children: [
            _isLoading
                ? const Center(child: CircularProgressIndicator())
                : _hasError
                    ? const Center(child: Text('Error loading services', style: TextStyle(color: Colors.red)))
                    : _showDetail
                        ? Expanded(
                            child: ListView.builder(
                              itemCount: services.length,
                              itemBuilder: (context, index) {
                                final service = services[index];
                                return Card(
                                  elevation: 4.0,
                                  shape: RoundedRectangleBorder(
                                    borderRadius: BorderRadius.circular(12.0),
                                  ),
                                  child: ListTile(
                                    leading: CircleAvatar(
                                      backgroundColor: Colors.teal,
                                      child: Text(service['name'][0].toUpperCase(), style: TextStyle(color: Colors.white)),
                                    ),
                                    title: Text(service['name'], style: TextStyle(fontWeight: FontWeight.bold)),
                                    subtitle: Text(service['description']),
                                    trailing: IconButton(
                                      icon: Icon(Icons.delete, color: Colors.red),
                                      onPressed: () async {
                                        await _removeCard(service['id']);
                                      },
                                    ),
                                    onTap: () {
                                      // Handle card tap
                                    },
                                  ),
                                );
                              },
                            ),
                          )
                        : const Center(child: Text('No services available')),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _fetchServices,
        backgroundColor: Colors.teal,
        child: const Icon(Icons.refresh),
      ),
    );
  }
}