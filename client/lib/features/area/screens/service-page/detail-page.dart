import 'package:client/features/services/api.area.service.dart';
import 'package:client/features/area/screens/service-page/intermediate-page.dart';
import 'package:flutter/material.dart';

class DetailPage extends StatefulWidget {
  final int itemIndex;
  int id;
  final dynamic card;

  DetailPage({required this.itemIndex, required this.id, required this.card, super.key});

  @override
  State<DetailPage> createState() => _DetailPageState();
}

class _DetailPageState extends State<DetailPage> {

  late final ApiService apiService;
  List<Map<String, String>> actions = [];

  @override
  void initState() {
    super.initState();
    apiService = ApiService(
    baseUrl: IntermediatePage.baseUrlString,
    route: '/actions/apis/${widget.id}'
    );
    apiService.fetchCards().then((value) {
      print("OK");
      print("Passed value: $value");
      setState(() {
        for (var i = 0; i < value.length; i++) {
          actions.add({
            'value': value[i]['name'],
            ...value[i]['data_keys'],
          });
        }
        print("Actions: $actions");
      });
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Detail Page'),
      ),
      body: actions.isEmpty
          ? const Center(
              child: CircularProgressIndicator(),
            )
          : ListView(
              children: actions.map<Widget>((action) {
                var actionIndex = actions.indexOf(action);
                print("Action: $action");
                print("Widget Index: ${widget.itemIndex}");
                print("Widget Card: ${widget.card}");
                print("Widget ID: ${widget.id}");
                return Padding(
                  padding: const EdgeInsets.all(16.0),
                  child: Card(
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(15.0),
                    ),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      mainAxisSize: MainAxisSize.min,
                      children: [
                        // Title Section
                        Padding(
                          padding: const EdgeInsets.all(16.0),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                widget.itemIndex < actions.length &&
                                        actions[widget.itemIndex]['type'] == 'name'
                                    ? actions[widget.itemIndex]['value'] as String
                                    : 'Invalid Item',
                                style: const TextStyle(
                                  fontSize: 22.0,
                                  fontWeight: FontWeight.bold,
                                ),
                              ),
                              const SizedBox(height: 10.0),
                              const Text(
                                'No Description',
                                style: TextStyle(fontSize: 16.0),
                              ),
                            ],
                          ),
                        ),
                        const Divider(),
                        Padding(
                          padding: const EdgeInsets.all(16.0),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              ElevatedButton(
                                onPressed: () {
                                  int count = 0;
                                  Navigator.of(context).popUntil((route) {
                                    count++;
                                    if (count == 3) {
                                      Navigator.pop(context, {
                                        'action': actions,
                                        'index': actionIndex,
                                      });
                                      return true;
                                    }
                                    return false;
                                  });
                                },
                                child: Text(action['value'] ?? 'No Value'),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                  ),
                );
              }).toList(),
            ),
    );
  }
}
