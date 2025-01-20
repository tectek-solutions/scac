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
  static const clientUrl = String.fromEnvironment('CLIENT_URL', defaultValue: 'http://localhost:3000');
  List<dynamic> services = [];
  bool _isLoading = true;
  bool _hasError = false;
  bool _isDownloadButtonHovered = false; // Booléen pour l'effet de survol du bouton de téléchargement

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
          child: const Text('My Workflows'),
        ),
        backgroundColor: Colors.teal,
        automaticallyImplyLeading: false,
        actions: [
          // Utilisation de MouseRegion pour détecter le survol du bouton de téléchargement
          Padding(
            padding: const EdgeInsets.only(right: 16.0),
            child: MouseRegion(
              onEnter: (_) {
                setState(() {
                  _isDownloadButtonHovered = true; // Survol actif
                });
              },
              onExit: (_) {
                setState(() {
                  _isDownloadButtonHovered = false; // Survol terminé
                });
              },
              child: IconButton(
                onPressed: () {
                  void downloadFile() async {
                    try {
                      final response = await apiService.downloadFile('${clientUrl}/client.apk');
                      if (response.statusCode == 200) {
                        print('File downloaded successfully');
                      } else {
                        print('Failed to download file');
                      }
                    } catch (e) {
                      print('Error downloading file: $e');
                    }
                  }

                  downloadFile();
                },
                icon: Icon(Icons.download),
                color: _isDownloadButtonHovered ? Colors.blue : Colors.white, // Changer la couleur du bouton au survol
                iconSize: _isDownloadButtonHovered ? 30.0 : 24.0, // Modifier la taille du bouton au survol
              ),
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
                                      Navigator.push(
                                        context,
                                        MaterialPageRoute(
                                          builder: (context) => IntermediatePageWorkflow(itemIndex: index, id: service['id']),
                                        ),
                                      );
                                    },
                                  ),
                                );
                              },
                            ),
                          )
                        : const Center(child: Text('No services available')),
            // Affichage d'une description sous le bouton de téléchargement lors du survol
            if (_isDownloadButtonHovered)
              Padding(
                padding: const EdgeInsets.only(top: 8.0),
                child: Text(
                  'Click to download the client APK.',
                  style: TextStyle(color: Colors.grey),
                ),
              ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: clearAppCache,
        backgroundColor: Colors.teal,
        child: const Icon(Icons.refresh),
        tooltip: "Clear cache",
      ),
    );
  }
}
