# Data is pulled from https://covid.ourworldindata.org/data/owid-covid-data.json
import matplotlib.pyplot as plt
import matplotlib.ticker
import json

countries = ["CHN", "USA", "RUS", "JPN", "DEU", "IND", "OWID_WRL"]

if __name__ == "__main__":
    fig, ax = plt.subplots(figsize=(14, 8))

    with open("../rust/plotters_ex/owid-covid-data.json") as f:
        data = json.load(f)
        for c in countries:
            nc = []
            tc = []
            for row in data[c]['data']:
                nc.append(row.get('new_cases', 0))
                tc.append(row.get('total_cases', 0))
            ax.plot(tc, nc, label=c)

    ax.set_xscale('log')
    ax.set_xticks([50, 100, 1000, 10000, 100000, 1000000, 10000000])
    non_sci = matplotlib.ticker.ScalarFormatter()
    non_sci.set_scientific(False)
    ax.get_xaxis().set_major_formatter(non_sci)
    ax.set_yscale('log')
    ax.set_yticks([10, 50, 100, 1000, 10000, 100000, 200000])
    ax.get_yaxis().set_major_formatter(matplotlib.ticker.ScalarFormatter())

    ax.set_title('World COVID-19 Cases')
    ax.set_xlabel('total cases')
    ax.set_ylabel('new cases')
    ax.grid(True)

    text = 'Data Source: https://covid.ourworldindata.org/data/owid-covid-data.json'
    plt.figtext(0.5, 0, text, fontsize=8, va="bottom", ha="center")

    box = ax.get_position()
    ax.set_position([box.x0, box.y0, box.width * 0.7, box.height])
    legend = ax.legend(title="Countries",
                       shadow=False,
                       fontsize='x-large',
                       loc='center left',
                       bbox_to_anchor=(1.05, 0.5))

    plt.savefig("test.svg")
    #plt.show()
