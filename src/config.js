"use strict";
/*
 * Compile time configuration, some only relevant for debug mode
 */

/**
 * @define {boolean}
 * Overridden for production by closure compiler
 */
var DEBUG = false;

/** @const */
var LOG_TO_FILE = false;

/**
 * @const
 * Enables logging all IO port reads and writes. Very verbose
 */
var LOG_ALL_IO = false;

/**
 * @const
 */
var DUMP_GENERATED_WASM = false;

/**
 * @const
 */
var DUMP_UNCOMPILED_ASSEMBLY = false;

/**
 * @const
 * More accurate filenames in 9p debug messages at the cost of performance.
 */
var TRACK_FILENAMES = false;

var LOG_LEVEL = LOG_ERROR | LOG_WARN;

/**
 * @const
 * Draws entire buffer and visualizes the layers that would be drawn
 */
var DEBUG_SCREEN_LAYERS = DEBUG && false;

/**
 * @const
 * How many ticks the TSC does per millisecond
 */
var TSC_RATE = 1 * 1000 * 1000;

/** @const */
var APIC_TIMER_FREQ = TSC_RATE;