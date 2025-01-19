import 'package:client/features/services/api.area.service.dart';
import 'package:client/features/area/screens/reaction-page/intermediate-page-reaction.dart';
import 'package:flutter/material.dart';

class DetailPage extends StatefulWidget {
  final int itemIndex;
  final int id;
  final dynamic card;
  final dynamic action;

  DetailPage({required this.itemIndex, required this.id, required this.card, required this.action, super.key});

  @override
  State<DetailPage> createState() => _DetailPageState();
}

class _DetailPageState extends State<DetailPage> {
  late final ApiService apiService;
  List<Map<String, String>> reactions = [];

  @override
  void initState() {
    super.initState();
    apiService = ApiService(
      baseUrl: IntermediatePageReaction.baseUrlString,
      route: '/reactions/apis/${widget.id}',
    );
    apiService.fetchCards().then((value) {
      if (value is Map<String, dynamic>) {
        value = [value];
      }
      setState(() {
        for (var i = 0; i < value.length; i++) {
          reactions.add({
            'value': value[i]['name'],
            ...value[i]['data_keys'],
          });
        }
      });
    });
  }

@override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Detail Page'),
      ),
      body: reactions.isEmpty
          ? const Center(
              child: CircularProgressIndicator(),
            )
          : ListView(
              children: reactions.map<Widget>((reaction) {
                var reactionIndex = reactions.indexOf(reaction);
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
                                widget.itemIndex < reactions.length
                                    ? reactions[widget.itemIndex]['value'] as String
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
                                        'action': widget.action,
                                        'reaction': reactions,
                                        'index': reactionIndex,
                                      });
                                      return true;
                                    }
                                    return false;
                                  });
                                },
                                child: Text(reaction['value'] ?? 'No Value'),
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
