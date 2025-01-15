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

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Clickable Card Screen'),
      ),
      body: Padding(
        padding: const EdgeInsets.only(top: TSizes.appBarHeight, left: 16.0, right: 16.0),
        child: Column(
          children: [
            _isLoading
                ? const CircularProgressIndicator()
                : _hasError
                    ? const Text('Error loading services')
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
                                    title: Text(service['name']),
                                    subtitle: Text(service['description']),
                                  ),
                                );
                              },
                            ),
                          )
                        : const Text('No services available'),
          ],
        ),
      ),
    );
  }
}