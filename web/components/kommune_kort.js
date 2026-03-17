import { createTooltip } from './tooltip.js';

const NS = 'http://www.w3.org/2000/svg';
const FMT = new Intl.NumberFormat('da-DK', { maximumFractionDigits: 0 });
const FMT_KR = new Intl.NumberFormat('da-DK', { style: 'currency', currency: 'DKK', maximumFractionDigits: 0 });

const KOMMUNE_NAMES = {
  1:'København',2:'Aarhus',3:'Odense',4:'Aalborg',5:'Esbjerg',
  6:'Randers',7:'Vejle',8:'Horsens',9:'Kolding',10:'Silkeborg',
  11:'Herning',12:'Roskilde',13:'Næstved',14:'Frederiksberg',15:'Viborg',
  16:'Holstebro',17:'Slagelse',18:'Svendborg',19:'Hjørring',20:'Frederikshavn',
  21:'Hillerød',22:'Helsingør',23:'Holbæk',24:'Ringsted',25:'Sorø',
  26:'Kalundborg',27:'Faxe',28:'Stevns',29:'Lejre',30:'Greve',
  31:'Solrød',32:'Køge',33:'Gribskov',34:'Halsnæs',35:'Frederikssund',
  36:'Egedal',37:'Allerød',38:'Fredensborg',39:'Hørsholm',40:'Rudersdal',
  41:'Lyngby-Taarbæk',42:'Gentofte',43:'Gladsaxe',44:'Herlev',45:'Albertslund',
  46:'Ballerup',47:'Furesø',48:'Glostrup',49:'Brøndby',50:'Ishøj',
  51:'Vallensbæk',52:'Hvidovre',53:'Rødovre',54:'Dragør',55:'Tårnby',
  56:'Høje-Taastrup',57:'Middelfart',58:'Assens',59:'Faaborg-Midtfyn',60:'Kerteminde',
  61:'Nyborg',62:'Nordfyns',63:'Langeland',64:'Ærø',65:'Haderslev',
  66:'Billund',67:'Sønderborg',68:'Tønder',69:'Aabenraa',70:'Fanø',
  71:'Varde',72:'Vejen',73:'Fredericia',74:'Horsens',75:'Skanderborg',
  76:'Odder',77:'Favrskov',78:'Syddjurs',79:'Norddjurs',80:'Lemvig',
  81:'Struer',82:'Skive',83:'Thisted',84:'Morsø',85:'Ikast-Brande',
  86:'Ringkøbing-Skjern',87:'Hedensted',88:'Vejle',89:'Bornholm',90:'Odsherred',
  91:'Lolland',92:'Guldborgsund',93:'Vordingborg',94:'Samsø',95:'Rebild',
  96:'Mariagerfjord',97:'Jammerbugt',98:'Brønderslev',
};

/**
 * Generate simplified SVG paths for 98 Danish kommuner.
 * Layout: approximate geographic positions on a 500×600 grid.
 * Each kommune is a small polygon/rect placed at its rough position.
 */
function generateKommunePaths() {
  const positions = computeKommunePositions();
  const paths = [];
  for (let id = 1; id <= 98; id++) {
    const [cx, cy] = positions[id] || [250, 300];
    const s = 18 + (id <= 20 ? 4 : 0);
    const d = `M${cx - s},${cy - s * 0.6} ` +
              `L${cx + s},${cy - s * 0.6} ` +
              `L${cx + s * 0.9},${cy + s * 0.6} ` +
              `L${cx - s * 0.9},${cy + s * 0.6} Z`;
    paths.push({ id, d });
  }
  return paths;
}

