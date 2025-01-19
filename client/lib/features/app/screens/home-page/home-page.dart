import 'package:flutter/material.dart';
import '../../../services/api.area.service.dart';
import 'package:flutter_cache_manager/flutter_cache_manager.dart';
import './intermediate-page-workflow.dart';

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

  Future<void> clearAppCache() async {
    await DefaultCacheManager().emptyCache();
    print("App cache cleared");
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Text('My Workflows', style: TextStyle(fontWeight: FontWeight.bold, fontSize: 24)),
        ),
        backgroundColor: Colors.teal[600],
        automaticallyImplyLeading: false,
        actions: [
          Padding(
            padding: const EdgeInsets.only(right: 16.0),
            child: IconButton(
              onPressed: () {
              },
              icon: Icon(Icons.download),
              tooltip: 'Download Data',
            ),
          ),
        ],
      ),
      body: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16.0, vertical: 8.0),
        child: Column(
          children: [
            _isLoading
                ? Center(child: CircularProgressIndicator())
                : _hasError
                    ? _buildErrorState()
                    : _showDetail
                        ? Expanded(child: _buildServiceList())
                        : Center(child: Text('No services available', style: TextStyle(color: Colors.grey))),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: clearAppCache,
        backgroundColor: Colors.teal[600],
        child: Icon(Icons.refresh),
        tooltip: "Clear cache",
      ),
    );
  }

  Widget _buildErrorState() {
    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(Icons.error_outline, color: Colors.red, size: 40),
          SizedBox(height: 8),
          Text('Error loading services', style: TextStyle(color: Colors.red, fontSize: 16, fontWeight: FontWeight.bold)),
          SizedBox(height: 16),
          ElevatedButton(
            onPressed: _fetchServices,
            style: ElevatedButton.styleFrom(
              backgroundColor: Colors.teal[600],
            ),
            child: Text('Retry', style: TextStyle(color: Colors.white)),
          )
        ],
      ),
    );
  }

  Widget _buildServiceList() {
    return ListView.builder(
      itemCount: services.length,
      itemBuilder: (context, index) {
        final service = services[index];
        return Dismissible(
          key: Key(service['id'].toString()),
          direction: DismissDirection.endToStart,
          onDismissed: (direction) async {
            await _removeCard(service['id']);
            ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('Service removed')));
          },
          background: Container(
            color: Colors.red,
            alignment: Alignment.centerRight,
            padding: EdgeInsets.symmetric(horizontal: 20),
            child: Icon(Icons.delete, color: Colors.white),
          ),
          child: Card(
            elevation: 5.0,
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(16.0),
            ),
            margin: EdgeInsets.symmetric(vertical: 8.0),
            child: ListTile(
              contentPadding: EdgeInsets.all(16),
              leading: CircleAvatar(
                backgroundColor: Colors.teal[400],
                child: Text(service['name'][0].toUpperCase(), style: TextStyle(color: Colors.white, fontWeight: FontWeight.bold)),
              ),
              title: Text(service['name'], style: TextStyle(fontWeight: FontWeight.bold, fontSize: 18)),
              subtitle: Text(service['description'], maxLines: 2, overflow: TextOverflow.ellipsis),
              trailing: Icon(Icons.chevron_right, color: Colors.teal[600]),
              onTap: () {
                Navigator.push(
                  context,
                  MaterialPageRoute(
                    builder: (context) => IntermediatePageWorkflow(itemIndex: index, id: service['id']),
                  ),
                );
              },
            ),
          ),
        );
      },
    );
  }
}
