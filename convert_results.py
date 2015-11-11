import numpy as np
import matplotlib
import matplotlib.pyplot as plt
from operator import itemgetter
import io
import yaml
import os.path
import argparse
import math

# loadResultsTable :: String -> IO (Tree String)
def loadResultsTable(fname):
    if not os.path.isfile(fname):
        raise ValueError('Input file \'{}\' does not exist!'.format(fname))
    file = open(fname, 'r')
    data = yaml.load(file)
    file.close()
    return data

def saveAsCsv(results_table, fname):
    with open(fname, 'w+') as results_file:
        col_names = [
            'instance_name',
            'soln_value',
            'running_time_seconds',
            'time_to_best_solution_seconds',
            'best_known_solution_value',
         ]
        # col_names = sorted(results_table[0].keys())
        header_row = '\t'.join(col_names)

        results_file.write(header_row + '\n')

        def getRow(result):
            row = []
            for col in col_names:
                if col in result.keys():
                    row += [result[col]]
                else:
                    row += [0]
            return row

        for result in results_table:
            # row_raw = map(lambda n: result[n], col_names)
            row_raw = getRow(result)
            row_raw = map(lambda n: '' if n is None else n, row_raw)
            row_str = '\t'.join(map(str, row_raw))
            results_file.write(row_str + '\n')

if __name__ == "__main__":
    # Parse arguments:
    parser = argparse.ArgumentParser(description='Convert results to tsv')
    # parser.add_argument('resultsFile', type=str, nargs='?', default='walkTrials copy.yaml', help='Results file name.')
    # args = parser.parse_args()

    # Load results from file:
    # trialResults = loadResultsTable(args.resultsFile)
    # saveAsCsv(trialResults, 'results.tsv')
    saveAsCsv(loadResultsTable('results_bma.yaml'), 'results_bma.tsv')
    saveAsCsv(loadResultsTable('results_ea.yaml'), 'results_ea.tsv')
    saveAsCsv(loadResultsTable('results_its.yaml'), 'results_its.tsv')
    saveAsCsv(loadResultsTable('results_sa.yaml'), 'results_sa.tsv')
