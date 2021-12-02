
testexe = executable(
	'testexe', # test executable name 
	'liblength_test.cpp', # tests source files to be compiled
	include_directories : inc,  # declared include directories in root :code:`meson.build`
	link_with : liblength) # link test executable with previously declared shared library

# test execution 
test('Liblength test', testexe)