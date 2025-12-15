
    export function check_d3_available() {
        return typeof d3 !== 'undefined';
    }

    export function create_svg(container_id, width, height) {
        const svg = d3.select('#' + container_id)
            .append('svg')
            .attr('width', width)
            .attr('height', height)
            .attr('id', 'main-svg');

        // Add gradient for sphere
        const defs = svg.append('defs');
        const gradient = defs.append('radialGradient')
            .attr('id', 'sphere-gradient')
            .attr('cx', '35%')
            .attr('cy', '35%')
            .attr('r', '60%');

        gradient.append('stop')
            .attr('offset', '0%')
            .attr('stop-color', '#6699ff');
        gradient.append('stop')
            .attr('offset', '70%')
            .attr('stop-color', '#3366cc');
        gradient.append('stop')
            .attr('offset', '100%')
            .attr('stop-color', '#1a3366');

        return svg.node();
    }

    export function update_svg_size(width, height) {
        d3.select('#main-svg')
            .attr('width', width)
            .attr('height', height);
    }

    export function create_sphere(svg, cx, cy, radius) {
        return d3.select(svg)
            .append('circle')
            .attr('cx', cx)
            .attr('cy', cy)
            .attr('r', radius)
            .attr('fill', 'url(#sphere-gradient)')
            .node();
    }

    export function update_sphere_position(sphere, cx, cy) {
        d3.select(sphere)
            .attr('cx', cx)
            .attr('cy', cy);
    }

    export function create_text_element(svg, x, y, char, fill, font_size) {
        return d3.select(svg)
            .append('text')
            .attr('x', x)
            .attr('y', y)
            .text(char)
            .attr('fill', fill)
            .attr('font-size', font_size + 'px')
            .attr('font-family', 'Arial, sans-serif')
            .attr('font-weight', 'bold')
            .attr('text-anchor', 'middle')
            .attr('dominant-baseline', 'central')
            .attr('opacity', 1)
            .node();
    }

    export function update_text_element(element, x, y, font_size, opacity) {
        d3.select(element)
            .attr('x', x)
            .attr('y', y)
            .attr('font-size', font_size + 'px')
            .attr('opacity', opacity);
    }

    export function bring_to_front(element) {
        const node = d3.select(element).node();
        if (node && node.parentNode) {
            node.parentNode.appendChild(node);
        }
    }

    export function send_to_back(element) {
        const node = d3.select(element).node();
        if (node && node.parentNode) {
            node.parentNode.insertBefore(node, node.parentNode.firstChild);
        }
    }

    export function reorder_elements(elements) {
        // elements is an array sorted back-to-front (lowest z first)
        elements.forEach(el => {
            const node = d3.select(el).node();
            if (node && node.parentNode) {
                node.parentNode.appendChild(node);
            }
        });
    }

    export function get_window_size() {
        return {
            width: window.innerWidth,
            height: window.innerHeight
        };
    }
