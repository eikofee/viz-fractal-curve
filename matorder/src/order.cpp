#include <arrayfire.h>
#include <iostream>
#include <string.h>
#include <fstream>
#include <sstream>
#include <vector>
#include "csv.h"
#define NOMINMAX
//using namespace std;
using namespace af;

using std::cout;
using std::string;
using std::vector;
using std::endl;
using std::flush;



struct VatParameters {
    int MAX_SIZE=64;
    int MAT_SIZE=10000;
    int DEBUG=0;
    int READ_BUFFER = 1024 * 1024;
    int MAX_VALUE = 4;
    int IMG_WIDTH = 800;
    int IMG_HEIGHT = 800;
    char* AF_MATRIX_KEY = "arraykey";
};





void read_mat(const VatParameters& params, std::string filename, af::array &matrix_array) {
    io::CSVReader<
        3,  // 3 columns
        io::trim_chars<'\t'>, // just to put something ...
        io::no_quote_escape<' '> // separator is ' '
    > in(filename);

    int x, y;
    float v;

    size_t i = 0;
    while(in.read_row(x, y, v)) {
        ++i;
        if (i % 100000 == 0) {
            std::cout << i << std::endl;
        }
        matrix_array(x, y) = v;
        matrix_array(y, x) = v;
    }


}

void read_p(const VatParameters& params, std::string filename, af::array &p_array) {
    io::CSVReader<
        1,  // 1 column
        io::trim_chars<'\t'>, // just to put something ...
        io::no_quote_escape<' '> // separator is ' '
    > in(filename);

    int x;
    size_t i = 0;
    while(in.read_row(x)) {
        p_array(i) = x;
        ++i;
        if (i % 1000 == 0) {
            std::cout << i << std::endl;
        }
    }
}

void write_mat(const VatParameters & params, string filename, array &p_matrix) {
    std::ofstream outf(filename.c_str());
    float* n = p_matrix.host<float>();
    for (int i = 0; i < params.MAT_SIZE; ++i) {
        int nn = (int) n[i];
        outf << nn << std::endl;
    }
    outf.close();
}

void applyVAT(const VatParameters &params, array matrix_array, array &p_result) {
    array I = constant(0, params.MAT_SIZE);
    array J = constant(1, params.MAT_SIZE);
    p_result = constant(0, params.MAT_SIZE);

    array maxLine;
    array maxValues;
    maxValues = (af::max)(matrix_array, 0);
    if (params.DEBUG == 1) {
        af_print(maxValues);
    }

    array maxValue; //useless
    (af::max)(maxValue, maxLine, maxValues);
    if (params.DEBUG == 1) {
        af_print(maxValue);
        af_print(maxLine);
    }

    array new_arr = matrix_array + identity(params.MAT_SIZE, params.MAT_SIZE) * 99;
    matrix_array = new_arr;
    I(maxLine) = 1;
    J(maxLine) = 0;
    p_result(0) = maxLine;
    array iIndexes1 = range(params.MAT_SIZE);
    int percent = (int) (params.MAT_SIZE / 100);
    for (int i = 1; i < params.MAT_SIZE; ++i) {
        if (percent > 0 && i % percent == 0) {
            std::cout << i / percent << std::endl;
        }
        array iValues1;
        array iIndexes;
        iValues1 = matrix_array(I == 1, span);
        array iValues = iValues1(span, J == 1);
        iIndexes = iIndexes1(J == 1);
        if (params.DEBUG == 1) {
            af_print(iValues);
            af_print(iIndexes);
        }

        array iMinValues = (af::min)(iValues, 0);
        array localMinIndex;
        array localMinValue; //useless
        array iMinIndex;
        (af::min)(localMinValue, localMinIndex, iMinValues);
        iMinIndex = iIndexes(localMinIndex);
        if (params.DEBUG == 1) {
            af_print(iMinValues);
            af_print(localMinValue);
            af_print(localMinIndex);
            af_print(iMinIndex);
            af_print(localMinIndex);
        }

        J(iMinIndex) = 0;
        I(iMinIndex) = 1;
        p_result(i) = iMinIndex;
    }
}