function computeKommunePositions() {
  const p = {};
  // Jylland Nord
  p[4]=  [220,40];  p[19]=[180,30]; p[20]=[260,25]; p[97]=[160,55]; p[98]=[220,60];
  p[95]=[260,70];   p[96]=[280,60]; p[83]=[130,80]; p[84]=[150,100]; p[15]=[230,95];
  // Jylland Midt
  p[80]=[100,115];  p[81]=[120,125]; p[82]=[160,115]; p[16]=[110,140];
  p[11]=[180,140];  p[86]=[90,160]; p[85]=[200,145]; p[10]=[240,135];
  p[6]=  [280,110]; p[77]=[260,125]; p[78]=[300,120]; p[79]=[310,100];
  // Jylland Syd + Midt-øst
  p[71]=[110,185];  p[66]=[160,185]; p[72]=[140,200]; p[5]=[80,200];
  p[70]=[60,210];   p[7]=[200,195]; p[87]=[220,185]; p[8]=[260,170];
  p[75]=[270,160];  p[76]=[290,155]; p[2]=[300,140];
  p[73]=[210,210];  p[9]=[190,220]; p[68]=[120,240]; p[69]=[160,235];
  p[65]=[200,240];  p[67]=[140,255];
  // Fyn
  p[57]=[185,260];  p[62]=[210,255]; p[60]=[240,255]; p[3]=[220,275];
  p[58]=[190,280];  p[59]=[210,290]; p[61]=[250,275]; p[18]=[220,305];
  p[63]=[240,320];  p[64]=[210,330]; p[94]=[280,245];
  // Sjælland Vest
  p[90]=[310,230];  p[34]=[320,215]; p[35]=[330,225]; p[26]=[300,250];
  p[23]=[340,245];  p[36]=[350,235]; p[29]=[320,265]; p[12]=[350,260];
  p[24]=[330,275];  p[25]=[310,275]; p[17]=[290,285]; p[33]=[360,215];
  // Sjælland Øst (Hovedstaden)
  p[21]=[370,230];  p[22]=[390,220]; p[38]=[380,235]; p[37]=[365,240];
  p[39]=[385,245];  p[40]=[380,250]; p[47]=[360,250]; p[46]=[355,255];
  p[41]=[385,258];  p[42]=[395,250]; p[43]=[370,260]; p[44]=[360,263];
  p[45]=[350,268];  p[48]=[365,270]; p[53]=[375,268]; p[14]=[390,262];
  p[1]=  [400,270]; p[52]=[380,278]; p[49]=[368,278]; p[50]=[358,280];
  p[51]=[363,283];  p[56]=[348,278]; p[55]=[393,285]; p[54]=[400,290];
  // Sjælland Syd
  p[30]=[355,285];  p[32]=[340,290]; p[31]=[348,292]; p[27]=[325,295];
  p[28]=[335,300];  p[13]=[305,305]; p[93]=[310,320]; p[92]=[290,330];
  p[91]=[275,340];  p[89]=[420,300];
  p[88]=[200,195]; // duplicate Vejle alias
  return p;
}

function deltaColor(value, maxAbs) {
  if (maxAbs < 0.001) return '#555';
  const t = Math.min(Math.abs(value) / maxAbs, 1);
  if (value > 0.001) {
    const r = Math.round(80 + 175 * t);
    const g = Math.round(60 - 20 * t);
    const b = Math.round(60 - 20 * t);
    return `rgb(${r},${g},${b})`;
  } else if (value < -0.001) {
    const r = Math.round(60 - 20 * t);
    const g = Math.round(80 + 150 * t);
    const b = Math.round(60 - 20 * t);
    return `rgb(${r},${g},${b})`;
  }
  return '#555';
}

/**
 * Factory: createKommuneKort(container)
 * Returns { update(geoData, perCapita), setFilter(id), clearFilter(), onFilterChange(cb) }
 */
