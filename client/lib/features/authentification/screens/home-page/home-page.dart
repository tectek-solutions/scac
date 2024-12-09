// import 'package:flutter/material.dart';

// class HomePage extends StatefulWidget {
//   const HomePage({ Key? key }) : super(key: key);

//   @override
//   _HomePageState createState() => _HomePageState();
// }

// class _HomePageState extends State<HomePage> {
//   @override
//   Widget build(BuildContext context) {
//     return MaterialApp(
//       home: Scaffold(
//         appBar: AppBar(
//           title: const Text('Home Page'),
//         ),
//         body: ListView(
//           children: <Widget>[
//             const Text("Let's get started"),
//             InkWell(
//               onTap: () {
//                 showDialog(
//                   context: context,
//                   builder: (BuildContext context) {
//                     return AlertDialog(
//                       shape: RoundedRectangleBorder(
//                         borderRadius: BorderRadius.circular(15.0),
//                       ),
//                       content: const NewPage(),
//                     );
//                   },
//                 );
//               },
//               child: Card(
//                 shape: RoundedRectangleBorder(
//                   borderRadius: BorderRadius.circular(15.0),
//                 ),
//                 elevation: 5,
//                 child: const ListTile(
//                   title: Text('Welcome to the Home Page'),
//                 ),
//               ),
//             ),
//           ],
//         ),
//       ),
//     );
//   }
// }

// class NewPage extends StatelessWidget {
//   const NewPage({ Key? key }) : super(key: key);

//   @override
//   Widget build(BuildContext context) {
//     return const Center(
//       child: Text('This is the new page'),
//     );
//   }
// }

import 'package:client/utils/constants/sizes.dart';
import 'package:flutter/material.dart';

class ClickableCardScreen extends StatefulWidget {
  @override
  _ClickableCardScreenState createState() => _ClickableCardScreenState();
}

class _ClickableCardScreenState extends State<ClickableCardScreen> {
  bool _showDetail = false; // State to manage detail section visibility

  void toggleDetail() {
    setState(() {
      _showDetail = !_showDetail;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Stack(
        children: [
          Padding(
            padding: const EdgeInsets.only(top: TSizes.appBarHeight, left: 16.0, right: 16.0),
            child: Column(
              children: [
                GestureDetector(
                  onTap: toggleDetail,
                  child: Card(
                    elevation: 4.0,
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(12.0),
                    ),
                    child: Padding(
                      padding: const EdgeInsets.all(16.0),
                      child: Row(
                        crossAxisAlignment: CrossAxisAlignment.center,
                        children: [
                          SizedBox(width: 16.0),
                          Expanded(
                            child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              children: [
                                Text(
                                  'Automation Trigger',
                                  style: TextStyle(
                                    fontSize: 18.0,
                                    fontWeight: FontWeight.bold,
                                  ),
                                ),
                                SizedBox(height: 8.0),
                                Text(
                                  'Perform an action when a condition is met.',
                                  style: TextStyle(
                                      fontSize: 14.0, color: Colors.grey[700]),
                                ),
                              ],
                            ),
                          ),
                          Icon(
                            _showDetail
                                ? Icons.keyboard_arrow_left
                                : Icons.keyboard_arrow_right,
                            size: 24.0,
                            color: Colors.grey,
                          ),
                        ],
                      ),
                    ),
                  ),
                ),
              ],
            ),
          ),

          // Animated Detail Section
          AnimatedPositioned(
            duration: Duration(milliseconds: 300), // Animation duration
            curve: Curves.easeInOut, // Animation curve
            right: _showDetail ? 0 : -MediaQuery.of(context).size.width,
            top: 0,
            bottom: 0,
            child: Padding(
              padding: const EdgeInsets.only(top: TSizes.appBarHeight),
              child: Material(
                elevation: 8.0,
                child: Container(
                  color: Colors.white,
                  width: MediaQuery.of(context).size.width,
                  height: MediaQuery.of(context).size.height,
                  padding: const EdgeInsets.all(16.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Row(
                        children: [
                          IconButton(
                            icon: const Icon(Icons.arrow_back),
                            onPressed: toggleDetail,
                          ),
                          const SizedBox(width: 16.0),
                          const Text(
                            'Card Details',
                            style: TextStyle(
                              fontSize: 20.0,
                              fontWeight: FontWeight.bold,
                            ),
                          ),
                        ],
                      ),
                      const SizedBox(height: 16.0),
                      Text(
                        'This is the detailed section of the card. It slides in from the right.',
                        style: TextStyle(fontSize: 16.0, color: Colors.grey[700]),
                      ),
                      const Spacer(),
                    ],
                  ),
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }
}