int main(int argc, char* argv[]) {
    VatParameters params;

    std::cout << "matorder starting" << std::endl;
    string mode, input_filename, output_filename, matrix_size, p_matrix_filename;;
    int mode_index, input_filename_index, output_filename_index, matrix_size_index, p_matrix_filename_index;
    int needed_argc = 2;
    int legit_command = 0;
    mode_index = 1;
    input_filename_index = 2;
    output_filename_index = 3;
    p_matrix_filename_index = 3;
    matrix_size_index = 4;

    mode = argv[mode_index];
    if (argc < needed_argc) {
        std::cout << "Please read README" << std::endl;
        return 1;
    }
    size_t is_debug = mode.compare("debug") == 0;
    if (is_debug) {
        ++mode_index;
        ++input_filename_index;
        ++output_filename_index;
        ++p_matrix_filename_index;
        ++matrix_size_index;
    }
        
    mode = argv[mode_index];
    size_t is_load = mode.compare("load") == 0;
    size_t is_read = mode.compare("read") == 0;
    size_t is_convert = mode.compare("convert") == 0;
    size_t is_display = mode.compare("display") == 0;
    size_t is_image = mode.compare("image") == 0;
    if (is_load || is_read || is_convert || is_display) {
        needed_argc = 5;
    }

    if (is_image) {
        needed_argc = 6;
        ++matrix_size_index;
        ++output_filename_index;
    }

    if (argc < needed_argc) {
        std::cout << "Please read README" << std::endl;
        return 1;
    }

    input_filename = argv[input_filename_index];
    output_filename = argv[output_filename_index];
    matrix_size = argv[matrix_size_index];
    p_matrix_filename = argv[p_matrix_filename_index];
    params.MAT_SIZE = atoi(matrix_size.c_str());

    if (is_debug) {
        params.DEBUG = 1;
    }

    array matrix_array = constant(0, params.MAT_SIZE, params.MAT_SIZE);
    if (is_read || is_convert) {
        string binary_filename = output_filename;   
        if (is_read) {
            binary_filename = input_filename + "-af";
        }

        read_mat(params, input_filename, matrix_array);
        saveArray(params.AF_MATRIX_KEY, matrix_array, binary_filename.c_str());
        std::cout << "Matrix has been saved to " << binary_filename << std::endl;
    }

    if (is_load) {
        legit_command = 1;
        matrix_array = readArray(input_filename.c_str(), params.AF_MATRIX_KEY) - identity(params.MAT_SIZE, params.MAT_SIZE) * params.MAX_VALUE;
    }

    array p;
    if (is_read || is_load) {
        applyVAT(params, matrix_array, p);
        write_mat(params, output_filename, p);
    }
    if (is_display || is_image) {
        const static int width = params.IMG_WIDTH, height = params.IMG_HEIGHT;
        matrix_array = readArray(input_filename.c_str(), "arraykey");

        // since AF matrix rendering works with values between 0. and 1., we normalize the values to avoid color saturation
        float* localMax = ((af::max)((af::max)(matrix_array))).host<float>();
        float coef = 1 / localMax[0];
        matrix_array *= coef;
        
        p = constant(0, params.MAT_SIZE);
        read_p(params, p_matrix_filename, p);
        array intMatrix = matrix_array(span, p);
        array fullMatrix = intMatrix(p, span);

        if (is_display) {
            af::Window window(width, height, "Ordered Distance Matrix Preview (800x800)");
            do{
                af::timer delay = timer::start();
                window.image(fullMatrix);
                double fps = 5;
                while (timer::stop(delay) < (1 / fps)) {}
            } while( !window.close() );
        }

        if (is_image) {
            fullMatrix = resize(0.08, 0.08, fullMatrix);
            saveImage(output_filename.c_str(), fullMatrix);
        }
    }

    std::cout << "Done" << std::endl;
    return 0;
}