export function createKommuneKort(container) {
  const svg = document.createElementNS(NS, 'svg');
  svg.setAttribute('viewBox', '0 0 460 380');
  svg.setAttribute('role', 'img');
  svg.setAttribute('aria-label', 'Kommunekort over Danmark');
  svg.style.width = '100%';
  svg.style.maxHeight = '500px';

  const tooltip = createTooltip();
  container.style.position = 'relative';
  container.appendChild(svg);
  container.appendChild(tooltip.element);

  const badge = document.createElement('div');
  badge.className = 'geo-filter-badge';
  badge.hidden = true;

  const badgeLabel = document.createElement('span');
  badge.appendChild(badgeLabel);

  const resetBtn = document.createElement('button');
  resetBtn.className = 'geo-filter-reset';
  resetBtn.setAttribute('aria-label', 'Nulstil kommune-filter');
  resetBtn.textContent = '\u2715';
  badge.appendChild(resetBtn);
  container.appendChild(badge);

  const toggle = document.createElement('button');
  toggle.className = 'per-capita-toggle';
  toggle.setAttribute('aria-pressed', 'false');
  toggle.setAttribute('role', 'switch');
  toggle.textContent = 'Per capita';
  container.appendChild(toggle);

  let pathEls = {};
  let currentData = null;
  let perCapitaMode = false;
  let activeFilter = null;
  let filterCallbacks = [];

  const kommunePaths = generateKommunePaths();
  kommunePaths.forEach(({ id, d }) => {
    const path = document.createElementNS(NS, 'path');
    path.setAttribute('d', d);
    path.setAttribute('data-kommune-id', id);
    path.setAttribute('fill', '#555');
    path.setAttribute('stroke', '#333');
    path.setAttribute('stroke-width', '0.5');
    path.style.cursor = 'pointer';
    path.style.transition = 'fill 0.15s ease';

    path.addEventListener('mouseenter', (e) => {
      if (!currentData) return;
      const k = currentData.kommuner.find(k => k.kommune_id === id);
      if (!k) return;
      const name = KOMMUNE_NAMES[id] || 'Kommune ' + id;
      const val = perCapitaMode ? k.per_capita_delta : k.total_delta;
      const label = perCapitaMode ? 'Per capita' : 'Total';
      const text = `${name}\n${label}: ${FMT_KR.format(val)}\nBerørte: ${FMT.format(k.affected_count)} af ${FMT.format(k.population)}`;
      const rect = path.getBoundingClientRect();
      const contRect = container.getBoundingClientRect();
      tooltip.show(rect.left - contRect.left + rect.width / 2, rect.top - contRect.top - 4, text);
    });

    path.addEventListener('mouseleave', () => {
      tooltip.hide();
    });

    path.addEventListener('click', () => {
      if (activeFilter === id) {
        clearFilterInternal();
      } else {
        setFilterInternal(id);
      }
    });

    svg.appendChild(path);
    pathEls[id] = path;
  });

  resetBtn.addEventListener('click', (e) => {
    e.stopPropagation();
    clearFilterInternal();
  });

  toggle.addEventListener('click', () => {
    perCapitaMode = !perCapitaMode;
    toggle.setAttribute('aria-pressed', perCapitaMode ? 'true' : 'false');
    if (perCapitaMode) {
      toggle.classList.add('active');
    } else {
      toggle.classList.remove('active');
    }
    if (currentData) applyColors();
  });

  function applyColors() {
    if (!currentData) return;
    const vals = currentData.kommuner.map(k =>
      perCapitaMode ? k.per_capita_delta : k.total_delta
    );
    const maxAbs = Math.max(...vals.map(Math.abs), 0.001);

    currentData.kommuner.forEach(k => {
      const p = pathEls[k.kommune_id];
      if (!p) return;
      const val = perCapitaMode ? k.per_capita_delta : k.total_delta;
      p.setAttribute('fill', deltaColor(val, maxAbs));
    });
  }

  function setFilterInternal(id) {
    if (activeFilter != null) {
      const prev = pathEls[activeFilter];
      if (prev) prev.classList.remove('active');
    }
    activeFilter = id;
    const el = pathEls[id];
    if (el) el.classList.add('active');

    const name = KOMMUNE_NAMES[id] || 'Kommune ' + id;
    badgeLabel.textContent = name;
    badge.hidden = false;

    filterCallbacks.forEach(cb => cb(id));
  }

  function clearFilterInternal() {
    if (activeFilter != null) {
      const prev = pathEls[activeFilter];
      if (prev) prev.classList.remove('active');
    }
    activeFilter = null;
    badge.hidden = true;
    filterCallbacks.forEach(cb => cb(null));
  }

  return {
    update(geoData, perCapita) {
      currentData = geoData;
      if (typeof perCapita === 'boolean') perCapitaMode = perCapita;
      applyColors();
    },
    setFilter(id) {
      setFilterInternal(id);
    },
    clearFilter() {
      clearFilterInternal();
    },
    onFilterChange(cb) {
      filterCallbacks.push(cb);
    },
  };
}
